use crate::{
    ScreenMode,
    app_state::AppState,
    config::Config,
    model::button::InputEvent,
    otel::TraceGraphSnapshot,
    prometheus::model::NodeMetrics,
    states::{Action, InspectOption},
    tui::{Event, Tui},
    update::{UPDATE_DEFS, UpdateList},
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::{Backend, Rect};
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use tokio::sync::mpsc::{Receiver, UnboundedReceiver, UnboundedSender, unbounded_channel};
use tracing::{debug, error, info, trace};

pub struct App {
    config: Config,
    app_state: AppState, // Model
    updates: UpdateList, // Update
    last_store_option: InspectOption,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

impl App {
    pub fn new(
        ledger_db: ReadOnlyRocksDB,
        chain_db: ReadOnlyChainDB,
        trace_graph: TraceGraphSnapshot,
        prom_metrics: Receiver<NodeMetrics>,
        button_events: mpsc::Receiver<InputEvent>,
        frame_area: Rect,
        screen_mode: ScreenMode,
    ) -> Result<Self> {
        let (action_tx, action_rx) = unbounded_channel();

        let app_state = AppState::new(
            ledger_db,
            chain_db,
            trace_graph,
            prom_metrics,
            button_events,
            frame_area,
            screen_mode,
        )?;
        let last_inspect_tabs = *app_state.get_inspect_tabs().cursor.current();

        Ok(Self {
            app_state,
            updates: UPDATE_DEFS.to_vec(),
            last_store_option: last_inspect_tabs,
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
        })
    }

    pub fn enter<B: Backend>(&mut self, tui: &mut Tui<B>) -> Result<()> {
        tui.terminal
            .clear()
            .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        tui.enter()?;
        Ok(())
    }

    pub async fn run<B: Backend>(&mut self, tui: &mut Tui<B>) -> Result<()> {
        self.enter(tui)?;
        loop {
            let should_continue = self.run_once(tui).await?;
            if !should_continue {
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    pub async fn run_once<B: Backend>(&mut self, tui: &mut Tui<B>) -> Result<bool> {
        self.handle_events(tui).await?;
        self.handle_actions(tui)?;
        if self.should_suspend {
            tui.suspend()?;
            self.action_tx.send(Action::Resume)?;
            self.action_tx.send(Action::ClearScreen)?;
            // tui.mouse(true);
            tui.enter()?;
        } else if self.should_quit {
            tui.stop()?;
            return Ok(false);
        }

        Ok(true)
    }

    async fn handle_events<B: Backend>(&mut self, tui: &mut Tui<B>) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => {
                action_tx.send(Action::Key(key.code))?;
                self.handle_key_event(key)?
            }
            Event::Mouse(mouse) => action_tx.send(Action::MouseEvent(mouse))?,
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            trace!("App::handle_key_event - no keymap: {:?}", key);
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Key to action: {action:?}. Will broadcast.");
                action_tx.send(action.clone())?;
            }
            _ => {
                trace!("App::handle_key_event - no single-key action: {:?}", key);
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }

    fn handle_actions<B: Backend>(&mut self, tui: &mut Tui<B>) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if !action.is_noisy() {
                debug!("{action:?}");
            }

            match action {
                Action::Tick => {
                    self.last_tick_key_events.clear();
                    for component in self.app_state.component_registry.values_mut() {
                        let actions = component.tick();
                        for a in actions {
                            self.action_tx.send(a)?;
                        }
                    }
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui
                    .clear()
                    .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                _ => {}
            }

            self.run_updates(&action)?;
        }

        Ok(())
    }

    fn run_updates(&mut self, action: &Action) -> Result<()> {
        let mut next_actions = Vec::new();
        for updater in &self.updates {
            next_actions.extend(updater.update(action, &mut self.app_state));
        }
        for next_action in next_actions {
            self.action_tx.send(next_action)?
        }
        Ok(())
    }

    fn handle_resize<B: Backend>(&mut self, tui: &mut Tui<B>, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))
            .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        self.render(tui)
    }

    fn render<B: Backend>(&mut self, tui: &mut Tui<B>) -> Result<()> {
        tui.draw(|f| {
            let frame_area = f.area();
            if frame_area != self.app_state.frame_area
                || self.app_state.get_inspect_tabs().selected() != self.last_store_option
            {
                debug!("Frame area or store option changed");

                let action = Action::UpdateLayout(frame_area);
                let _ = self.run_updates(&action);
                self.last_store_option = self.app_state.get_inspect_tabs().selected();
            }

            let layout = self.app_state.layout_model.get_layout();
            for (component_id, _) in layout.iter() {
                if let Some(component) = self.app_state.component_registry.get(component_id) {
                    component.render(f, &self.app_state, layout);
                } else {
                    error!(
                        "Render loop: Component {:?} in layout but not in registry",
                        component_id
                    );
                }
            }
        })
        .map(|_| ())
        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))
    }
}

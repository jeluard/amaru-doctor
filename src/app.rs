use crate::{
    ScreenMode,
    app_state::AppState,
    config::Config,
    effects::logo::LogoScreen,
    model::button::InputEvent,
    otel::TraceGraphSnapshot,
    prometheus::model::NodeMetrics,
    states::{Action, InspectOption},
    tui::{Event, Tui},
    update::{UPDATE_DEFS, UpdateList},
    view::{SlotViews, compute_slot_views},
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::{Backend, Rect};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, UnboundedReceiver, UnboundedSender, unbounded_channel};
use tracing::{debug, info, trace};

pub struct App {
    config: Config,
    app_state: AppState, // Model
    updates: UpdateList, // Update
    last_store_option: InspectOption,
    slot_views: SlotViews, // View
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
    logo_screen: LogoScreen,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    Splash,
    Home,
}

impl App {
    pub fn new(
        ledger_db: ReadOnlyRocksDB,
        chain_db: ReadOnlyChainDB,
        trace_graph: TraceGraphSnapshot,
        prom_metrics: Receiver<NodeMetrics>,
        button_events: std::sync::mpsc::Receiver<InputEvent>,
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
        let last_inspect_tabs = *app_state.inspect_tabs.cursor.current();
        let slot_views = compute_slot_views(&app_state);

        Ok(Self {
            app_state,
            updates: UPDATE_DEFS.to_vec(),
            last_store_option: last_inspect_tabs,
            slot_views,
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Splash,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
            logo_screen: LogoScreen::new(Duration::from_secs(1)),
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

            let recompute_slot_widgets = matches!(
                action,
                Action::ScrollUp | Action::ScrollDown | Action::MouseEvent(_)
            );

            match action {
                Action::Tick => {
                    self.last_tick_key_events.clear();
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui
                    .clear()
                    .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => match self.mode {
                    Mode::Splash => {
                        tui.draw(|f| self.logo_screen.display(f))?;
                        if self.logo_screen.is_done() {
                            self.mode = Mode::Home;
                        }
                    }
                    Mode::Home => self.render(tui)?,
                },
                _ => {}
            }

            self.run_updates(&action)?;

            if recompute_slot_widgets {
                self.slot_views = compute_slot_views(&self.app_state);
            }
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
                || self.app_state.inspect_tabs.cursor.current() != &self.last_store_option
            {
                debug!("Frame area or store option changed");

                let action = Action::UpdateLayout(frame_area);
                let _ = self.run_updates(&action);

                self.last_store_option = *self.app_state.inspect_tabs.cursor.current();
            }

            for (slot, area) in self.app_state.layout_model.get_layout().clone().iter() {
                if let Some(view) = self.slot_views.get(slot) {
                    view.render(f, *area, &self.app_state);
                } else {
                    debug!("Found no view for slot {}", slot);
                }
            }
        })
        .map(|_| ())
        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))
    }
}

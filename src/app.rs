use crate::{
    ScreenMode,
    app_state::AppState,
    components::{InputRoute, root::RootComponent},
    config::Config,
    model::button::InputEvent,
    otel::TraceGraphSnapshot,
    prometheus::model::NodeMetrics,
    states::{Action, ComponentId, InspectOption},
    tui::{Event, Tui},
    update::{UPDATE_DEFS, UpdateList},
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::{Backend, Rect};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::mpsc};
use tokio::sync::mpsc::{Receiver, UnboundedReceiver, UnboundedSender, unbounded_channel};
use tracing::{debug, error};

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

        Ok(Self {
            app_state,
            updates: UPDATE_DEFS.to_vec(),
            last_store_option: InspectOption::default(),
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::default(),
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
                // Let the focused component try to handle it
                let handled = self.dispatch_event(Event::Key(key))?;

                // Global Fallback: If not handled, check Config
                if !handled {
                    self.handle_config_key(key)?;
                }
            }

            Event::Mouse(mouse) => {
                self.dispatch_event(Event::Mouse(mouse))?;
            }
            _ => {}
        }
        Ok(())
    }

    /// From Root to find the target ComponentId.
    /// Borrow that specific component mutably to execute logic.
    fn dispatch_event(&mut self, event: Event) -> Result<bool> {
        let mut target_id = ComponentId::Root;
        let mut target_area = self.app_state.frame_area;

        // Prevent infinite recursion
        let mut depth = 0;
        const MAX_DEPTH: usize = 50;

        loop {
            if depth > MAX_DEPTH {
                error!("Dispatch Loop Detected. Aborting route at {:?}", target_id);
                return Ok(false);
            }
            depth += 1;

            let Some(comp) = self.app_state.component_registry.get(&target_id) else {
                debug!("Dispatch Broken: {} not found", target_id);
                return Ok(false);
            };

            let route = comp.route_event(&event, &self.app_state);
            match route {
                InputRoute::Delegate(child_id, child_rect) => {
                    // If dividing to self, break immediately
                    if child_id == target_id {
                        error!(
                            "Infinite Loop: Component {} tried to delegate to itself",
                            target_id
                        );
                        return Ok(false);
                    }
                    target_id = child_id;
                    target_area = child_rect;
                }
                InputRoute::Handle => {
                    break;
                }
                InputRoute::Ignore => {
                    return Ok(false);
                }
            }
        }

        // Handling
        // Now we have the ID. We can safely take a mutable borrow on this single component.
        if let Some(comp) = self.app_state.component_registry.get_mut(&target_id) {
            if let Event::Mouse(_) = event
                && self.app_state.layout_model.get_focus() != target_id
            {
                self.app_state.layout_model.set_focus(target_id);
            }

            let mut actions = comp.handle_event(&event, target_area);

            // Trigger Layout Update for Options Lists
            // When selection changes in these lists, the whole page layout changes.
            if matches!(
                target_id,
                ComponentId::LedgerBrowseOptions | ComponentId::LedgerSearchOptions
            ) {
                // Only trigger if it was an input type that changes selection (Key/Mouse)
                actions.push(Action::UpdateLayout(self.app_state.frame_area));
            }

            if !actions.is_empty() {
                for action in actions {
                    self.action_tx.send(action)?;
                }
                return Ok(true);
            }
        }

        Ok(false) // Not Consumed
    }

    /// Checks the configuration for a matching keybinding and dispatches the action.
    fn handle_config_key(&mut self, key: KeyEvent) -> Result<()> {
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            return Ok(());
        };

        // Single Key match
        if let Some(action) = keymap.get(&vec![key]) {
            self.action_tx.send(action.clone())?;
            return Ok(());
        }

        self.last_tick_key_events.push(key);
        if let Some(action) = keymap.get(&self.last_tick_key_events) {
            self.action_tx.send(action.clone())?;
            self.last_tick_key_events.clear();
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
            let current_selection = self
                .app_state
                .component_registry
                .get(&ComponentId::Root)
                .and_then(|c| c.as_any().downcast_ref::<RootComponent>())
                .map(|root| root.tabs.selected())
                .unwrap_or_default();
            if frame_area != self.app_state.frame_area
                || current_selection != self.last_store_option
            {
                // TODO: Remove this, we shouldn't be updating the layout here
                debug!("Frame area or store option changed");
                let action = Action::UpdateLayout(frame_area);
                let _ = self.run_updates(&action);
                self.last_store_option = current_selection;
            }

            if let Some(root) = self.app_state.component_registry.get(&ComponentId::Root) {
                root.render(f, &self.app_state, &HashMap::new());
            }
        })
        .map(|_| ())
        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))
    }
}

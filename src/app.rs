use crate::{
    app_state::AppState,
    config::Config,
    states::{Action, StoreOption},
    tui::{Event, Tui},
    update::{UpdateList, get_updates},
    view::{SlotViews, compute_slot_views},
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use std::io::Error;
use tokio::sync::mpsc;
use tracing::{debug, info, trace};

pub struct App {
    config: Config,
    app_state: AppState, // Model
    updates: UpdateList, // Update
    last_store_option: StoreOption,
    slot_views: SlotViews, // View
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
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
        frame_area: Rect,
    ) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        let app_state = AppState::new(ledger_db, chain_db)?;
        action_tx.send(Action::UpdateLayout(frame_area))?;
        let last_store_option = app_state.store_option.current().clone();
        let slot_views = compute_slot_views(&app_state);

        Ok(Self {
            app_state,
            updates: get_updates(),
            last_store_option,
            slot_views,
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
        })
    }

    pub async fn run(&mut self, tui: &mut Tui) -> Result<()> {
        tui.terminal.clear()?;
        tui.enter()?;

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(tui).await?;
            self.handle_actions(tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
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
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        trace!("App::handle_key_event - received: {:?}", key);
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

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if !matches!(action, Action::Tick | Action::Render) {
                debug!("{action:?}");
            }

            let recompute_slot_widgets = matches!(action, Action::ScrollUp | Action::ScrollDown);

            match action {
                Action::Tick => {
                    self.last_tick_key_events.clear();
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
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

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.try_draw(|f| -> std::result::Result<(), _> {
            let frame_area = f.area();
            if frame_area != self.app_state.frame_area
                || self.app_state.store_option.current() != &self.last_store_option
            {
                trace!("Frame area or store option changed");

                // Synchronously update the layout
                let action = Action::UpdateLayout(frame_area);
                self.run_updates(&action).map_err(Error::other)?;

                self.last_store_option = self.app_state.store_option.current().clone();
            }
            for (slot, area) in self.app_state.layout.iter() {
                if let Some(view) = self.slot_views.get(slot) {
                    if let Err(e) = view.render(f, *area, &self.app_state) {
                        let _ = self
                            .action_tx
                            .send(Action::Error(format!("Failed to draw: {e:?}")));
                    }
                } else {
                    trace!("Found no view for slot {}", slot);
                }
            }
            Ok::<(), std::io::Error>(())
        })
        .map(|_| ())
        .map_err(Into::into)
    }
}

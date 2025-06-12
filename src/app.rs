use crate::{
    app_state::AppState,
    config::Config,
    controller::get_selected_widget,
    layout::{SlotLayout, compute_slot_layout, compute_slot_map},
    states::{Action, WidgetId, WidgetSlot},
    store::rocks_db_switch::RocksDBSwitch,
    tui::{Event, Tui},
    update::{UpdateList, get_updates},
    view::{ViewMap, get_views},
};
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc;
use tracing::{debug, info, trace};

pub struct App {
    ledger_path_str: String,
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    app_state: AppState, // Model
    views: ViewMap,      // View
    updates: UpdateList, // Update
    layout: Option<SlotLayout>,
    slot_map: HashMap<WidgetSlot, WidgetId>,
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
        ledger_path_str: &String,
        tick_rate: f64,
        frame_rate: f64,
        db: Arc<RocksDBSwitch>,
    ) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let app_state = AppState::new(db);
        let slot_map = compute_slot_map(&app_state);
        Ok(Self {
            ledger_path_str: ledger_path_str.to_owned(),
            tick_rate,
            frame_rate,
            app_state,
            views: get_views(),
            updates: get_updates(),
            layout: None,
            slot_map,
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            // .mouse(true) // uncomment this line to enable mouse support
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.terminal.clear()?;
        tui.enter()?;

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
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
            Event::Key(key) => self.handle_key_event(key)?,
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

            let recompute_slot_map = matches!(action, Action::ScrollUp | Action::ScrollDown);

            match action {
                Action::Tick => {
                    self.last_tick_key_events.clear();
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                _ => {}
            }

            for updater in &self.updates {
                updater.update(&action, &mut self.app_state);
            }

            if recompute_slot_map {
                self.slot_map = compute_slot_map(&self.app_state);
            }
        }

        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        let views = &self.views;
        let app_state = &self.app_state;
        let action_tx = self.action_tx.clone();

        tui.draw(|f| {
            let layout = self
                .layout
                .get_or_insert_with(|| compute_slot_layout(f.area()));

            for (slot, area) in layout {
                let Some(widget_id) = self.slot_map.get(slot) else {
                    continue;
                };
                let Some(view) = views.get(widget_id) else {
                    continue;
                };
                if let Err(e) = view.render(f, *area, app_state) {
                    let _ = action_tx.send(Action::Error(format!("Failed to draw: {:?}", e)));
                }
            }
        })?;

        Ok(())
    }
}

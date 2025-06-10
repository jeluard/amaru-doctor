use crate::{
    app_state::AppState,
    build::{self},
    components::Component,
    config::Config,
    shared::{Shared, shared},
    states::Action,
    store::rocks_db_switch::RocksDBSwitch,
    tui::{Event, Tui},
};
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
};
use serde::{Deserialize, Serialize};
use std::{rc::Rc, sync::Arc};
use tokio::sync::mpsc;
use tracing::{debug, info, trace};

pub struct AppComponents {
    all: Vec<Shared<dyn Component>>,
    pub layout_rev: usize,
}

impl AppComponents {
    pub fn new(all: Vec<Shared<dyn Component>>) -> Self {
        Self { all, layout_rev: 0 }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Shared<dyn Component>> {
        self.all.iter()
    }
}

pub struct App {
    ledger_path_str: String,
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    app_state: Shared<AppState>,
    components: AppComponents,
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
        let app_state = shared(AppState::new(db));
        let components = build::build_layout(ledger_path_str, app_state.clone());
        Ok(Self {
            ledger_path_str: ledger_path_str.to_owned(),
            tick_rate,
            frame_rate,
            app_state,
            components,
            // focus,
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

        self.components.iter().try_for_each(|c| {
            c.borrow_mut()
                .register_action_handler(self.action_tx.clone())
        })?;
        self.components
            .iter()
            .try_for_each(|c| c.borrow_mut().register_config_handler(self.config.clone()))?;
        self.components
            .iter()
            .try_for_each(|c| c.borrow_mut().init(tui.size()?))?;

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
        for component in self.components.iter() {
            for action in component.borrow_mut().handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        trace!("App::handle_key_event - received: {:?}", key);
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Key to action: {action:?}. Will broadcast.");
                action_tx.send(action.clone())?;
            }
            _ => {
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
            if action != Action::Tick && action != Action::Render {
                debug!("{action:?}");
            }
            match action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                Action::FocusPrev => self.app_state.borrow_mut().shift_focus_prev(),
                Action::FocusNext => self.app_state.borrow_mut().shift_focus_next(),
                _ => {}
            }
            for component in self.components.iter() {
                for action in component.borrow_mut().update(action.clone())? {
                    self.action_tx.send(action)?;
                }
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
        self.compute_layout();
        tui.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(area);
            let header = self.components.all[0].borrow_mut().draw(frame, chunks[0]);
            let body = self.components.all[1].borrow_mut().draw(frame, chunks[1]);
            let footer = self.components.all[2].borrow_mut().draw(frame, chunks[2]);

            for result in [header, body, footer] {
                if let Err(err) = result {
                    let _ = self
                        .action_tx
                        .send(Action::Error(format!("Failed to draw: {:?}", err)));
                }
            }
        })?;
        Ok(())
    }

    /// Conditionally recomputes the layout given changes to the app's state
    /// This is temporary while we centralize the state
    fn compute_layout(&mut self) {
        if self.components.layout_rev < self.app_state.borrow().layout_rev {
            self.components = build::build_layout(&self.ledger_path_str, self.app_state.clone());
        }
    }
}

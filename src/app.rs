use crate::{
    components::{Component, root::RootComponent},
    config::Config,
    model::button::InputEvent,
    otel::TraceGraphSnapshot,
    states::Action,
    tui::{Event, Tui},
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use ratatui::prelude::{Backend, Rect};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, mpsc};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tracing::debug;

pub struct App {
    config: Config,
    button_events: mpsc::Receiver<InputEvent>,
    frame_area: Rect,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: UnboundedSender<Action>,
    action_rx: UnboundedReceiver<Action>,
    root: RootComponent,
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
        button_events: mpsc::Receiver<InputEvent>,
        frame_area: Rect,
    ) -> Result<Self> {
        let (action_tx, action_rx) = unbounded_channel();

        Ok(Self {
            button_events,
            frame_area,
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::default(),
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
            root: RootComponent::new(Arc::new(ledger_db), Arc::new(chain_db), trace_graph),
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
            Event::Input(CrosstermEvent::Resize(w, h)) => {
                self.action_tx.send(Action::Resize(w, h))?
            }
            Event::Input(crossterm_event) => {
                let handled = self.dispatch_event(crossterm_event.clone()).await?;
                // Fallback for global keys if component didn't handle it
                if !handled && let CrosstermEvent::Key(key) = crossterm_event {
                    self.handle_config_key(key)?;
                }
            }
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Quit => action_tx.send(Action::Quit)?,
            _ => {}
        }
        Ok(())
    }

    async fn dispatch_event(&mut self, event: CrosstermEvent) -> Result<bool> {
        let actions = self.root.handle_event(&event, self.frame_area);
        let handled = !actions.is_empty();

        for action in actions {
            self.action_tx.send(action)?;
        }

        Ok(handled)
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

            let root_actions = self.root.handle_action(action.clone());
            for a in root_actions {
                self.action_tx.send(a)?;
            }

            match action {
                Action::Tick => {
                    self.last_tick_key_events.clear();
                    let actions = self.root.tick();
                    for a in actions {
                        self.action_tx.send(a)?;
                    }
                    for input_event in self.button_events.try_iter() {
                        let action: Action = input_event.into();
                        self.action_tx.send(action)?;
                    }
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui
                    .clear()
                    .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => {
                    self.render(tui)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn handle_resize<B: Backend>(&mut self, tui: &mut Tui<B>, w: u16, h: u16) -> Result<()> {
        let new_area = Rect::new(0, 0, w, h);
        tui.resize(new_area)
            .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        self.frame_area = new_area;
        self.render(tui)
    }

    fn render<B: Backend>(&mut self, tui: &mut Tui<B>) -> Result<()> {
        tui.draw(|f| self.root.render(f, f.area()))
            .map(|_| ())
            .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))
    }
}

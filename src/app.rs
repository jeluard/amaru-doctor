use std::sync::Arc;

use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, info, trace};

use crate::{
    action::Action,
    components::{
        Component,
        empty::EmptyComponent,
        fps::FpsCounter,
        group::ComponentGroup,
        layout::AppLayout,
        message::Message,
        resources::ResourceList,
        split::{Axis, SplitComponent},
        utxos::UtxoList,
    },
    config::Config,
    tui::{Event, Tui},
};

pub struct App {
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    components: AppLayout,
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
        db: Arc<RocksDB>,
    ) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Ok(Self {
            tick_rate,
            frame_rate,
            components: Self::init_layout(ledger_path_str, db),
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
        })
    }

    fn init_layout(ledger_path_str: &String, db: Arc<RocksDB>) -> AppLayout {
        AppLayout::new(
            Box::new(ComponentGroup::new(vec![
                Box::new(Message::new(format!(
                    "Reading amaru ledger at {:?}",
                    ledger_path_str
                ))),
                Box::new(FpsCounter::default()),
            ])),
            Box::new(SplitComponent::new_2(
                Axis::Vertical,
                30,
                Box::new(SplitComponent::new_2_evenly(
                    Axis::Horizontal,
                    Box::new(ResourceList::default()),
                    Box::new(UtxoList::new(db)),
                )),
                70,
                Box::new(EmptyComponent::default()),
            )),
            Box::new(ComponentGroup::new(vec![Box::new(Message::new(
                "Use arrow keys ←↑→↓ to navigate.".to_string(),
            ))])),
        )
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            // .mouse(true) // uncomment this line to enable mouse support
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.terminal.clear()?;
        tui.enter()?;

        self.components
            .register_action_handler(self.action_tx.clone())?;
        self.components
            .register_config_handler(self.config.clone())?;
        self.components.init(tui.size()?)?;

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
        for action in self.components.handle_events(Some(event.clone()))? {
            action_tx.send(action)?;
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
                info!("Got action: {action:?}");
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
                _ => {}
            }
            for action in self.components.update(action.clone())? {
                self.action_tx.send(action)?;
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
        tui.draw(|frame| {
            if let Err(err) = self.components.draw(frame, frame.area()) {
                let _ = self
                    .action_tx
                    .send(Action::Error(format!("Failed to draw: {:?}", err)));
            }
        })?;
        Ok(())
    }
}

#![allow(dead_code)] // Remove this once you start using the code

use std::{
    io::stdout,
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{
        DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
        Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent,
    },
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::{
    Terminal,
    prelude::{Backend, CrosstermBackend},
};
use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time::interval,
};
use tokio_util::sync::CancellationToken;
use tracing::error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Init,
    Quit,
    Error,
    Closed,
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct Tui<B: Backend> {
    pub terminal: Terminal<B>,
    pub task: tokio::task::JoinHandle<()>,
    pub cancellation_token: CancellationToken,
    pub event_rx: UnboundedReceiver<Event>,
    pub event_tx: UnboundedSender<Event>,
    pub mouse: bool,
    pub paste: bool,
}

async fn event_loop(event_tx: UnboundedSender<Event>, cancellation_token: CancellationToken) {
    let mut event_stream = EventStream::new();
    let mut tick_interval = interval(Duration::from_secs_f64(1.0 / 4.0));
    let mut render_interval = interval(Duration::from_secs_f64(1.0 / 60.0));

    // if this fails, then it's likely a bug in the calling code
    event_tx
        .send(Event::Init)
        .expect("failed to send init event");
    loop {
        let event = tokio::select! {
            _ = cancellation_token.cancelled() => {
                break;
            }
            _ = tick_interval.tick() => Event::Tick,
            _ = render_interval.tick() => Event::Render,
            crossterm_event = event_stream.next().fuse() => match crossterm_event {
                Some(Ok(event)) => match event {
                    CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => Event::Key(key),
                    CrosstermEvent::Mouse(mouse) => Event::Mouse(mouse),
                    CrosstermEvent::Resize(x, y) => Event::Resize(x, y),
                    CrosstermEvent::FocusLost => Event::FocusLost,
                    CrosstermEvent::FocusGained => Event::FocusGained,
                    CrosstermEvent::Paste(s) => Event::Paste(s),
                    _ => continue, // ignore other events
                }
                Some(Err(_)) => Event::Error,
                None => break, // the event stream has stopped and will not produce any more events
            },
        };
        if event_tx.send(event).is_err() {
            // the receiver has been dropped, so there's no point in continuing the loop
            break;
        }
    }
    cancellation_token.cancel();
}

impl Default for Tui<CrosstermBackend<std::io::Stdout>> {
    fn default() -> Self {
        Self::new(CrosstermBackend::new(stdout())).unwrap()
    }
}

impl<B: Backend> Tui<B> {
    pub fn new(backend: B) -> Result<Self>{
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Ok(Self {
            terminal: Terminal::new(backend).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?,
            task: tokio::spawn(async {}),
            cancellation_token: CancellationToken::new(),
            event_rx,
            event_tx,
            mouse: false,
            paste: false,
        })
    }

    pub fn mouse(mut self, mouse: bool) -> Self {
        self.mouse = mouse;
        self
    }

    pub fn paste(mut self, paste: bool) -> Self {
        self.paste = paste;
        self
    }

    pub fn start(&mut self) {
        self.cancel(); // Cancel any existing task
        self.cancellation_token = CancellationToken::new();
        let event_tx = self.event_tx.clone();
        let cancellation_token = self.cancellation_token.clone();
        let event_loop = event_loop(event_tx, cancellation_token);
        self.task = tokio::spawn(async {
            event_loop.await;
        });
    }

    pub fn stop(&self) -> Result<()> {
        self.cancel();
        let mut counter = 0;
        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));
            counter += 1;
            if counter > 50 {
                self.task.abort();
            }
            if counter > 100 {
                error!("Failed to abort task in 100 milliseconds for unknown reason");
                break;
            }
        }
        Ok(())
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
        if self.mouse {
            crossterm::execute!(stdout(), EnableMouseCapture)?;
        }
        if self.paste {
            crossterm::execute!(stdout(), EnableBracketedPaste)?;
        }
        self.start();
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.stop()?;
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.flush().map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
            if self.paste {
                crossterm::execute!(stdout(), DisableBracketedPaste)?;
            }
            if self.mouse {
                crossterm::execute!(stdout(), DisableMouseCapture)?;
            }
            crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    pub fn suspend(&mut self) -> Result<()> {
        self.exit()?;
        #[cfg(not(windows))]
        signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        self.enter()?;
        Ok(())
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }
}

impl<B: Backend> Deref for Tui<B> {
    type Target = ratatui::Terminal<B>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl<B: Backend> DerefMut for Tui<B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl<B: Backend> Drop for Tui<B> {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}

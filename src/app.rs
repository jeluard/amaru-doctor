use crate::{
    app_state::AppState,
    config::Config,
    mutator::Mutator,
    render::compute_slot_layout,
    shared::{Shared, shared},
    states::Action,
    store::rocks_db_switch::RocksDBSwitch,
    tui::{Event, Tui},
    view::{ViewMap, get_views},
};
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info, trace};

pub struct App {
    ledger_path_str: String,
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    app_state: Shared<AppState>,
    views: ViewMap,
    // widget_map: HashMap<WidgetId, SharedComp>,
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
        // let widget_map = build_widget_map(app_state.clone());
        Ok(Self {
            ledger_path_str: ledger_path_str.to_owned(),
            tick_rate,
            frame_rate,
            app_state: app_state.clone(),
            // widget_map,
            views: get_views(app_state.clone()),
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

        // self.widget_map.values().try_for_each(|c| {
        //     c.borrow_mut()
        //         .register_action_handler(self.action_tx.clone())
        // })?;
        // self.widget_map
        //     .values()
        //     .try_for_each(|c| c.borrow_mut().register_config_handler(self.config.clone()))?;
        // self.widget_map
        //     .values()
        //     .try_for_each(|c| c.borrow_mut().init(tui.size()?))?;

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
        // for component in self.widget_map.values() {
        //     for action in component.borrow_mut().handle_events(Some(event.clone()))? {
        //         action_tx.send(action)?;
        //     }
        // }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        trace!("App::handle_key_event - received: {:?}", key);
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            trace!("App::handle_key_event - no keymap: {:?}", key);
            return Ok(());
        };
        trace!("App::handle_key_event - keymap: {:?}", keymap);
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
            // TODO: Move all actions (the above) in mutate
            action.mutate(self.app_state.clone());
            // for component in self.widget_map.values() {
            //     for action in component.borrow_mut().update(action_clone.clone())? {
            //         self.action_tx.send(action)?;
            //     }
            // }
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    // fn render(&mut self, tui: &mut Tui) -> Result<()> {
    //     tui.draw(|frame| {
    //         let area = frame.area();
    //         let chunks = Layout::default()
    //             .direction(Direction::Vertical)
    //             .constraints(vec![
    //                 Constraint::Length(1),
    //                 Constraint::Min(1),
    //                 Constraint::Length(1),
    //             ])
    //             .split(area);

    //         let (nav, options, list, details) =
    //             build::resolve_layout_widgets(self.app_state.clone());
    //         let (header, body, footer) = (
    //             build::make_header(&self.ledger_path_str),
    //             make_body(nav, options, list, details, &self.widget_map),
    //             make_footer(),
    //         );

    //         let (header_res, body_res, footer_res) = (
    //             header.borrow_mut().draw(frame, chunks[0]),
    //             body.borrow_mut().draw(frame, chunks[1]),
    //             footer.borrow_mut().draw(frame, chunks[2]),
    //         );

    //         for result in [header_res, body_res, footer_res] {
    //             if let Err(err) = result {
    //                 let _ = self
    //                     .action_tx
    //                     .send(Action::Error(format!("Failed to draw: {:?}", err)));
    //             }
    //         }
    //     })?;
    //     Ok(())
    // }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        let app_state = self.app_state.clone();
        let views = &self.views;
        let action_tx = self.action_tx.clone();

        tui.draw(|f| {
            for (slot, area) in compute_slot_layout(f.area()) {
                let Some(widget_id) = app_state.borrow().get_selected_widget(slot) else {
                    continue;
                };
                let Some(view) = views.get(&widget_id) else {
                    continue;
                };
                if let Err(e) = view.render(f, area, app_state.clone()) {
                    let _ = action_tx.send(Action::Error(format!("Failed to draw: {:?}", e)));
                }
            }
        })?;
        Ok(())
    }
}

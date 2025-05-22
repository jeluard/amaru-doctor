use crate::action::Action;
use crate::components::Component;
use crate::config::Config;
use crate::tui::Event;

use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use delegate::delegate;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Size},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::trace;

use super::group::ComponentGroup;

pub struct AppLayout<'a> {
    group: ComponentGroup<'a>, // holds header, body, footer
}

impl<'a> AppLayout<'a> {
    pub fn new(
        header: Box<dyn Component + 'a>,
        body: Box<dyn Component + 'a>,
        footer: Box<dyn Component + 'a>,
    ) -> Self {
        Self {
            group: ComponentGroup::new(vec![header, body, footer]),
        }
    }
}

impl<'a> Component for AppLayout<'a> {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [header_area, body_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(area);

        let components = &mut self.group.components_mut();
        components[0].draw(frame, header_area)?;
        components[1].draw(frame, body_area)?;
        components[2].draw(frame, footer_area)?;
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        trace!("AppLayout::handle_key_event - {:?}", key);
        let components = &mut self.group.components_mut();

        let mut results = Vec::new();
        results.extend(components[0].handle_key_event(key)?);
        results.extend(components[1].handle_key_event(key)?);
        results.extend(components[2].handle_key_event(key)?);
        Ok(results)
    }

    delegate! {
        to self.group {
            fn update(&mut self, action: Action) -> Result<Vec<Action>>;
            fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()>;
            fn register_config_handler(&mut self, config: Config) -> Result<()>;
            fn init(&mut self, area: Size) -> Result<()>;
            fn handle_events(&mut self, event: Option<Event>) -> Result<Vec<Action>>;
            fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<Vec<Action>>;
        }
    }
}

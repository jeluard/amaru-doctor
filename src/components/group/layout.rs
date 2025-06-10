use crate::{
    states::Action,
    components::{Component, Shared},
    config::Config,
    tui::Event,
};
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Size,
};
use tokio::sync::mpsc::UnboundedSender;

pub struct LayoutComponent {
    direction: Direction,
    children: Vec<(Constraint, Shared<dyn Component>)>,
}

impl LayoutComponent {
    pub fn new(direction: Direction, children: Vec<(Constraint, Shared<dyn Component>)>) -> Self {
        Self {
            direction,
            children,
        }
    }
}
impl Component for LayoutComponent {
    fn debug_name(&self) -> String {
        "LayoutComponent".to_string()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let (constraints, widgets): (Vec<_>, Vec<_>) = self.children.iter().cloned().unzip();
        let chunks = Layout::default()
            .direction(self.direction)
            .constraints(constraints)
            .split(area);

        for (child, rect) in widgets.iter().zip(chunks.iter()) {
            child.borrow_mut().draw(frame, *rect)?;
        }

        Ok(())
    }

    fn init(&mut self, area: Size) -> Result<()> {
        for (_, c) in &self.children {
            c.borrow_mut().init(area)?;
        }
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        for (_, c) in &self.children {
            c.borrow_mut().register_action_handler(tx.clone())?;
        }
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        for (_, c) in &self.children {
            c.borrow_mut().register_config_handler(config.clone())?;
        }
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Result<Vec<Action>> {
        let mut actions = vec![];
        for (_, c) in &self.children {
            actions.extend(c.borrow_mut().handle_events(event.clone())?);
        }
        Ok(actions)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        let mut actions = vec![];
        for (_, c) in &self.children {
            actions.extend(c.borrow_mut().handle_key_event(key)?);
        }
        Ok(actions)
    }

    fn update(&mut self, action: Action) -> Result<Vec<Action>> {
        let mut actions = vec![];
        for (_, c) in &self.children {
            actions.extend(c.borrow_mut().update(action.clone())?);
        }
        Ok(actions)
    }
}

use crate::components::Component;
use crate::shared::Shared;
use crate::{action::Action, config::Config, tui::Event};
use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    Frame,
    layout::{Rect, Size},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::trace;

pub mod scroll;
pub mod split;
pub mod switch;

pub struct ComponentGroup<'a> {
    components: Vec<Shared<'a, dyn Component + 'a>>,
}

impl<'a> ComponentGroup<'a> {
    pub fn new(components: Vec<Shared<'a, dyn Component + 'a>>) -> Self {
        Self { components }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Shared<dyn Component + 'a>> {
        self.components.iter_mut()
    }
}

impl<'a> Component for ComponentGroup<'a> {
    fn debug_name(&self) -> String {
        "ComponentGroup".to_string()
    }

    fn update(&mut self, action: Action) -> Result<Vec<Action>> {
        if !matches!(action, Action::Render | Action::Tick) {
            trace!("ComponentGroup::update(key={:?})", action);
        }
        let mut results = Vec::new();
        for component in self.components.iter_mut() {
            let actions = component.borrow_mut().update(action.clone())?;
            results.extend(actions);
        }
        Ok(results)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        for component in self.components.iter_mut() {
            component.borrow_mut().draw(frame, area)?;
        }
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Result<Vec<Action>> {
        let mut results = Vec::new();
        for component in self.components.iter_mut() {
            let actions = component.borrow_mut().handle_events(event.clone())?;
            results.extend(actions);
        }
        Ok(results)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        trace!("ComponentGroup::handle_key_event(key={:?})", key);
        let mut results = Vec::new();
        for (i, component) in self.components.iter_mut().enumerate() {
            trace!("Forwarding to child component [{}]", i);
            results.extend(component.borrow_mut().handle_key_event(key)?);
        }
        Ok(results)
    }

    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<Vec<Action>> {
        let mut results = Vec::new();
        for component in self.components.iter_mut() {
            let actions = component.borrow_mut().handle_mouse_event(mouse)?;
            results.extend(actions);
        }
        Ok(results)
    }

    fn init(&mut self, area: Size) -> Result<()> {
        for component in self.components.iter_mut() {
            component.borrow_mut().init(area)?;
        }
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        for component in self.components.iter_mut() {
            component.borrow_mut().register_action_handler(tx.clone())?;
        }
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        for component in self.components.iter_mut() {
            component
                .borrow_mut()
                .register_config_handler(config.clone())?;
        }
        Ok(())
    }
}

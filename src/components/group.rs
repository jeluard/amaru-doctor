use crate::components::Component;
use crate::{action::Action, config::Config, tui::Event};
use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    Frame,
    layout::{Rect, Size},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::trace;

#[derive(Default)]
pub struct ComponentGroup {
    components: Vec<Box<dyn Component>>,
}

impl ComponentGroup {
    pub fn new(components: Vec<Box<dyn Component>>) -> Self {
        Self { components }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Component>> {
        self.components.iter_mut()
    }

    pub fn components_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
        &mut self.components
    }
}

impl Component for ComponentGroup {
    fn update(&mut self, action: Action) -> Result<Vec<Action>> {
        let mut results = Vec::new();
        for component in self.components.iter_mut() {
            let actions = component.update(action.clone())?;
            results.extend(actions);
        }
        Ok(results)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        for component in self.components.iter_mut() {
            component.draw(frame, area)?;
        }
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Result<Vec<Action>> {
        let mut results = Vec::new();
        for component in self.components.iter_mut() {
            let actions = component.handle_events(event.clone())?;
            results.extend(actions);
        }
        Ok(results)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        trace!("ComponentGroup::handle_key_event - key: {:?}", key);
        let mut results = Vec::new();
        for (i, component) in self.components.iter_mut().enumerate() {
            trace!("Forwarding to child component [{}]", i);
            results.extend(component.handle_key_event(key)?);
        }
        Ok(results)
    }

    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<Vec<Action>> {
        let mut results = Vec::new();
        for component in self.components.iter_mut() {
            let actions = component.handle_mouse_event(mouse)?;
            results.extend(actions);
        }
        Ok(results)
    }

    fn init(&mut self, area: Size) -> Result<()> {
        for component in self.components.iter_mut() {
            component.init(area)?;
        }
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        for component in self.components.iter_mut() {
            component.register_action_handler(tx.clone())?;
        }
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        for component in self.components.iter_mut() {
            component.register_config_handler(config.clone())?;
        }
        Ok(())
    }
}

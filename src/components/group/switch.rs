use crate::{
    action::Action,
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::{Shared, SharedGetter},
};
use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    Frame,
    layout::{Rect, Size},
};
use std::{fmt::Debug, hash::Hash};
use tokio::sync::mpsc::UnboundedSender;
use tracing::trace;

pub struct SwitchComponent<'a, K>
where
    K: Eq + Hash + Debug,
{
    shared: SharedGetter<'a, K>,
    focus: FocusState,
    components: Vec<(K, Shared<'a, dyn FocusableComponent + 'a>)>,
}

impl<'a, K> SwitchComponent<'a, K>
where
    K: Eq + Hash + Debug + Clone,
{
    pub fn new(
        shared: SharedGetter<'a, K>,
        components: Vec<(K, Shared<'a, dyn FocusableComponent + 'a>)>,
    ) -> Self {
        let k = &components[0].0;
        trace!("SwitchComponent init'ing, first selected: {:?}", k);
        Self {
            shared,
            focus: FocusState::default(),
            components,
        }
    }

    fn current_key(&self) -> Option<K> {
        self.shared.borrow().get().map(|r| r.clone())
    }

    fn current(&self) -> Option<&Shared<'a, dyn FocusableComponent + 'a>> {
        self.current_key()
            .and_then(|k| self.components.iter().find(|(key, _)| *key == k))
            .map(|(_, c)| c)
    }

    fn current_mut(&mut self) -> Option<&Shared<'a, dyn FocusableComponent + 'a>> {
        self.current_key()
            .and_then(|k| self.components.iter().find(|(key, _)| *key == k))
            .map(|(_, c)| c)
    }
}

impl<'a, K> FocusableComponent for SwitchComponent<'a, K>
where
    K: Eq + Hash + Clone + Debug,
{
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }

    fn has_focus(&self) -> bool {
        self.current()
            .map(|c| c.borrow().has_focus())
            .unwrap_or(false)
    }

    fn set_focus(&mut self, b: bool) {
        self.focus.set(b);
        if let Some(c) = self.current_mut() {
            c.borrow_mut().set_focus(b);
        }
    }
}

impl<'a, K> Component for SwitchComponent<'a, K>
where
    K: Eq + Hash + Clone + Debug,
{
    fn debug_name(&self) -> String {
        format!(
            "{:?}",
            self.components.iter().map(|(k, _)| k).collect::<Vec<_>>()
        )
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if let Some(c) = self.current_mut() {
            c.borrow_mut().draw(frame, area)
        } else {
            Ok(())
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        let selected_key = self.shared.borrow().get().map(|r| r.clone());

        if !self.has_focus() {
            trace!("{}: no focus", self.debug_name());
            return Ok(vec![]);
        }

        if let Some(c) = self.current() {
            trace!(
                "{}: forwarding key to component {:?}",
                self.debug_name(),
                selected_key
            );
            c.borrow_mut().handle_key_event(key)
        } else {
            trace!("{}: nothing selected", self.debug_name());
            Ok(vec![])
        }
    }

    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<Vec<Action>> {
        if let Some(c) = self.current_mut() {
            c.borrow_mut().handle_mouse_event(mouse)
        } else {
            Ok(vec![])
        }
    }

    fn handle_events(&mut self, event: Option<crate::tui::Event>) -> Result<Vec<Action>> {
        if let Some(c) = self.current_mut() {
            c.borrow_mut().handle_events(event)
        } else {
            Ok(vec![])
        }
    }

    fn init(&mut self, area: Size) -> Result<()> {
        for (_, c) in &self.components {
            c.borrow_mut().init(area)?;
        }
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        for (_, c) in &self.components {
            c.borrow_mut().register_action_handler(tx.clone())?;
        }
        Ok(())
    }

    fn register_config_handler(&mut self, config: crate::config::Config) -> Result<()> {
        for (_, c) in &self.components {
            c.borrow_mut().register_config_handler(config.clone())?;
        }
        Ok(())
    }
}

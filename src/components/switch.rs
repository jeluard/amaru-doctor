use super::Component;
use crate::{
    action::{Action, SelectedState, SelectsFrom},
    focus::{FocusState, Focusable, FocusableComponent},
    shared::Shared,
};
use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    Frame,
    layout::{Rect, Size},
};
use std::collections::HashMap;
use std::hash::Hash;
use tokio::sync::mpsc::UnboundedSender;

pub struct SwitchComponent<'a, K>
where
    K: Eq + Hash + Clone + SelectsFrom,
{
    selected: SelectedState<K>,
    focus: FocusState,
    components: HashMap<K, Shared<'a, dyn FocusableComponent + 'a>>,
}

impl<'a, K> SwitchComponent<'a, K>
where
    K: Eq + Hash + Clone + SelectsFrom,
{
    pub fn new(components: HashMap<K, Shared<'a, dyn FocusableComponent + 'a>>) -> Self {
        Self {
            selected: SelectedState::new(components.iter().next().map(|(k, _)| k.clone())),
            focus: FocusState::default(),
            components,
        }
    }

    fn current(&self) -> Option<&Shared<'a, dyn FocusableComponent + 'a>> {
        self.selected
            .value
            .as_ref()
            .and_then(|k| self.components.get(k))
    }

    fn current_mut(&mut self) -> Option<&mut Shared<'a, dyn FocusableComponent + 'a>> {
        self.selected
            .value
            .as_ref()
            .and_then(|k| self.components.get_mut(k))
    }
}

impl<'a, K> Focusable for SwitchComponent<'a, K>
where
    K: Eq + Hash + Clone + SelectsFrom,
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
    K: Eq + Hash + Clone + SelectsFrom,
{
    fn update(&mut self, action: Action) -> Result<Vec<Action>> {
        self.selected.update(&action);
        if let Some(c) = self.current_mut() {
            c.borrow_mut().update(action)
        } else {
            Ok(vec![])
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if let Some(c) = self.current_mut() {
            c.borrow_mut().draw(frame, area)
        } else {
            Ok(())
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            return Ok(vec![]);
        }
        if let Some(c) = self.current_mut() {
            c.borrow_mut().handle_key_event(key)
        } else {
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
        for (_, c) in self.components.iter_mut() {
            c.borrow_mut().init(area)?;
        }
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        for (_, c) in self.components.iter_mut() {
            c.borrow_mut().register_action_handler(tx.clone())?;
        }
        Ok(())
    }

    fn register_config_handler(&mut self, config: crate::config::Config) -> Result<()> {
        for (_, c) in self.components.iter_mut() {
            c.borrow_mut().register_config_handler(config.clone())?;
        }
        Ok(())
    }
}

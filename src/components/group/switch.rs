use crate::{
    action::Action,
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::{SharedFC, SharedGetterOpt},
};
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
};
use tracing::trace;

pub struct SwitchComponent<K>
where
    K: Clone + Debug + Eq,
{
    key: SharedGetterOpt<K>,
    components: Vec<(K, SharedFC)>,
    focus: FocusState,
}

impl<K> SwitchComponent<K>
where
    K: Clone + Debug + Eq,
{
    pub fn new(key: SharedGetterOpt<K>, components: Vec<(K, SharedFC)>) -> Self {
        SwitchComponent {
            key,
            components,
            focus: FocusState::default(),
        }
    }

    pub fn current_key(&self) -> Option<K> {
        self.key.borrow().get().cloned()
    }

    pub fn current(&self) -> Ref<'_, dyn FocusableComponent> {
        let key = self.current_key().expect("No matching key in components");
        let (_, comp) = self
            .components
            .iter()
            .find(|(k, _)| k == &key)
            .expect("No matching key in components");
        comp.borrow()
    }

    pub fn current_mut(&self) -> RefMut<'_, dyn FocusableComponent> {
        let key = self.current_key().expect("No matching key in components");
        let (_, comp) = self
            .components
            .iter()
            .find(|(k, _)| k == &key)
            .expect("No matching key in components");
        comp.borrow_mut()
    }
}

impl<K> FocusableComponent for SwitchComponent<K>
where
    K: Clone + Debug + Eq,
{
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }

    fn has_focus(&self) -> bool {
        self.current().has_focus()
    }

    fn set_focus(&mut self, b: bool) {
        self.current_mut().set_focus(b);
    }
}

impl<K> Component for SwitchComponent<K>
where
    K: Clone + Debug + Eq,
{
    fn debug_name(&self) -> String {
        format!(
            "{:?}",
            self.components.iter().map(|(k, _)| k).collect::<Vec<_>>()
        )
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.current_mut().draw(frame, area)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            trace!("{}: No focus", self.debug_name());
            return Ok(vec![]);
        }
        trace!("{}: Have focus", self.debug_name());
        self.current_mut().handle_key_event(key)
    }

    fn update(&mut self, action: Action) -> Result<Vec<Action>> {
        self.current_mut().update(action)
    }
}

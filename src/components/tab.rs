use std::cell::{Ref, RefCell};

use crate::{
    action::Action,
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::Getter,
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    text::ToLine,
    widgets::{Block, Borders, Tabs},
};

pub struct TabComponent<T>
where
    T: ToLine,
{
    title: &'static str,
    tabs: RefCell<Vec<T>>,
    index: usize,
    focus: FocusState,
}

impl<T> TabComponent<T>
where
    T: ToLine,
{
    pub fn new(title: &'static str, tabs: Vec<T>) -> Self {
        Self {
            title,
            tabs: RefCell::new(tabs),
            index: 0,
            focus: FocusState::default(),
        }
    }

    fn next(&mut self) {
        let len = self.tabs.borrow().len();
        self.index = (self.index + 1) % len;
    }

    fn previous(&mut self) {
        let len = self.tabs.borrow().len();
        self.index = (self.index + len - 1) % len;
    }
}

impl<T> Getter<T> for TabComponent<T>
where
    T: ToLine,
{
    fn get(&self) -> Option<Ref<T>> {
        let tabs_ref = self.tabs.borrow();
        let idx = self.index;
        if idx < tabs_ref.len() {
            Some(Ref::map(tabs_ref, |v| &v[idx]))
        } else {
            None
        }
    }
}

impl<T> FocusableComponent for TabComponent<T>
where
    T: ToLine,
{
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl<T> Component for TabComponent<T>
where
    T: ToLine,
{
    fn debug_name(&self) -> String {
        "TabComponent".into()
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            return Ok(vec![]);
        }

        match key.code {
            KeyCode::Up => self.previous(),
            KeyCode::Down => self.next(),
            _ => {}
        }

        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut block = Block::default().borders(Borders::ALL).title(self.title);

        if self.has_focus() {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let tabs_ref = self.tabs.borrow();
        let tab_lis: Vec<Line> = tabs_ref.iter().map(ToLine::to_line).collect();
        let tabs = Tabs::new(tab_lis)
            .select(self.index)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(tabs, area);
        Ok(())
    }
}

use crate::{
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::GetterOpt,
    states::Action,
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    text::ToLine,
    widgets::{Block, Borders, Tabs},
};

pub struct Cursor<T> {
    vec: Vec<T>,
    idx: usize,
}

impl<T> Cursor<T> {
    pub fn new(vec: Vec<T>) -> Self {
        if vec.is_empty() {
            panic!("Empty vec provided");
        }
        Self { vec, idx: 0 }
    }

    pub fn current(&self) -> Option<&T> {
        self.vec.get(self.idx)
    }

    pub fn index(&self) -> usize {
        self.idx
    }

    pub fn next(&mut self) {
        self.idx = (self.idx + 1) % self.vec.len();
    }

    pub fn next_back(&mut self) {
        let len = self.vec.len();
        self.idx = (len + self.idx - 1) % len;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.vec.iter()
    }
}

pub struct TabComponent<T>
where
    T: ToLine,
{
    title: String,
    tabs: Cursor<T>,
    focus: FocusState,
}

impl<T> TabComponent<T>
where
    T: ToLine,
{
    pub fn new(title: String, tabs: Vec<T>) -> Self {
        Self {
            title,
            tabs: Cursor::new(tabs),
            focus: FocusState::default(),
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

impl<T> GetterOpt<T> for TabComponent<T>
where
    T: ToLine,
{
    fn get(&self) -> Option<&T> {
        self.tabs.current()
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
            KeyCode::Up => self.tabs.next_back(),
            KeyCode::Down => self.tabs.next(),
            _ => {}
        }

        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.to_owned());

        if self.has_focus() {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let tab_lis: Vec<Line> = self.tabs.iter().map(ToLine::to_line).collect();
        let tabs = Tabs::new(tab_lis)
            .select(self.tabs.index())
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(tabs, area);
        Ok(())
    }
}

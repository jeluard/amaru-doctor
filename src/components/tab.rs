use crate::{
    components::{Component, r#static::entity_types::Entity},
    cursor::Cursor,
    focus::{FocusState, FocusableComponent},
    shared::Shared,
    states::Action,
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
    title: String,
    tabs: Shared<Cursor<T>>,
    focus: FocusState,
}

impl<T> TabComponent<T>
where
    T: ToLine,
{
    pub fn new(title: String, tabs: Shared<Cursor<T>>) -> Self {
        Self {
            title,
            tabs,
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

// impl<T> GetterOpt<T> for TabComponent<T>
// where
//     T: ToLine,
// {
//     fn get(&self) -> Option<&T> {
//         self.tabs.current()
//     }
// }

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

        let mut actions = vec![];
        match key.code {
            KeyCode::Up => actions.push(Action::ScrollUp(Entity::Nav)),
            KeyCode::Down => actions.push(Action::ScrollDown(Entity::Nav)),
            _ => {}
        }

        Ok(actions)
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
        let tabs = self.tabs.borrow();
        let tab_lis: Vec<Line> = tabs.iter().map(ToLine::to_line).collect();
        let tabs = Tabs::new(tab_lis)
            .select(tabs.index())
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(tabs, area);
        Ok(())
    }
}

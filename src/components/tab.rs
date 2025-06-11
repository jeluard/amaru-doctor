use crate::{
    app_state::AppState,
    components::Component,
    cursor::Cursor,
    shared::Shared,
    states::{Action, WidgetId},
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
    comp_id: WidgetId,
    tabs: Shared<Cursor<T>>,
    app_state: Shared<AppState>,
}

impl<T> TabComponent<T>
where
    T: ToLine,
{
    pub fn new(comp_id: WidgetId, tabs: Shared<Cursor<T>>, app_state: Shared<AppState>) -> Self {
        Self {
            comp_id,
            tabs,
            app_state,
        }
    }

    fn has_focus(&self) -> bool {
        self.app_state
            .borrow()
            .is_widget_focused(self.comp_id.clone())
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

        let mut actions = vec![];
        match key.code {
            KeyCode::Up => actions.push(Action::ScrollUp(self.comp_id.clone())),
            KeyCode::Down => actions.push(Action::ScrollDown(self.comp_id.clone())),
            _ => {}
        }

        Ok(actions)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(self.comp_id.to_string());

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

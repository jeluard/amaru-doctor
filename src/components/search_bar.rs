use crate::{
    components::Component,
    states::{Action, ComponentId},
};
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::any::Any;
use tracing::debug;

pub struct SearchBarComponent {
    id: ComponentId,
    input: String,
}

impl SearchBarComponent {
    pub fn new(id: ComponentId) -> Self {
        Self {
            id,
            input: String::new(),
        }
    }

    pub fn render_focused(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        let mut block = Block::default().title("Search").borders(Borders::ALL);
        if is_focused {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let paragraph = Paragraph::new(Line::from(Span::raw(&self.input))).block(block);
        f.render_widget(paragraph, area);
    }
}

impl Component for SearchBarComponent {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn handle_event(&mut self, event: &Event, _area: Rect) -> Vec<Action> {
        if let Event::Mouse(_) = event {
            debug!("SearchBar: Received Mouse Event!");
        }
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => {
                    self.input.push(c);
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Enter => {
                    return vec![Action::SubmitSearch(self.input.clone())];
                }
                _ => {}
            },
            Event::Mouse(mouse) => {
                if mouse.kind == MouseEventKind::Moved
                    || mouse.kind == MouseEventKind::Down(MouseButton::Left)
                {
                    debug!("SearchBar: Requesting Focus (SetFocus)");
                    return vec![Action::SetFocus(self.id)];
                }
            }
            _ => {}
        }
        Vec::new()
    }
}

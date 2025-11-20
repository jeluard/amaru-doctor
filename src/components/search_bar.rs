use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout},
    states::{Action, ComponentId},
    tui::Event,
    update::scroll::ScrollDirection,
};
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::any::Any;

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

    fn calculate_layout(&self, area: Rect, _s: &AppState) -> ComponentLayout {
        let mut layout = ComponentLayout::new();
        layout.insert(self.id, area);
        layout
    }

    /// Renders the search bar, pulling its state from the old model
    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };

        let is_focused = s.layout_model.is_focused(self.id);
        let mut block = Block::default().title("Search").borders(Borders::ALL);
        if is_focused {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let paragraph = Paragraph::new(Line::from(Span::raw(&self.input))).block(block);
        f.render_widget(paragraph, area);
    }

    fn handle_scroll(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }

    fn handle_event(&mut self, event: &Event, _area: Rect) -> Vec<Action> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char(c) => {
                    self.input.push(c);
                    return vec![Action::Render]; // Force redraw
                }
                KeyCode::Backspace => {
                    self.input.pop();
                    return vec![Action::Render];
                }
                KeyCode::Enter => {
                    // Emit the Submit Action
                    return vec![Action::SubmitSearch(self.input.clone())];
                }
                _ => {}
            }
        }
        Vec::new()
    }
}

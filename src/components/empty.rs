use super::Component;
use crate::focus::Focusable;
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
pub struct EmptyComponent {
    has_focus: bool,
}

impl Focusable for EmptyComponent {
    fn set_focus(&mut self, focus: bool) {
        self.has_focus = focus;
    }

    fn has_focus(&self) -> bool {
        self.has_focus
    }
}

impl Component for EmptyComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut block = Block::default()
            .title("Empty")
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);
        if self.has_focus() {
            block = block.border_style(Style::default().fg(Color::Blue));
        }
        frame.render_widget(block, area);
        Ok(())
    }
}

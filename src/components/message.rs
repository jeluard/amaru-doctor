use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::Span,
    widgets::Paragraph,
};

use super::Component;

#[derive(Default)]
pub struct Message {
    message: String,
}

impl Message {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Component for Message {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let span = Span::styled(&self.message, Style::new().dim());
        let paragraph = Paragraph::new(span).left_aligned();
        frame.render_widget(paragraph, area);
        Ok(())
    }
}

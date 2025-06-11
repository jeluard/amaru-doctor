use super::Component;
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Default)]
pub struct Message {
    title: Option<String>,
    message: String,
}

impl Message {
    pub fn new(title: Option<String>, message: String) -> Self {
        Self { title, message }
    }
}

impl Component for Message {
    fn debug_name(&self) -> String {
        "Message".to_string()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let span = Span::styled(self.message.to_owned(), Style::new().dim());
        let mut paragraph = Paragraph::new(span).left_aligned();

        if let Some(title) = self.title.to_owned() {
            let block = Block::default()
                .title(title)
                .title_style(Style::default().fg(Color::White))
                .borders(Borders::ALL);
            // if self.has_focus() {
            //     block = block
            //         .border_style(Style::default().fg(Color::Blue))
            //         .title_style(Style::default().fg(Color::White));
            // }
            paragraph = paragraph.block(block)
        }

        frame.render_widget(paragraph, area);
        Ok(())
    }
}

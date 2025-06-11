use super::Component;
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
pub struct EmptyComponent {}

impl Component for EmptyComponent {
    fn debug_name(&self) -> String {
        "EmptyComponent".to_string()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let block = Block::default()
            .title("Empty")
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);
        // if self.has_focus() {
        //     block = block.border_style(Style::default().fg(Color::Blue));
        // }
        frame.render_widget(block, area);
        Ok(())
    }
}

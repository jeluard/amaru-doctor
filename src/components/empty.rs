use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use super::Component;

#[derive(Default)]
pub struct EmptyComponent {}

impl Component for EmptyComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let border = Block::default().borders(Borders::ALL);
        frame.render_widget(border, area);
        Ok(())
    }
}

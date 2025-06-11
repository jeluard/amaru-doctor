use crate::{ app_state::AppState, shared::Shared, states::WidgetId, view::View};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub struct EmptyView {}

impl View for EmptyView {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: Shared<AppState>) -> Result<()> {
        let mut block = Block::default()
            .title("Empty")
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);

        if app_state.borrow().is_widget_focused(WidgetId::Empty) {
            block = block.border_style(Style::default().fg(Color::Blue));
        }

        frame.render_widget(block, area);
        Ok(())
    }
}

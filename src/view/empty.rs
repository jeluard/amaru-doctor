use crate::{app_state::AppState, controller::is_widget_focused, states::WidgetId, view::View};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub struct EmptyView {
    widget_id: WidgetId,
}

impl EmptyView {
    pub fn new(widget_id: WidgetId) -> Self {
        Self { widget_id }
    }
}

impl View for EmptyView {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let mut block = Block::default()
            .title("Empty")
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);

        if is_widget_focused(app_state, &self.widget_id) {
            block = block.border_style(Style::default().fg(Color::Blue));
        }

        frame.render_widget(block, area);
        Ok(())
    }
}

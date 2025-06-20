use crate::{app_state::AppState, controller::is_widget_focused, states::WidgetSlot, view::View};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub struct EmptyView {
    widget_slot: WidgetSlot,
}

impl EmptyView {
    pub fn new(widget_slot: WidgetSlot) -> Self {
        Self { widget_slot }
    }
}

impl View for EmptyView {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let mut block = Block::default()
            .title("Empty")
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);

        if is_widget_focused(app_state, &self.widget_slot) {
            block = block.border_style(Style::default().fg(Color::Blue));
        }

        frame.render_widget(block, area);
        Ok(())
    }
}

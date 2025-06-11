use crate::{
    app_state::AppState,
    model::window::WindowState,
    shared::Shared,
    states::WidgetId,
    ui::to_rich::{RichText, ToRichText},
    view::View,
};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct DetailsView<T> {
    pub widget_id: WidgetId,
    pub list: Shared<WindowState<T>>,
}

impl<T: ToRichText> View for DetailsView<T> {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: Shared<AppState>) -> Result<()> {
        let mut block = Block::default()
            .title(serde_plain::to_string(&self.widget_id)?)
            .borders(Borders::ALL);

        if app_state.borrow().is_widget_focused(self.widget_id.clone()) {
            block = block
                .title_style(Style::default().fg(Color::White))
                .border_style(Style::default().fg(Color::Blue));
        }

        let lines = self
            .list
            .borrow()
            .selected()
            .map_or(RichText::Single(Span::raw("None selected")), |t| {
                t.to_rich_text()
            })
            .unwrap_lines();
        let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
        // TODO: Add offset state to AppState
        // .scroll((self.scroll_offset, 0));
        frame.render_widget(paragraph, area);
        Ok(())
    }
}

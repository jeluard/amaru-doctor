use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    model::window::WindowState,
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
    pub get_list: fn(&AppState) -> &WindowState<T>,
}

impl<T: ToRichText> View for DetailsView<T> {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let mut block = Block::default()
            .title(serde_plain::to_string(&self.widget_id)?)
            .borders(Borders::ALL);

        if is_widget_focused(app_state, &self.widget_id) {
            block = block
                .title_style(Style::default().fg(Color::White))
                .border_style(Style::default().fg(Color::Blue));
        }

        let list = (self.get_list)(app_state);
        let lines = list
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

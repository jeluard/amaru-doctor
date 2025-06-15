use crate::{app_state::AppState, controller::is_widget_focused, states::WidgetId, view::View};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub struct SearchQueryView {
    widget_id: WidgetId,
    get_search_query: fn(&AppState) -> &String,
}

impl SearchQueryView {
    pub fn new(widget_id: WidgetId, get_list: fn(&AppState) -> &String) -> Self {
        Self {
            widget_id,
            get_search_query: get_list,
        }
    }
}

impl View for SearchQueryView {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let search_query = (self.get_search_query)(app_state);

        let mut block = Block::default()
            .title(self.widget_id.clone())
            .borders(Borders::ALL);

        if is_widget_focused(app_state, &self.widget_id) {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let paragraph = Paragraph::new(Line::from(Span::raw(search_query))).block(block);
        frame.render_widget(paragraph, area);

        Ok(())
    }
}

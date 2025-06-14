use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    model::cursor::Cursor,
    states::{TabOption, WidgetId},
    view::View,
};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, ToLine},
    widgets::{Block, Borders, Tabs},
};

pub struct TabsView {
    widget_id: WidgetId,
    get_tabs: fn(&AppState) -> &Cursor<TabOption>,
}

impl TabsView {
    pub fn new(widget_id: WidgetId, get_tabs: fn(&AppState) -> &Cursor<TabOption>) -> Self {
        Self {
            widget_id,
            get_tabs,
        }
    }
}

impl View for TabsView {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(self.widget_id.clone());

        if is_widget_focused(app_state, &self.widget_id) {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let cursor = (self.get_tabs)(app_state);
        let tab_lines: Vec<Line> = cursor.iter().map(ToLine::to_line).collect();
        let tabs_widget = Tabs::new(tab_lines)
            .select(cursor.index())
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(tabs_widget, area);
        Ok(())
    }
}

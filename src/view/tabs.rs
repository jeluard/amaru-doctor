use crate::{
    app_state::AppState, controller::is_widget_focused, model::cursor::Cursor, states::WidgetSlot,
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

pub struct TabsView<T> {
    title: &'static str,
    widget_slot: WidgetSlot,
    get_tabs: fn(&AppState) -> &Cursor<T>,
}

impl<T> TabsView<T> {
    pub fn new(
        title: &'static str,
        widget_slot: WidgetSlot,
        get_tabs: fn(&AppState) -> &Cursor<T>,
    ) -> Self {
        Self {
            title,
            widget_slot,
            get_tabs,
        }
    }
}

impl<T: ToLine> View for TabsView<T> {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let mut block = Block::default().borders(Borders::ALL).title(self.title);

        if is_widget_focused(app_state, &self.widget_slot) {
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

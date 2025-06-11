use crate::{
    app_state::AppState,
    model::cursor::Cursor,
    shared::Shared,
    states::{Tab, WidgetId},
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
    pub widget_id: WidgetId,
    pub tabs: Shared<Cursor<Tab>>,
}

impl View for TabsView {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: Shared<AppState>) -> Result<()> {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(serde_plain::to_string(&self.widget_id)?);

        if app_state.borrow().is_widget_focused(self.widget_id.clone()) {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let tab_brw = self.tabs.borrow();
        let tab_lis: Vec<Line> = tab_brw.iter().map(ToLine::to_line).collect();
        let tabs = Tabs::new(tab_lis)
            .select(tab_brw.index())
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(tabs, area);
        Ok(())
    }
}

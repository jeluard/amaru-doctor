use crate::{
    app_state::AppState, controller::is_widget_focused, model::window::WindowState,
    states::WidgetId, ui::to_list_item::ToListItem, view::View,
};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub struct ListView<T> {
    pub widget_id: WidgetId,
    pub get_list: fn(&AppState) -> &WindowState<T>,
}

impl<T: ToListItem> View for ListView<T> {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        // TODO: Capture somewhere else so that this doesn't need to be mut
        // self.list.borrow_mut().set_window_size(area.rows().count());

        let list = (self.get_list)(app_state);
        let (view, selected) = list.window_view();
        let items: Vec<ListItem> = view.iter().map(|i| i.to_list_item()).collect();

        let mut block = Block::default()
            .title(serde_plain::to_string(&self.widget_id)?)
            .borders(Borders::ALL);
        if is_widget_focused(app_state, &self.widget_id) {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let list = List::new(items)
            .highlight_symbol(">> ")
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .block(block);

        let mut state = ListState::default();
        state.select(Some(selected));
        frame.render_stateful_widget(list, area, &mut state);

        Ok(())
    }
}

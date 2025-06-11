use crate::{
    app_state::AppState, model::window::WindowState, states::WidgetId,
    ui::to_list_item::ToListItem, view::View,
};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::cell::RefCell;

pub struct ListView<T> {
    pub widget_id: WidgetId,
    pub get_list: fn(&AppState) -> &RefCell<WindowState<T>>,
}

impl<T: ToListItem> View for ListView<T> {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        // TODO: Capture somewhere else so that this doesn't need to be mut
        // self.list.borrow_mut().set_window_size(area.rows().count());

        let list = (self.get_list)(app_state);
        let binding = list.borrow();
        let (view, selected) = binding.window_view();
        let items: Vec<ListItem> = view.iter().map(|i| i.to_list_item()).collect();

        let mut block = Block::default()
            .title(serde_plain::to_string(&self.widget_id)?)
            .borders(Borders::ALL);
        if app_state.is_widget_focused(self.widget_id.clone()) {
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

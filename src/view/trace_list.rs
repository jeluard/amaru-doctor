use crate::{model::dynamic_list::DynamicList, otel::id::TraceId};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub fn render_traces(frame: &mut Frame, area: Rect, list: &DynamicList<TraceId>, is_focused: bool) {
    let mut block = Block::default().title("Traces").borders(Borders::ALL);
    if is_focused {
        block = block.border_style(Style::default().fg(Color::Blue));
    }

    let items: Vec<ListItem> = list
        .items()
        .iter()
        .map(ToString::to_string)
        .map(ListItem::new)
        .collect();

    let widget = List::new(items)
        .highlight_symbol(">> ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .block(block);

    let mut state = ListState::default();
    state.select(list.selected_index());
    frame.render_stateful_widget(widget, area, &mut state);
}

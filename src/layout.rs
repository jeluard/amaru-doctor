use crate::{
    app_state::AppState,
    controller::get_selected_widget,
    states::{WidgetId, WidgetSlot},
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub type SlotLayout = Vec<(WidgetSlot, Rect)>;
pub type SlotMap = HashMap<WidgetSlot, WidgetId>;

pub fn compute_slot_layout(area: Rect) -> SlotLayout {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);
    let (left, right) = (columns[0], columns[1]);

    let left_regions = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Fill(1),
        ])
        .split(left);
    let (nav, options, list) = (left_regions[0], left_regions[1], left_regions[2]);

    let right_regions = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(right);
    let (_search_query, details) = (right_regions[0], right_regions[1]);

    vec![
        (WidgetSlot::Nav, nav),
        (WidgetSlot::NavType, options),
        (WidgetSlot::List, list),
        (WidgetSlot::Details, details),
    ]
}

pub fn compute_slot_map(app_state: &AppState) -> SlotMap {
    let mut widgets = HashMap::new();
    WidgetSlot::iter().for_each(|s| {
        if let Some(wid) = get_selected_widget(app_state, &s) {
            widgets.insert(s, wid);
        }
    });
    widgets
}

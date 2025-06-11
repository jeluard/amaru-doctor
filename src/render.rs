use crate::states::WidgetSlot;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub type SlotLayout = Vec<(WidgetSlot, Rect)>;

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

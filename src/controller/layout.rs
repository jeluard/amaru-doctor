use crate::{
    app_state::AppState,
    controller::resolve_placed_widget_id,
    states::{WidgetId, WidgetSlot},
};
use color_eyre::{Result, eyre::eyre};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;
use strum::IntoEnumIterator;

/// Determines where to render a slot
pub type SlotLayout = HashMap<WidgetSlot, Rect>;

/// Determines what widget to render within a slot
pub type SlotWidgets = HashMap<WidgetSlot, WidgetId>;

pub fn compute_slot_layout(area: Rect) -> Result<SlotLayout> {
    let [header, body, footer] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area)
    else {
        return Err(eyre!("Couldn't destructure left and right columns"));
    };

    let [left, right] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(body)
    else {
        return Err(eyre!("Couldn't destructure left and right columns"));
    };

    let [nav, options, list] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Fill(3),
        ])
        .split(left)
    else {
        return Err(eyre!("Couldn't destructure left column rows"));
    };

    let [search, details] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(right)
    else {
        return Err(eyre!("Couldn't destructure right column rows"));
    };

    let layout = WidgetSlot::iter()
        .map(|slot| {
            let rect = match slot {
                WidgetSlot::Header => header,
                WidgetSlot::Nav => nav,
                WidgetSlot::SearchBar => search,
                WidgetSlot::Options => options,
                WidgetSlot::List => list,
                WidgetSlot::Details => details,
                WidgetSlot::Footer => footer,
            };
            (slot, rect)
        })
        .collect();

    Ok(layout)
}

pub fn compute_slot_widgets(app_state: &AppState) -> SlotWidgets {
    WidgetSlot::iter()
        .map(|slot| (slot, resolve_placed_widget_id(app_state, slot)))
        .collect()
}

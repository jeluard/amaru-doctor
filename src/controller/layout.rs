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

pub fn compute_ledger_slot_layout(area: Rect) -> Result<SlotLayout> {
    let [top_line, rest, bottom_line] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area)
    else {
        return Err(eyre!(
            "Couldn't destructure top line, rest, and bottom line"
        ));
    };

    let [header, body] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(rest)
    else {
        return Err(eyre!("Couldn't destructure header and body"));
    };

    let [mode, nav, search] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(5),
        ])
        .split(header)
    else {
        return Err(eyre!("Couldn't destructure header left and right"));
    };

    let [left, right] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(body)
    else {
        return Err(eyre!("Couldn't destructure left and right columns"));
    };

    let [options, list] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Fill(3)])
        .split(left)
    else {
        return Err(eyre!("Couldn't destructure left column rows"));
    };

    let layout = WidgetSlot::iter()
        .map(|slot| {
            let rect = match slot {
                WidgetSlot::TopLine => top_line,
                WidgetSlot::StoreOption => mode,
                WidgetSlot::LedgerMode => nav,
                WidgetSlot::SearchBar => search,
                WidgetSlot::Options => options,
                WidgetSlot::List => list,
                WidgetSlot::Details => right,
                WidgetSlot::BottomLine => bottom_line,
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

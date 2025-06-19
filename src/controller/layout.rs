use crate::{
    app_state::AppState,
    controller::resolve_placed_widget_id,
    states::{StoreOption, WidgetId, WidgetSlot},
};
use color_eyre::{Result, eyre::eyre};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;
use strum::IntoEnumIterator;

/// Determines what widget to render within a slot
pub type SlotWidgets = HashMap<WidgetSlot, WidgetId>;

pub enum SlotLayout {
    Ledger(LedgerLayout),
    Chain(ChainLayout),
}

pub struct LedgerLayout {
    pub top_line: Rect,
    pub store_option: Rect,
    pub ledger_mode: Rect,
    pub search_bar: Rect,
    pub options: Rect,
    pub list: Rect,
    pub details: Rect,
    pub bottom_line: Rect,
}

pub struct ChainLayout {
    pub top_line: Rect,
    pub store_option: Rect,
    pub search_bar: Rect,
    pub details: Rect,
    pub bottom_line: Rect,
}

impl SlotLayout {
    pub fn renderables(&self) -> impl Iterator<Item = (WidgetSlot, &Rect)> {
        let items = match self {
            SlotLayout::Ledger(l) => vec![
                (WidgetSlot::TopLine, &l.top_line),
                (WidgetSlot::StoreOption, &l.store_option),
                (WidgetSlot::LedgerMode, &l.ledger_mode),
                (WidgetSlot::SearchBar, &l.search_bar),
                (WidgetSlot::Options, &l.options),
                (WidgetSlot::List, &l.list),
                (WidgetSlot::Details, &l.details),
                (WidgetSlot::BottomLine, &l.bottom_line),
            ],
            SlotLayout::Chain(c) => vec![
                (WidgetSlot::TopLine, &c.top_line),
                (WidgetSlot::StoreOption, &c.store_option),
                (WidgetSlot::SearchBar, &c.search_bar),
                (WidgetSlot::Details, &c.details),
                (WidgetSlot::BottomLine, &c.bottom_line),
            ],
        };
        items.into_iter()
    }
}

pub fn compute_slot_layout(area: Rect, store_option: &StoreOption) -> Result<SlotLayout> {
    match store_option {
        StoreOption::Ledger => compute_ledger_slot_layout(area),
        StoreOption::Chain => compute_chain_slot_layout(area),
    }
}

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

    let [store_option, ledger_mode, search_bar] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Fill(1),
        ])
        .split(header)
    else {
        return Err(eyre!("Couldn't destructure header left and right"));
    };

    let [left, details] = *Layout::default()
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

    Ok(SlotLayout::Ledger(LedgerLayout {
        top_line,
        store_option,
        ledger_mode,
        search_bar,
        options,
        list,
        details,
        bottom_line,
    }))
}

pub fn compute_chain_slot_layout(area: Rect) -> Result<SlotLayout> {
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

    let [header, details] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(rest)
    else {
        return Err(eyre!("Couldn't destructure header and body"));
    };

    let [store_option, search_bar] = *Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Fill(1)])
        .split(header)
    else {
        return Err(eyre!("Couldn't destructure header left and right"));
    };

    Ok(SlotLayout::Chain(ChainLayout {
        top_line,
        store_option,
        search_bar,
        details,
        bottom_line,
    }))
}

pub fn compute_slot_widgets(app_state: &AppState) -> SlotWidgets {
    WidgetSlot::iter()
        .map(|slot| (slot, resolve_placed_widget_id(app_state, slot)))
        .collect()
}

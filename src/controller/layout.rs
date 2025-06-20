use crate::{
    app_state::AppState,
    controller::resolve_placed_widget_id,
    states::{StoreOption, WidgetId, WidgetSlot},
};
use either::Either::{self, Left, Right};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;
use strum::IntoEnumIterator;

/// Determines what widget to render within a slot
pub type SlotWidgets = HashMap<WidgetSlot, WidgetId>;

pub type SlotLayout = HashMap<WidgetSlot, Rect>;

pub fn compute_slot_layout(app_state: &AppState, area: Rect) -> SlotLayout {
    let spec = build_layout_spec(app_state);
    let mut out = HashMap::new();
    walk_layout(&mut out, &spec, area);
    out
}

fn walk_layout(out: &mut HashMap<WidgetSlot, Rect>, spec: &LayoutSpec, area: Rect) {
    let constraints: Vec<Constraint> = spec.constraints.iter().map(|(c, _)| *c).collect();
    let regions = Layout::default()
        .direction(spec.direction)
        .constraints(constraints)
        .split(area);

    for ((_, slot_or_spec), sub_area) in spec.constraints.iter().zip(regions.iter()) {
        match slot_or_spec {
            Left(slot) => {
                out.insert(*slot, *sub_area);
            }
            Right(child_spec) => {
                walk_layout(out, child_spec, *sub_area);
            }
        }
    }
}

// pub fn compute_chain_slot_layout(area: Rect) -> Result<SlotLayout> {
//     let [top_line, rest, bottom_line] = *Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Length(1),
//             Constraint::Fill(1),
//             Constraint::Length(1),
//         ])
//         .split(area)
//     else {
//         return Err(eyre!(
//             "Couldn't destructure top line, rest, and bottom line"
//         ));
//     };

//     let [header, details] = *Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([Constraint::Length(3), Constraint::Fill(1)])
//         .split(rest)
//     else {
//         return Err(eyre!("Couldn't destructure header and body"));
//     };

//     let [store_option, search_bar] = *Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Length(20), Constraint::Fill(1)])
//         .split(header)
//     else {
//         return Err(eyre!("Couldn't destructure header left and right"));
//     };

//     Ok(SlotLayout::Chain(ChainLayout {
//         top_line,
//         store_option,
//         search_bar,
//         details,
//         bottom_line,
//     }))
// }

pub fn compute_slot_widgets(app_state: &AppState) -> SlotWidgets {
    WidgetSlot::iter()
        .map(|slot| (slot, resolve_placed_widget_id(app_state, slot)))
        .collect()
}

pub struct LayoutSpec {
    direction: Direction,
    constraints: Vec<(Constraint, Either<WidgetSlot, LayoutSpec>)>,
}

pub fn build_layout_spec(app_state: &AppState) -> LayoutSpec {
    match app_state.store_option.current() {
        StoreOption::Ledger => build_ledger_ls(app_state),
        StoreOption::Chain => build_chain_ls(app_state),
    }
}

fn build_ledger_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Length(1), Left(WidgetSlot::TopLine)),
            (Constraint::Fill(1), Right(build_ledger_rest_ls(app_state))),
            (Constraint::Length(1), Left(WidgetSlot::BottomLine)),
        ],
    }
}

fn build_ledger_rest_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (
                Constraint::Length(3),
                Right(build_ledger_header_ls(app_state)),
            ),
            (Constraint::Fill(1), Right(build_ledger_body_ls(app_state))),
        ],
    }
}

fn build_ledger_header_ls(_app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Length(20), Left(WidgetSlot::StoreOption)),
            (Constraint::Length(20), Left(WidgetSlot::LedgerMode)),
            (Constraint::Fill(1), Left(WidgetSlot::SearchBar)),
        ],
    }
}

fn build_ledger_body_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (
                Constraint::Percentage(20),
                Right(build_ledger_left_col_ls(app_state)),
            ),
            (Constraint::Percentage(80), Left(WidgetSlot::Details)),
        ],
    }
}

fn build_ledger_left_col_ls(_app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Fill(1), Left(WidgetSlot::Options)),
            (Constraint::Fill(3), Left(WidgetSlot::List)),
        ],
    }
}

fn build_chain_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (Constraint::Length(1), Left(WidgetSlot::TopLine)),
            (Constraint::Fill(1), Right(build_chain_rest_ls(app_state))),
            (Constraint::Length(1), Left(WidgetSlot::BottomLine)),
        ],
    }
}

fn build_chain_rest_ls(app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Vertical,
        constraints: vec![
            (
                Constraint::Length(3),
                Right(build_chain_header_ls(app_state)),
            ),
            (Constraint::Fill(1), Left(WidgetSlot::Details)),
        ],
    }
}

fn build_chain_header_ls(_app_state: &AppState) -> LayoutSpec {
    LayoutSpec {
        direction: Direction::Horizontal,
        constraints: vec![
            (Constraint::Length(20), Left(WidgetSlot::StoreOption)),
            (Constraint::Fill(1), Left(WidgetSlot::SearchBar)),
        ],
    }
}

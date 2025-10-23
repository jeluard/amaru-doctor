use crate::{
    ScreenMode,
    controller::layout::build_layout_spec,
    states::{
        InspectOption, LedgerMode,
        WidgetSlot::{self},
    },
};
use either::Either::{self, Left, Right};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;

pub mod layout;

pub type SlotLayout = HashMap<WidgetSlot, Rect>;

pub struct LayoutSpec {
    direction: Direction,
    constraints: Vec<(Constraint, Either<WidgetSlot, LayoutSpec>)>,
}

pub fn compute_slot_layout(
    screen_mode: ScreenMode,
    inspect_tabs: InspectOption,
    ledger_mode: LedgerMode,
    area: Rect,
) -> SlotLayout {
    let spec = build_layout_spec(screen_mode, inspect_tabs, ledger_mode);
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

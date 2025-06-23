use crate::{
    app_state::AppState,
    controller::layout::build_layout_spec,
    states::WidgetSlot::{self},
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

pub fn is_widget_focused(app_state: &AppState, widget_slot: WidgetSlot) -> bool {
    app_state.slot_focus == widget_slot
}

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

use crate::{
    ScreenMode,
    controller::layout::build_layout_spec,
    states::{ComponentId, InspectOption, LedgerBrowse, LedgerMode, LedgerSearch},
};
use either::Either::{self, Left, Right};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;

pub mod layout;

pub type ComponentLayout = HashMap<ComponentId, Rect>;

/// Holds the state required to calculate the specific component layout.
#[derive(Debug, Clone)]
pub struct LayoutContext {
    pub screen_mode: ScreenMode,
    pub inspect_option: InspectOption,
    pub ledger_mode: LedgerMode,
    pub ledger_browse: LedgerBrowse,
    pub ledger_search: LedgerSearch,
}

pub struct LayoutSpec {
    direction: Direction,
    constraints: Vec<(Constraint, Either<ComponentId, LayoutSpec>)>,
}

pub fn compute_component_layout(ctx: LayoutContext, area: Rect) -> ComponentLayout {
    let spec = build_layout_spec(&ctx);
    let mut out = HashMap::new();
    walk_layout(&mut out, &spec, area);
    out
}

fn walk_layout(out: &mut HashMap<ComponentId, Rect>, spec: &LayoutSpec, area: Rect) {
    let constraints: Vec<Constraint> = spec.constraints.iter().map(|(c, _)| *c).collect();
    let regions = Layout::default()
        .direction(spec.direction)
        .constraints(constraints)
        .split(area);

    for ((_, slot_or_spec), sub_area) in spec.constraints.iter().zip(regions.iter()) {
        match slot_or_spec {
            Left(component_id) => {
                out.insert(*component_id, *sub_area);
            }
            Right(child_spec) => {
                walk_layout(out, child_spec, *sub_area);
            }
        }
    }
}

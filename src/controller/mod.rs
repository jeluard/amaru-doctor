use crate::states::ComponentId;
use either::Either::{self, Left, Right};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::collections::HashMap;

pub type ComponentLayout = HashMap<ComponentId, Rect>;

/// A recursive layout specification helper.
pub struct LayoutSpec {
    pub direction: Direction,
    pub constraints: Vec<(Constraint, Either<ComponentId, LayoutSpec>)>,
}

/// Helper to convert a recursive LayoutSpec into a flat ComponentLayout map.
pub fn walk_layout(out: &mut ComponentLayout, spec: &LayoutSpec, area: Rect) {
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

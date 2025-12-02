use crate::states::ComponentId;
use either::Either::{self, Left, Right};
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveFocus {
    Up,
    Down,
    Left,
    Right,
}

trait RectExt {
    fn center(&self) -> Position;
}

impl RectExt for Rect {
    fn center(&self) -> Position {
        Position {
            x: self.x + self.width / 2,
            y: self.y + self.height / 2,
        }
    }
}

/// Stateless helper to find the best candidate for focus movement.
pub fn find_next_focus(
    layout: &ComponentLayout,
    current_focus: ComponentId,
    direction: MoveFocus,
) -> Option<ComponentId> {
    let current_rect = layout.get(&current_focus).copied()?;
    let current_center = current_rect.center();

    let candidates = layout.iter().filter(|(id, _)| **id != current_focus);

    let valid_candidates = candidates.filter(|(_, rect)| match direction {
        MoveFocus::Up => rect.bottom() <= current_rect.top(),
        MoveFocus::Down => rect.top() >= current_rect.bottom(),
        MoveFocus::Left => rect.right() <= current_rect.left(),
        MoveFocus::Right => rect.left() >= current_rect.right(),
    });

    valid_candidates
        .min_by_key(|(_, rect)| {
            let target_center = rect.center();
            match direction {
                MoveFocus::Up | MoveFocus::Down => (
                    target_center.y.abs_diff(current_center.y),
                    target_center.x.abs_diff(current_center.x),
                ),
                MoveFocus::Left | MoveFocus::Right => (
                    target_center.x.abs_diff(current_center.x),
                    target_center.y.abs_diff(current_center.y),
                ),
            }
        })
        .map(|(id, _)| *id)
}

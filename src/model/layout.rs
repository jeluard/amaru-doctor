use crate::{
    components::ComponentLayout,
    states::{ComponentId, InspectOption, LedgerMode},
};
use ratatui::layout::{Position, Rect};
use tracing::warn;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveFocus {
    Up,
    Down,
    Left,
    Right,
}

pub trait RectExt {
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
/// Returns `None` if no suitable candidate is found.
pub fn find_next_focus(
    layout: &ComponentLayout,
    current_focus: ComponentId,
    direction: MoveFocus,
) -> Option<ComponentId> {
    let current_rect = layout.get(&current_focus).copied()?;
    let current_center = current_rect.center();

    let candidates = layout.iter().filter(|(id, _)| **id != current_focus);

    // Filter candidates based on direction relative to current rect
    let valid_candidates = candidates.filter(|(_, rect)| match direction {
        MoveFocus::Up => rect.bottom() <= current_rect.top(),
        MoveFocus::Down => rect.top() >= current_rect.bottom(),
        MoveFocus::Left => rect.right() <= current_rect.left(),
        MoveFocus::Right => rect.left() >= current_rect.right(),
    });

    // Find the closest candidate
    valid_candidates
        .min_by_key(|(_, rect)| {
            let target_center = rect.center();
            match direction {
                MoveFocus::Up | MoveFocus::Down => (
                    target_center.y.abs_diff(current_center.y), // Primary axis: Y distance
                    target_center.x.abs_diff(current_center.x), // Secondary axis: X alignment
                ),
                MoveFocus::Left | MoveFocus::Right => (
                    target_center.x.abs_diff(current_center.x), // Primary axis: X distance
                    target_center.y.abs_diff(current_center.y), // Secondary axis: Y alignment
                ),
            }
        })
        .map(|(id, _)| *id)
}

#[derive(Debug)]
pub struct LayoutModel {
    pub layout: ComponentLayout,
    focus: ComponentId,
}

impl LayoutModel {
    pub fn new(_inspect_tabs: InspectOption, _ledger_mode: LedgerMode, _frame_area: Rect) -> Self {
        Self {
            layout: ComponentLayout::default(),
            focus: ComponentId::default(),
        }
    }

    pub fn set_layout(&mut self, new_layout: ComponentLayout) {
        self.layout = new_layout;
    }

    pub fn get_layout(&self) -> &ComponentLayout {
        &self.layout
    }

    pub fn get_focus(&self) -> ComponentId {
        self.focus
    }

    pub fn is_focused(&self, query: ComponentId) -> bool {
        query == self.focus
    }

    pub fn set_focus(&mut self, new_focus: ComponentId) {
        self.focus = new_focus
    }

    /// Sets the focus to the widget located at the given screen coordinates.
    /// Returns `true` if the focus was changed.
    pub fn set_focus_by_location(&mut self, column: u16, row: u16) -> bool {
        self.find_hovered_component(column, row)
            .map(|(component_id, _)| self.focus = component_id)
            .is_some()
    }

    /// Moves the focus to the next logical widget in the given direction,
    /// wrapping around the screen edges if necessary.
    pub fn set_focus_by_move(&mut self, direction: MoveFocus) {
        let Some(current_rect) = self.layout.get(&self.focus).copied() else {
            warn!("No rect in layout for current focus {:?}", self.focus);
            return;
        };
        let current_center = current_rect.center();

        // Try to find the best candidate in the desired direction.
        let best_candidate = self
            .layout
            .iter()
            .filter(|(slot, _)| **slot != self.focus)
            .filter(|(_, rect)| match direction {
                MoveFocus::Up => rect.bottom() <= current_rect.top(),
                MoveFocus::Down => rect.top() >= current_rect.bottom(),
                MoveFocus::Left => rect.right() <= current_rect.left(),
                MoveFocus::Right => rect.left() >= current_rect.right(),
            })
            .min_by_key(|(_, rect)| {
                let target_center = rect.center();
                match direction {
                    MoveFocus::Up | MoveFocus::Down => (
                        target_center.x.abs_diff(current_center.x),
                        target_center.y.abs_diff(current_center.y),
                    ),
                    MoveFocus::Left | MoveFocus::Right => (
                        target_center.y.abs_diff(current_center.y),
                        target_center.x.abs_diff(current_center.x),
                    ),
                }
            });

        // If no candidate was found, try to "wrap around".
        let new_focus = best_candidate.or_else(|| {
            self.layout
                .iter()
                .filter(|(slot, _)| **slot != self.focus)
                .min_by_key(|(_, rect)| match direction {
                    MoveFocus::Right => (rect.left(), rect.top()),
                    MoveFocus::Left => (u16::MAX - rect.right(), rect.top()),
                    MoveFocus::Down => (rect.top(), rect.left()),
                    MoveFocus::Up => (u16::MAX - rect.bottom(), rect.left()),
                })
        });

        // If any candidate was found, update the focus.
        if let Some((new_focus_slot, _)) = new_focus {
            self.focus = *new_focus_slot;
        }
    }

    /// Finds the component and its `Rect` at the given screen coordinates.
    pub fn find_hovered_component(&self, column: u16, row: u16) -> Option<(ComponentId, Rect)> {
        self.layout
            .iter()
            .filter(|(_slot, rect)| {
                column >= rect.x
                    && column < rect.x + rect.width
                    && row >= rect.y
                    && row < rect.y + rect.height
            })
            .min_by_key(|(_slot, rect)| rect.width as u32 * rect.height as u32)
            .map(|(slot, rect)| (*slot, *rect))
    }
}

use crate::{
    ScreenMode,
    controller::{SlotLayout, compute_slot_layout},
    states::{InspectOption, LedgerMode, WidgetSlot},
};
use ratatui::layout::{Position, Rect};
use tracing::warn;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveFocus {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct LayoutModel {
    layout: SlotLayout,
    focus: WidgetSlot,
}

impl LayoutModel {
    pub fn new(
        screen_mode: ScreenMode,
        inspect_tabs: InspectOption,
        ledger_mode: LedgerMode,
        frame_area: Rect,
    ) -> Self {
        Self {
            layout: compute_slot_layout(screen_mode, inspect_tabs, ledger_mode, frame_area),
            focus: WidgetSlot::default(),
        }
    }

    pub fn update_layout(&mut self, new_layout: SlotLayout) {
        self.layout = new_layout;
    }

    pub fn get_layout(&self) -> &SlotLayout {
        &self.layout
    }

    pub fn get_focus(&self) -> WidgetSlot {
        self.focus
    }

    pub fn is_focused(&self, query: WidgetSlot) -> bool {
        query == self.focus
    }

    pub fn set_focus(&mut self, new_focus: WidgetSlot) {
        self.focus = new_focus
    }

    /// Sets the focus to the widget located at the given screen coordinates.
    /// Returns `true` if the focus was changed.
    pub fn set_focus_by_location(&mut self, column: u16, row: u16) -> bool {
        self.find_hovered_slot(column, row)
            .map(|(slot, _)| self.focus = slot)
            .is_some()
    }

    /// Moves the focus to the next logical widget in the given direction,
    /// wrapping around the screen edges if necessary.
    pub fn set_focus_by_move(&mut self, direction: MoveFocus) {
        let Some(current_rect) = self.layout.get(&self.focus).copied() else {
            warn!("No rect in layout for current focus {}", self.focus);
            return;
        };
        let current_center = current_rect.center();

        // Try to find the best candidate in the desired direction.
        let best_candidate = self
            .layout
            .iter()
            .filter(|(slot, _)| **slot != self.focus && WidgetSlot::focusable().contains(slot))
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
                .filter(|(slot, _)| **slot != self.focus && WidgetSlot::focusable().contains(slot))
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

    /// Finds the widget slot and its `Rect` at the given screen coordinates.
    pub fn find_hovered_slot(&self, column: u16, row: u16) -> Option<(WidgetSlot, Rect)> {
        self.layout.iter().find_map(|(slot, rect)| {
            if column >= rect.x
                && column < rect.x + rect.width
                && row >= rect.y
                && row < rect.y + rect.height
            {
                Some((*slot, *rect))
            } else {
                None
            }
        })
    }
}

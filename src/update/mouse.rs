use crate::{
    app_state::AppState,
    states::{Action, WidgetSlot},
    update::Update,
};
use crossterm::event::{MouseButton, MouseEventKind};
use tracing::{debug, warn};

#[derive(Debug, Default, Clone, Copy)]
pub enum MouseState {
    #[default]
    NotDragging,
    Dragging {
        last_drag_row: u16,
    },
}

pub struct MouseEventUpdate;

impl Update for MouseEventUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::MouseEvent(mouse_event) = action else {
            return Vec::new();
        };

        if mouse_event.kind == MouseEventKind::Moved {
            // Ignore noisy mouse moves
            return Vec::new();
        }

        let Some((slot, rect)) = s
            .layout_model
            .find_hovered_slot(mouse_event.column, mouse_event.row)
        else {
            debug!("Couldn't find slot for mouse event {:?}", mouse_event);
            return Vec::new();
        };
        debug!(
            "Found slot {} and rect {} for mouse event {:?}",
            slot, rect, mouse_event
        );

        match (slot, mouse_event.kind) {
            (WidgetSlot::List, MouseEventKind::Down(MouseButton::Left)) => {
                // This is either a simple click or the beginning of a drag
                s.mouse_state = MouseState::Dragging {
                    last_drag_row: mouse_event.row,
                };

                vec![Action::MouseClick(mouse_event.column, mouse_event.row)]
            }

            (_, MouseEventKind::Up(MouseButton::Left)) => {
                debug!("Left mouse button released up--end of either a simple click or a drag");
                s.mouse_state = MouseState::NotDragging;
                Vec::new()
            }

            (WidgetSlot::List, MouseEventKind::Drag(MouseButton::Left)) => {
                let MouseState::Dragging { last_drag_row, .. } = &mut s.mouse_state else {
                    warn!("Unexpected: received a Drag MouseEvent without a prior Down event");
                    return Vec::new();
                };

                if mouse_event.row > *last_drag_row {
                    let rows_scrolled = mouse_event.row.saturating_sub(*last_drag_row);
                    debug!(
                        "Mouse event drag row ({}) is greater than last drag row ({}), dispatching {} MouseDragDown actions",
                        mouse_event.row, *last_drag_row, rows_scrolled
                    );
                    *last_drag_row = mouse_event.row;
                    vec![Action::MouseDragDown; rows_scrolled as usize]
                } else if mouse_event.row < *last_drag_row {
                    let rows_scrolled = last_drag_row.saturating_sub(mouse_event.row);
                    debug!(
                        "Mouse event drag row ({}) is less than last drag row ({}), dispatching {} MouseDragUp actions",
                        mouse_event.row, *last_drag_row, rows_scrolled
                    );
                    *last_drag_row = mouse_event.row;
                    vec![Action::MouseDragUp; rows_scrolled as usize]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }
}

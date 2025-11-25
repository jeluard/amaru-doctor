use crate::{
    app_state::AppState,
    states::{Action, ComponentId},
    update::Update,
};
use crossterm::event::{MouseButton, MouseEventKind};

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
            return Vec::new();
        }

        let Some((component_id, _)) = s
            .layout_model
            .find_hovered_component(mouse_event.column, mouse_event.row)
        else {
            return Vec::new();
        };

        let is_list = matches!(
            component_id,
            ComponentId::LedgerAccountsList
                | ComponentId::LedgerBlockIssuersList
                | ComponentId::LedgerDRepsList
                | ComponentId::LedgerPoolsList
                | ComponentId::LedgerProposalsList
                | ComponentId::LedgerUtxosList
                | ComponentId::LedgerUtxosByAddrList
                | ComponentId::OtelTraceList
        );

        match (is_list, mouse_event.kind) {
            (true, MouseEventKind::Down(MouseButton::Left)) => {
                s.mouse_state = MouseState::Dragging {
                    last_drag_row: mouse_event.row,
                };
                vec![Action::MouseClick(mouse_event.column, mouse_event.row)]
            }

            (_, MouseEventKind::Up(MouseButton::Left)) => {
                s.mouse_state = MouseState::NotDragging;
                Vec::new()
            }

            (true, MouseEventKind::Drag(MouseButton::Left)) => {
                let MouseState::Dragging { last_drag_row, .. } = &mut s.mouse_state else {
                    return Vec::new();
                };

                if mouse_event.row > *last_drag_row {
                    let rows = mouse_event.row.saturating_sub(*last_drag_row);
                    *last_drag_row = mouse_event.row;
                    vec![Action::MouseDragDown; rows as usize]
                } else if mouse_event.row < *last_drag_row {
                    let rows = last_drag_row.saturating_sub(mouse_event.row);
                    *last_drag_row = mouse_event.row;
                    vec![Action::MouseDragUp; rows as usize]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }
}

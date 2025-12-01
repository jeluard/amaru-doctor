use crate::{
    app_state::AppState,
    components::root::RootComponent,
    states::{Action, ComponentId},
    update::Update,
};
use crossterm::event::MouseEventKind;

pub struct MouseScrollUpdate;
impl Update for MouseScrollUpdate {
    fn update(&self, action: &Action, s: &mut AppState, _root: &mut RootComponent) -> Vec<Action> {
        let Action::MouseEvent(mouse_event) = action else {
            return Vec::new();
        };

        if mouse_event.kind != MouseEventKind::ScrollUp
            && mouse_event.kind != MouseEventKind::ScrollDown
        {
            return Vec::new();
        }

        let Some((component_id, _rect)) = s
            .layout_model
            .find_hovered_component(mouse_event.column, mouse_event.row)
        else {
            return Vec::new();
        };

        // If it's a scrollable area, emit Scroll Action
        match component_id {
            // Ledger Options
            ComponentId::LedgerBrowseOptions | ComponentId::LedgerSearchOptions => {
                match mouse_event.kind {
                    MouseEventKind::ScrollUp => vec![Action::ScrollUp],
                    MouseEventKind::ScrollDown => vec![Action::ScrollDown],
                    _ => Vec::new(),
                }
            }
            // Lists
            ComponentId::LedgerAccountsList
            | ComponentId::LedgerBlockIssuersList
            | ComponentId::LedgerDRepsList
            | ComponentId::LedgerPoolsList
            | ComponentId::LedgerProposalsList
            | ComponentId::LedgerUtxosList
            | ComponentId::LedgerUtxosByAddrList
            | ComponentId::OtelTraceList => match mouse_event.kind {
                MouseEventKind::ScrollUp => vec![Action::ScrollUp],
                MouseEventKind::ScrollDown => vec![Action::ScrollDown],
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    }
}

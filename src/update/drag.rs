use crate::{
    app_state::AppState,
    states::{Action, InspectOption, LedgerBrowse, LedgerMode, WidgetSlot},
    update::Update,
};
use strum::Display;

#[derive(Display, Debug, Clone, Copy)]
pub enum DragDirection {
    Up,
    Down,
}

pub struct DragUpdate;

impl Update for DragUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Some(direction) = (match action {
            Action::MouseDragDown => Some(DragDirection::Down),
            Action::MouseDragUp => Some(DragDirection::Up),
            _ => None,
        }) else {
            return Vec::new();
        };

        if s.layout_model.is_focused(WidgetSlot::List)
            && s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
        {
            scroll_ledger_list(s, direction);
        }

        Vec::new()
    }
}

/// Scrolls the currently active list within the ledger view.
fn scroll_ledger_list(s: &mut AppState, direction: DragDirection) {
    if let Some(browse_option) = s.get_ledger_browse_options().model_view.selected_item() {
        match browse_option {
            LedgerBrowse::Accounts => match direction {
                DragDirection::Up => s.get_accounts_list_mut().model_view.advance_window(),
                DragDirection::Down => s.get_accounts_list_mut().model_view.retreat_window(),
            },
            LedgerBrowse::BlockIssuers => match direction {
                DragDirection::Up => s.get_block_issuers_list_mut().model_view.advance_window(),
                DragDirection::Down => s.get_block_issuers_list_mut().model_view.retreat_window(),
            },
            LedgerBrowse::DReps => match direction {
                DragDirection::Up => s.get_dreps_list_mut().model_view.advance_window(),
                DragDirection::Down => s.get_dreps_list_mut().model_view.retreat_window(),
            },
            LedgerBrowse::Pools => match direction {
                DragDirection::Up => s.get_pools_list_mut().model_view.advance_window(),
                DragDirection::Down => s.get_pools_list_mut().model_view.retreat_window(),
            },
            LedgerBrowse::Proposals => match direction {
                DragDirection::Up => s.get_proposals_list_mut().model_view.advance_window(),
                DragDirection::Down => s.get_proposals_list_mut().model_view.retreat_window(),
            },
            LedgerBrowse::Utxos => match direction {
                DragDirection::Up => s.get_utxos_list_mut().model_view.advance_window(),
                DragDirection::Down => s.get_utxos_list_mut().model_view.retreat_window(),
            },
        }
    }
}

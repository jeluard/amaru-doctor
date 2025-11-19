use crate::{
    app_state::AppState,
    components::list::ListModel,
    states::{Action, ComponentId, InspectOption, LedgerBrowse, LedgerMode},
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

        // Check if we are in the correct mode (Ledger -> Browse)
        if s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
        {
            // Get the currently selected browse option (e.g. Accounts, Pools)
            // We use .model instead of .model_view because ListComponent wraps it in `pub model`
            if let Some(browse_option) = s.get_ledger_browse_options().model.selected_item() {
                // Map the option to the specific ComponentId
                let target_id = match browse_option {
                    LedgerBrowse::Accounts => ComponentId::LedgerAccountsList,
                    LedgerBrowse::BlockIssuers => ComponentId::LedgerBlockIssuersList,
                    LedgerBrowse::DReps => ComponentId::LedgerDRepsList,
                    LedgerBrowse::Pools => ComponentId::LedgerPoolsList,
                    LedgerBrowse::Proposals => ComponentId::LedgerProposalsList,
                    LedgerBrowse::Utxos => ComponentId::LedgerUtxosList,
                };

                // Check if that specific component is focused
                if s.layout_model.is_focused(target_id) {
                    scroll_ledger_list(s, direction);
                }
            }
        } else if s.get_ledger_mode_tabs().selected() == LedgerMode::Search {
            // Check if the Search List is focused
            if s.layout_model
                .is_focused(ComponentId::LedgerUtxosByAddrList)
                && let Some(model) = s.ledger_mvs.utxos_by_addr_search.get_current_res_mut()
            {
                match direction {
                    DragDirection::Up => model.advance_window(),
                    DragDirection::Down => model.retreat_window(),
                }
            }
        }

        Vec::new()
    }
}

/// Scrolls the currently active list within the ledger view.
fn scroll_ledger_list(s: &mut AppState, direction: DragDirection) {
    if let Some(browse_option) = s.get_ledger_browse_options().model.selected_item() {
        match browse_option {
            LedgerBrowse::Accounts => match direction {
                DragDirection::Up => s.get_accounts_list_mut().model.advance_window(),
                DragDirection::Down => s.get_accounts_list_mut().model.retreat_window(),
            },
            LedgerBrowse::BlockIssuers => match direction {
                DragDirection::Up => s.get_block_issuers_list_mut().model.advance_window(),
                DragDirection::Down => s.get_block_issuers_list_mut().model.retreat_window(),
            },
            LedgerBrowse::DReps => match direction {
                DragDirection::Up => s.get_dreps_list_mut().model.advance_window(),
                DragDirection::Down => s.get_dreps_list_mut().model.retreat_window(),
            },
            LedgerBrowse::Pools => match direction {
                DragDirection::Up => s.get_pools_list_mut().model.advance_window(),
                DragDirection::Down => s.get_pools_list_mut().model.retreat_window(),
            },
            LedgerBrowse::Proposals => match direction {
                DragDirection::Up => s.get_proposals_list_mut().model.advance_window(),
                DragDirection::Down => s.get_proposals_list_mut().model.retreat_window(),
            },
            LedgerBrowse::Utxos => match direction {
                DragDirection::Up => s.get_utxos_list_mut().model.advance_window(),
                DragDirection::Down => s.get_utxos_list_mut().model.retreat_window(),
            },
        }
    }
}

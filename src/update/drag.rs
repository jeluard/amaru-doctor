use crate::{
    app_state::AppState,
    model::ledger_view::LedgerModelViewState,
    states::{Action, InspectOption, LedgerBrowse, LedgerMode, WidgetSlot},
    update::Update,
};
use strum::Display;
use tracing::debug;

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
            scroll_ledger_list(&mut s.ledger_mvs, direction);
        }

        Vec::new()
    }
}

/// Scrolls the currently active list within the ledger view.
fn scroll_ledger_list(ledger_mvs: &mut LedgerModelViewState, direction: DragDirection) {
    if let Some(browse_option) = ledger_mvs.browse_options.selected_item() {
        debug!(
            "Scrolling ledger list for browse option: {:?}",
            browse_option
        );
        match browse_option {
            LedgerBrowse::Accounts => match direction {
                DragDirection::Up => ledger_mvs.accounts.advance_window(),
                DragDirection::Down => ledger_mvs.accounts.retreat_window(),
            },
            LedgerBrowse::BlockIssuers => match direction {
                DragDirection::Up => ledger_mvs.block_issuers.advance_window(),
                DragDirection::Down => ledger_mvs.block_issuers.retreat_window(),
            },
            LedgerBrowse::DReps => match direction {
                DragDirection::Up => ledger_mvs.dreps.advance_window(),
                DragDirection::Down => ledger_mvs.dreps.retreat_window(),
            },
            LedgerBrowse::Pools => match direction {
                DragDirection::Up => ledger_mvs.pools.advance_window(),
                DragDirection::Down => ledger_mvs.pools.retreat_window(),
            },
            LedgerBrowse::Proposals => match direction {
                DragDirection::Up => ledger_mvs.proposals.advance_window(),
                DragDirection::Down => ledger_mvs.proposals.retreat_window(),
            },
            LedgerBrowse::Utxos => match direction {
                DragDirection::Up => ledger_mvs.utxos.advance_window(),
                DragDirection::Down => ledger_mvs.utxos.retreat_window(),
            },
        }
    }
}

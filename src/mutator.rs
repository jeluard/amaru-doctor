use tracing::trace;

use crate::{
    app_state::AppState,
    states::{Action, WidgetId},
};

pub trait Mutator<T> {
    fn mutate(&self, to_mutate: &mut T);
}

impl Mutator<AppState> for Action {
    fn mutate(&self, app_state: &mut AppState) {
        if !matches!(self, Action::Render | Action::Tick) {
            trace!("Handling action {}", self);
        }
        match self {
            Action::FocusPrev => app_state.slot_focus.borrow_mut().next_back(),
            Action::FocusNext => app_state.slot_focus.borrow_mut().next(),
            Action::ScrollUp => scroll_up(app_state),
            Action::ScrollDown => scroll_down(app_state),
            _ => {}
        }
    }
}

fn scroll_up(app_state: &mut AppState) {
    let widget_id = app_state.get_focused_widget().unwrap();
    match widget_id {
        WidgetId::Empty => {} // Nothing to scroll
        WidgetId::CursorTabs => app_state.tabs.borrow_mut().next_back(),
        WidgetId::ListBrowseOptions => app_state.browse_options.borrow_mut().scroll_up(),
        WidgetId::ListSearchOptions => app_state.search_options.borrow_mut().scroll_up(),
        WidgetId::ListAccounts => app_state.accounts.borrow_mut().scroll_up(),
        WidgetId::ListBlockIssuers => app_state.block_issuers.borrow_mut().scroll_up(),
        WidgetId::ListDReps => app_state.dreps.borrow_mut().scroll_up(),
        WidgetId::ListPools => app_state.pools.borrow_mut().scroll_up(),
        WidgetId::ListProposals => app_state.proposals.borrow_mut().scroll_up(),
        WidgetId::ListUtxos => app_state.utxos.borrow_mut().scroll_up(),
        // TODO: Need to add a scroll offset state
        WidgetId::DetailsAccount => todo!(),
        WidgetId::DetailsBlockIssuer => todo!(),
        WidgetId::DetailsDRep => todo!(),
        WidgetId::DetailsPool => todo!(),
        WidgetId::DetailsProposal => todo!(),
        WidgetId::DetailsUtxo => todo!(),
    }
}

fn scroll_down(app_state: &mut AppState) {
    let widget_id = app_state.get_focused_widget().unwrap();
    match widget_id {
        WidgetId::Empty => {} // Nothing to scroll
        WidgetId::CursorTabs => app_state.tabs.borrow_mut().next_back(),
        WidgetId::ListBrowseOptions => app_state.browse_options.borrow_mut().scroll_down(),
        WidgetId::ListSearchOptions => app_state.search_options.borrow_mut().scroll_down(),
        WidgetId::ListAccounts => app_state.accounts.borrow_mut().scroll_down(),
        WidgetId::ListBlockIssuers => app_state.block_issuers.borrow_mut().scroll_down(),
        WidgetId::ListDReps => app_state.dreps.borrow_mut().scroll_down(),
        WidgetId::ListPools => app_state.pools.borrow_mut().scroll_down(),
        WidgetId::ListProposals => app_state.proposals.borrow_mut().scroll_down(),
        WidgetId::ListUtxos => app_state.utxos.borrow_mut().scroll_down(),
        WidgetId::DetailsAccount => todo!(),
        WidgetId::DetailsBlockIssuer => todo!(),
        WidgetId::DetailsDRep => todo!(),
        WidgetId::DetailsPool => todo!(),
        WidgetId::DetailsProposal => todo!(),
        WidgetId::DetailsUtxo => todo!(),
    }
}

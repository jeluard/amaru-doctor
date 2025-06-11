use tracing::trace;

use crate::{
    app_state::AppState,
    shared::Shared,
    states::{Action, WidgetId},
};

pub trait Mutator<T> {
    fn mutate(&self, to_mutate: Shared<T>);
}

impl Mutator<AppState> for Action {
    fn mutate(&self, app_state: Shared<AppState>) {
        if !matches!(self, Action::Render | Action::Tick) {
            trace!("Handling action {}", self);
        }
        match self {
            Action::FocusPrev => app_state.borrow().slot_focus.borrow_mut().next_back(),
            Action::FocusNext => app_state.borrow().slot_focus.borrow_mut().next(),
            Action::ScrollUp => scroll_up(app_state),
            Action::ScrollDown => scroll_down(app_state),
            _ => {}
        }
    }
}

fn scroll_up(app_state: Shared<AppState>) {
    let widget_id = app_state.borrow().get_focused_widget().unwrap();
    match widget_id {
        WidgetId::Empty => {} // Nothing to scroll
        WidgetId::CursorTabs => app_state.borrow().tabs.borrow_mut().next_back(),
        WidgetId::ListBrowseOptions => app_state.borrow().browse_options.borrow_mut().scroll_up(),
        WidgetId::ListSearchOptions => app_state.borrow().search_options.borrow_mut().scroll_up(),
        WidgetId::ListAccounts => app_state.borrow().accounts.borrow_mut().scroll_up(),
        WidgetId::ListBlockIssuers => app_state.borrow().block_issuers.borrow_mut().scroll_up(),
        WidgetId::ListDReps => app_state.borrow().dreps.borrow_mut().scroll_up(),
        WidgetId::ListPools => app_state.borrow().pools.borrow_mut().scroll_up(),
        WidgetId::ListProposals => app_state.borrow().proposals.borrow_mut().scroll_up(),
        WidgetId::ListUtxos => app_state.borrow().utxos.borrow_mut().scroll_up(),
        // TODO: Need to add a scroll offset state
        WidgetId::DetailAccount => todo!(),
        WidgetId::DetailBlockIssuer => todo!(),
        WidgetId::DetailDRep => todo!(),
        WidgetId::DetailPool => todo!(),
        WidgetId::DetailProposal => todo!(),
        WidgetId::DetailUtxo => todo!(),
    }
}

fn scroll_down(app_state: Shared<AppState>) {
    let widget_id = app_state.borrow().get_focused_widget().unwrap();
    match widget_id {
        WidgetId::Empty => {} // Nothing to scroll
        WidgetId::CursorTabs => app_state.borrow().tabs.borrow_mut().next_back(),
        WidgetId::ListBrowseOptions => app_state.borrow().browse_options.borrow_mut().scroll_down(),
        WidgetId::ListSearchOptions => app_state.borrow().search_options.borrow_mut().scroll_down(),
        WidgetId::ListAccounts => app_state.borrow().accounts.borrow_mut().scroll_down(),
        WidgetId::ListBlockIssuers => app_state.borrow().block_issuers.borrow_mut().scroll_down(),
        WidgetId::ListDReps => app_state.borrow().dreps.borrow_mut().scroll_down(),
        WidgetId::ListPools => app_state.borrow().pools.borrow_mut().scroll_down(),
        WidgetId::ListProposals => app_state.borrow().proposals.borrow_mut().scroll_down(),
        WidgetId::ListUtxos => app_state.borrow().utxos.borrow_mut().scroll_down(),
        WidgetId::DetailAccount => todo!(),
        WidgetId::DetailBlockIssuer => todo!(),
        WidgetId::DetailDRep => todo!(),
        WidgetId::DetailPool => todo!(),
        WidgetId::DetailProposal => todo!(),
        WidgetId::DetailUtxo => todo!(),
    }
}

// pub trait Scrollable {
//     fn scroll_up(&mut self);
//     fn scroll_down(&mut self);
//     fn iter(&self) -> Box<dyn Iterator<Item = &dyn Any> + '_>;
//     fn index(&self) -> usize;
//     fn iter_idx(&self) -> (Box<dyn Iterator<Item = &dyn Any> + '_>, usize);
// }

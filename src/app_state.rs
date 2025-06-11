use crate::{
    model::{cursor::Cursor, window::WindowState},
    shared::{Shared, shared},
    states::{BrowseOptions, SearchOptions, Tab, WidgetId, WidgetSlot},
    store::{
        owned_iter::{
            OwnedAccountsIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter,
            OwnedProposalIter, OwnedUtxoIter,
        },
        rocks_db_switch::RocksDBSwitch,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
};
use std::sync::Arc;
use strum::IntoEnumIterator;

/// Holds ALL the state for the app. Does not self-mutate.
/// Does provide readers / helper calcs
pub struct AppState {
    pub slot_focus: Shared<Cursor<WidgetSlot>>,
    pub tabs: Shared<Cursor<Tab>>,
    // Don't put these in Map, however tempting--it will cause pain with generics and ultimately increases complexity
    pub browse_options: Shared<WindowState<BrowseOptions>>,
    pub search_options: Shared<WindowState<SearchOptions>>,
    pub accounts: Shared<WindowState<AccountItem>>,
    pub block_issuers: Shared<WindowState<BlockIssuerItem>>,
    pub dreps: Shared<WindowState<DRepItem>>,
    pub pools: Shared<WindowState<PoolItem>>,
    pub proposals: Shared<WindowState<ProposalItem>>,
    pub utxos: Shared<WindowState<UtxoItem>>,
}

impl AppState {
    pub fn new(db: Arc<RocksDBSwitch>) -> Self {
        Self {
            slot_focus: shared(Cursor::new(WidgetSlot::iter().collect())),
            tabs: shared(Cursor::new(Tab::iter().collect())),
            browse_options: shared(WindowState::new(Box::new(BrowseOptions::iter()))),
            search_options: shared(WindowState::new(Box::new(SearchOptions::iter()))),
            accounts: shared(WindowState::new(Box::new(OwnedAccountsIter::new(
                db.clone(),
            )))),
            block_issuers: shared(WindowState::new(Box::new(OwnedBlockIssuerIter::new(
                db.clone(),
            )))),
            dreps: shared(WindowState::new(Box::new(OwnedDRepIter::new(db.clone())))),
            pools: shared(WindowState::new(Box::new(OwnedPoolIter::new(db.clone())))),
            proposals: shared(WindowState::new(Box::new(OwnedProposalIter::new(
                db.clone(),
            )))),
            utxos: shared(WindowState::new(Box::new(OwnedUtxoIter::new(db.clone())))),
        }
    }

    pub fn is_widget_focused(&self, widget_id: WidgetId) -> bool {
        self.get_focused_widget() == Some(widget_id)
    }

    pub fn get_focused_widget(&self) -> Option<WidgetId> {
        self.slot_focus
            .borrow()
            .current()
            .and_then(|s| self.get_selected_widget(s.clone()))
    }

    pub fn get_selected_widget(&self, slot: WidgetSlot) -> Option<WidgetId> {
        match slot {
            WidgetSlot::Nav => Some(WidgetId::CursorTabs),
            WidgetSlot::NavType => match self.tabs.borrow().current() {
                Some(Tab::Browse) => Some(WidgetId::ListBrowseOptions),
                Some(Tab::Search) => Some(WidgetId::ListSearchOptions),
                None => None,
            },
            WidgetSlot::List => match self.browse_options.borrow().selected() {
                Some(BrowseOptions::Accounts) => Some(WidgetId::ListAccounts),
                Some(BrowseOptions::BlockIssuers) => Some(WidgetId::ListBlockIssuers),
                Some(BrowseOptions::DReps) => Some(WidgetId::ListDReps),
                Some(BrowseOptions::Pools) => Some(WidgetId::ListPools),
                Some(BrowseOptions::Proposals) => Some(WidgetId::ListProposals),
                Some(BrowseOptions::Utxos) => Some(WidgetId::ListUtxos),
                None => None,
            },
            WidgetSlot::Details => match self.browse_options.borrow().selected() {
                Some(BrowseOptions::Accounts) => Some(WidgetId::DetailsAccount),
                Some(BrowseOptions::BlockIssuers) => Some(WidgetId::DetailsBlockIssuer),
                Some(BrowseOptions::DReps) => Some(WidgetId::DetailsDRep),
                Some(BrowseOptions::Pools) => Some(WidgetId::DetailsPool),
                Some(BrowseOptions::Proposals) => Some(WidgetId::DetailsProposal),
                Some(BrowseOptions::Utxos) => Some(WidgetId::DetailsUtxo),
                None => None,
            },
        }
    }
}

use crate::{
    model::{cursor::Cursor, window::WindowState},
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
use std::{cell::RefCell, sync::Arc};
use strum::IntoEnumIterator;

/// Holds ALL the state for the app. Does not self-mutate.
/// Does provide readers / helper calcs
pub struct AppState {
    pub slot_focus: RefCell<Cursor<WidgetSlot>>,
    pub tabs: RefCell<Cursor<Tab>>,
    // Don't put these in Map, however tempting--it will cause pain with generics and ultimately increases complexity
    pub browse_options: RefCell<WindowState<BrowseOptions>>,
    pub search_options: RefCell<WindowState<SearchOptions>>,
    pub accounts: RefCell<WindowState<AccountItem>>,
    pub block_issuers: RefCell<WindowState<BlockIssuerItem>>,
    pub dreps: RefCell<WindowState<DRepItem>>,
    pub pools: RefCell<WindowState<PoolItem>>,
    pub proposals: RefCell<WindowState<ProposalItem>>,
    pub utxos: RefCell<WindowState<UtxoItem>>,
}

impl AppState {
    pub fn new(db: Arc<RocksDBSwitch>) -> Self {
        Self {
            slot_focus: RefCell::new(Cursor::new(WidgetSlot::iter().collect())),
            tabs: RefCell::new(Cursor::new(Tab::iter().collect())),
            browse_options: RefCell::new(WindowState::new(Box::new(BrowseOptions::iter()))),
            search_options: RefCell::new(WindowState::new(Box::new(SearchOptions::iter()))),
            accounts: RefCell::new(WindowState::new(Box::new(OwnedAccountsIter::new(
                db.clone(),
            )))),
            block_issuers: RefCell::new(WindowState::new(Box::new(OwnedBlockIssuerIter::new(
                db.clone(),
            )))),
            dreps: RefCell::new(WindowState::new(Box::new(OwnedDRepIter::new(db.clone())))),
            pools: RefCell::new(WindowState::new(Box::new(OwnedPoolIter::new(db.clone())))),
            proposals: RefCell::new(WindowState::new(Box::new(OwnedProposalIter::new(
                db.clone(),
            )))),
            utxos: RefCell::new(WindowState::new(Box::new(OwnedUtxoIter::new(db.clone())))),
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

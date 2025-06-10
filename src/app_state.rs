use crate::{
    components::r#static::{entity_types::Entity, search_types::SearchOptions},
    cursor::Cursor,
    shared::{Shared, shared},
    states::{EntityOptions, Nav, Slot, SlotSelection},
    store::{
        owned_iter::{
            OwnedAccountsIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter,
            OwnedProposalIter, OwnedUtxoIter,
        },
        rocks_db_switch::RocksDBSwitch,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
    window::WindowState,
};
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct AppState {
    pub focus: Shared<Cursor<Slot>>,
    pub nav: Shared<Cursor<Nav>>,
    pub entity_list: Shared<WindowState<EntityOptions>>,
    pub search_list: Shared<WindowState<SearchOptions>>,
    pub account_list: Shared<WindowState<AccountItem>>,
    pub block_issuer_list: Shared<WindowState<BlockIssuerItem>>,
    pub drep_list: Shared<WindowState<DRepItem>>,
    pub pool_list: Shared<WindowState<PoolItem>>,
    pub proposal_list: Shared<WindowState<ProposalItem>>,
    pub utxo_list: Shared<WindowState<UtxoItem>>,
    pub layout_rev: usize,
}

impl AppState {
    pub fn new(db: Arc<RocksDBSwitch>) -> Self {
        Self {
            focus: shared(Cursor::new(Slot::iter().collect())),
            nav: shared(Cursor::new(Nav::iter().collect())),
            entity_list: shared(WindowState::new(Box::new(EntityOptions::iter()))),
            search_list: shared(WindowState::new(Box::new(SearchOptions::iter()))),
            account_list: shared(WindowState::new(Box::new(OwnedAccountsIter::new(
                db.clone(),
            )))),
            block_issuer_list: shared(WindowState::new(Box::new(OwnedBlockIssuerIter::new(
                db.clone(),
            )))),
            drep_list: shared(WindowState::new(Box::new(OwnedDRepIter::new(db.clone())))),
            pool_list: shared(WindowState::new(Box::new(OwnedPoolIter::new(db.clone())))),
            proposal_list: shared(WindowState::new(Box::new(OwnedProposalIter::new(
                db.clone(),
            )))),
            utxo_list: shared(WindowState::new(Box::new(OwnedUtxoIter::new(db.clone())))),
            layout_rev: 0,
        }
    }

    pub fn get_slot_selection(&self, slot: Slot) -> Option<SlotSelection> {
        match slot {
            Slot::Nav => Some(SlotSelection::Nav),
            Slot::NavType => match self.nav.borrow().current() {
                Some(Nav::Browse) => Some(SlotSelection::NavTypeBrowse),
                Some(Nav::Search) => Some(SlotSelection::NavTypeSearch),
                None => None,
            },
            Slot::List => match self.entity_list.borrow().selected() {
                Some(EntityOptions::Accounts) => Some(SlotSelection::BrowseAccounts),
                Some(EntityOptions::BlockIssuers) => Some(SlotSelection::BrowseBlockIssuers),
                Some(EntityOptions::DReps) => Some(SlotSelection::BrowseDReps),
                Some(EntityOptions::Pools) => Some(SlotSelection::BrowsePools),
                Some(EntityOptions::Proposals) => Some(SlotSelection::BrowseProposals),
                Some(EntityOptions::Utxos) => Some(SlotSelection::BrowseUtxos),
                None => None,
            },
            Slot::Details => match self.entity_list.borrow().selected() {
                Some(EntityOptions::Accounts) => Some(SlotSelection::DetailAccount),
                Some(EntityOptions::BlockIssuers) => Some(SlotSelection::DetailBlockIssuer),
                Some(EntityOptions::DReps) => Some(SlotSelection::DetailDRep),
                Some(EntityOptions::Pools) => Some(SlotSelection::DetailPool),
                Some(EntityOptions::Proposals) => Some(SlotSelection::DetailProposal),
                Some(EntityOptions::Utxos) => Some(SlotSelection::DetailUtxo),
                None => None,
            },
        }
    }

    pub fn get_focused(&self) -> Option<SlotSelection> {
        self.focus
            .borrow()
            .current()
            .and_then(|s| self.get_slot_selection(s.clone()))
    }

    pub fn shift_focus_prev(&mut self) {
        self.focus.borrow_mut().next_back();
    }

    pub fn shift_focus_next(&mut self) {
        self.focus.borrow_mut().next();
    }
}

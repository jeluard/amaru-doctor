use crate::{
    model::{cursor::Cursor, window::WindowState},
    states::{BrowseOption, SearchOption, TabOption, WidgetSlot},
    store::{
        owned_iter::{
            OwnedAccountIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter,
            OwnedProposalIter, OwnedUtxoIter,
        },
        rocks_db_switch::RocksDBSwitch,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
};
use amaru_kernel::Address;
use color_eyre::Result;
use std::{collections::HashMap, sync::Arc};
use strum::IntoEnumIterator;

/// Holds ALL the app's state. Does not self-mutate.
pub struct AppState {
    pub ledger_path: String,
    pub db: Arc<RocksDBSwitch>,
    pub slot_focus: Cursor<WidgetSlot>,
    pub tabs: Cursor<TabOption>,
    // Don't put these in Map, however tempting--it will cause pain with generics and ultimately increases complexity
    pub options_window_size: usize,
    pub browse_options: WindowState<BrowseOption>,
    pub search_options: WindowState<SearchOption>,

    pub list_window_size: usize,
    pub accounts: WindowState<AccountItem>,
    pub block_issuers: WindowState<BlockIssuerItem>,
    pub dreps: WindowState<DRepItem>,
    pub pools: WindowState<PoolItem>,
    pub proposals: WindowState<ProposalItem>,
    pub utxos: WindowState<UtxoItem>,

    // TODO: Encapsulate search state
    pub search_query_bldr: String,
    pub search_query_addr: Option<Address>,
    pub utxos_by_addr_search_res: HashMap<Address, WindowState<UtxoItem>>,
}

impl AppState {
    pub fn new(ledger_path: String, db: Arc<RocksDBSwitch>) -> Result<Self> {
        Ok(Self {
            ledger_path,
            db: db.clone(),
            slot_focus: Cursor::new(vec![
                WidgetSlot::Nav,
                WidgetSlot::Options,
                WidgetSlot::List,
                WidgetSlot::SearchBar,
                WidgetSlot::Details,
            ])?,
            tabs: Cursor::new(TabOption::iter().collect())?,
            options_window_size: 0,
            browse_options: WindowState::from_iter(BrowseOption::iter()),
            search_options: WindowState::from_iter(SearchOption::iter()),
            list_window_size: 0,
            accounts: WindowState::from_iter(OwnedAccountIter::new(db.clone())),
            block_issuers: WindowState::from_iter(OwnedBlockIssuerIter::new(db.clone())),
            dreps: WindowState::from_iter(OwnedDRepIter::new(db.clone())),
            pools: WindowState::from_iter(OwnedPoolIter::new(db.clone())),
            proposals: WindowState::from_iter(OwnedProposalIter::new(db.clone())),
            utxos: WindowState::from_iter(OwnedUtxoIter::new(db.clone())),
            search_query_bldr: "".to_owned(),
            search_query_addr: None,
            utxos_by_addr_search_res: HashMap::new(),
        })
    }
}

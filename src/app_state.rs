use crate::{
    model::{cursor::Cursor, window::WindowState},
    states::{BrowseOption, LedgerMode, SearchOption, StoreOption, WidgetSlot},
    store::{
        owned_iter::{
            OwnedAccountIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter,
            OwnedProposalIter, OwnedUtxoIter,
        },
        rocks_db_switch::LedgerDB,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
};
use amaru_consensus::consensus::store::ChainStore;
use amaru_kernel::Address;
use amaru_stores::rocksdb::consensus::RocksDBStore;
use color_eyre::Result;
use std::{collections::HashMap, sync::Arc};
use strum::IntoEnumIterator;

/// Holds ALL the app's state. Does not self-mutate.
pub struct AppState {
    pub ledger_path: String,
    pub ledger_db: Arc<LedgerDB>,

    pub chain_path: String,
    pub chain_db: Arc<RocksDBStore>,

    pub slot_focus: Cursor<WidgetSlot>,

    pub store_option: Cursor<StoreOption>,
    pub ledger_mode: Cursor<LedgerMode>,

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
    pub fn new(
        ledger_path: String,
        ledger_db: LedgerDB,
        chain_path: String,
        chain_db: RocksDBStore,
    ) -> Result<Self> {
        let ledger_db_arc = Arc::new(ledger_db);
        let chain_db_arc = Arc::new(chain_db);

        Ok(Self {
            ledger_path,
            ledger_db: ledger_db_arc.clone(),
            chain_path,
            chain_db: chain_db_arc.clone(),
            slot_focus: Cursor::new(vec![
                WidgetSlot::StoreOption,
                WidgetSlot::LedgerMode,
                WidgetSlot::SearchBar,
                WidgetSlot::Options,
                WidgetSlot::List,
                WidgetSlot::Details,
            ])?,
            store_option: Cursor::new(StoreOption::iter().collect())?,
            ledger_mode: Cursor::new(LedgerMode::iter().collect())?,
            options_window_size: 0,
            browse_options: WindowState::from_iter(BrowseOption::iter()),
            search_options: WindowState::from_iter(SearchOption::iter()),
            list_window_size: 0,
            accounts: WindowState::from_iter(OwnedAccountIter::new(ledger_db_arc.clone())),
            block_issuers: WindowState::from_iter(OwnedBlockIssuerIter::new(ledger_db_arc.clone())),
            dreps: WindowState::from_iter(OwnedDRepIter::new(ledger_db_arc.clone())),
            pools: WindowState::from_iter(OwnedPoolIter::new(ledger_db_arc.clone())),
            proposals: WindowState::from_iter(OwnedProposalIter::new(ledger_db_arc.clone())),
            utxos: WindowState::from_iter(OwnedUtxoIter::new(ledger_db_arc.clone())),
            search_query_bldr: "".to_owned(),
            search_query_addr: None,
            utxos_by_addr_search_res: HashMap::new(),
        })
    }
}

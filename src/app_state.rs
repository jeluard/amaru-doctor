use crate::{
    model::{cursor::Cursor, window::WindowState},
    states::{BrowseOption, LedgerMode, LedgerSearchOption, StoreOption, WidgetSlot},
    store::{
        owned_iter::{
            OwnedAccountIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter,
            OwnedProposalIter, OwnedUtxoIter,
        },
        rocks_db_switch::LedgerDB,
    },
    types::chain::ChainSearchOption,
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
};
use amaru_consensus::Nonces;
use amaru_kernel::{Address, Header, RawBlock};
use amaru_ledger::store::StoreError;
use amaru_stores::rocksdb::consensus::RocksDBStore;
use color_eyre::Result;
use pallas_crypto::hash::Hash;
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

    pub options_window_size: usize,
    pub ledger_browse_options: WindowState<BrowseOption>,
    pub ledger_search_options: WindowState<LedgerSearchOption>,

    pub list_window_size: usize,
    pub accounts: WindowState<AccountItem>,
    pub block_issuers: WindowState<BlockIssuerItem>,
    pub dreps: WindowState<DRepItem>,
    pub pools: WindowState<PoolItem>,
    pub proposals: WindowState<ProposalItem>,
    pub utxos: WindowState<UtxoItem>,

    // TODO: Encapsulate search state
    pub ledger_search_query_bldr: String,
    pub ledger_search_query_addr: Option<Address>,
    pub utxos_by_addr_search_res: HashMap<Address, WindowState<UtxoItem>>,

    pub chain_search_options: WindowState<ChainSearchOption>,

    pub chain_search_query_bldr: String,
    pub chain_search_query_hash: Option<Hash<32>>,
    pub headers_by_hash_search_res: HashMap<Hash<32>, Option<Header>>,
    pub block_by_hash_search_res: HashMap<Hash<32>, Result<RawBlock, StoreError>>,
    pub nonces_by_hash_search_res: HashMap<Hash<32>, Option<Nonces>>,
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
            ledger_browse_options: WindowState::from_iter(BrowseOption::iter()),
            ledger_search_options: WindowState::from_iter(LedgerSearchOption::iter()),
            list_window_size: 0,
            accounts: WindowState::from_iter(OwnedAccountIter::new(ledger_db_arc.clone())),
            block_issuers: WindowState::from_iter(OwnedBlockIssuerIter::new(ledger_db_arc.clone())),
            dreps: WindowState::from_iter(OwnedDRepIter::new(ledger_db_arc.clone())),
            pools: WindowState::from_iter(OwnedPoolIter::new(ledger_db_arc.clone())),
            proposals: WindowState::from_iter(OwnedProposalIter::new(ledger_db_arc.clone())),
            utxos: WindowState::from_iter(OwnedUtxoIter::new(ledger_db_arc.clone())),
            ledger_search_query_bldr: "".to_owned(),
            ledger_search_query_addr: None,
            utxos_by_addr_search_res: HashMap::new(),
            chain_search_options: WindowState::from_iter(ChainSearchOption::iter()),
            chain_search_query_bldr: "".to_owned(),
            chain_search_query_hash: None,
            headers_by_hash_search_res: HashMap::new(),
            block_by_hash_search_res: HashMap::new(),
            nonces_by_hash_search_res: HashMap::new(),
        })
    }
}

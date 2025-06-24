use crate::{
    controller::SlotLayout,
    model::{cursor::Cursor, window::WindowState},
    states::{BrowseOption, LedgerMode, LedgerSearchOption, StoreOption, WidgetSlot},
    store::{
        owned_iter::{
            OwnedAccountIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter,
            OwnedProposalIter, OwnedUtxoIter,
        },
        rocks_db_switch::LedgerDB,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
    update::search::SearchState,
};
use amaru_consensus::Nonces;
use amaru_kernel::{Address, Hash, Header, RawBlock};
use amaru_stores::rocksdb::consensus::RocksDBStore;
use color_eyre::Result;
use ratatui::layout::Rect;
use std::sync::Arc;
use strum::IntoEnumIterator;

/// Holds ALL the app's state. Does not self-mutate.
pub struct AppState {
    pub ledger_path: String,
    pub ledger_db: Arc<LedgerDB>,

    // TODO: Add this in a header message
    // pub chain_path: String,
    pub chain_db: Arc<RocksDBStore>,

    pub frame_area: Rect,
    pub layout: SlotLayout,
    pub slot_focus: WidgetSlot,

    pub store_option: Cursor<StoreOption>,
    pub ledger_mode: Cursor<LedgerMode>,

    pub options_window_size: usize,

    pub ledger_browse_options: WindowState<BrowseOption>,
    pub list_window_size: usize,
    pub accounts: WindowState<AccountItem>,
    pub block_issuers: WindowState<BlockIssuerItem>,
    pub dreps: WindowState<DRepItem>,
    pub pools: WindowState<PoolItem>,
    pub proposals: WindowState<ProposalItem>,
    pub utxos: WindowState<UtxoItem>,

    pub ledger_search_options: WindowState<LedgerSearchOption>,
    pub utxos_by_addr_search: SearchState<Address, WindowState<UtxoItem>>,

    pub chain_search: SearchState<Hash<32>, Option<(Header, RawBlock, Nonces)>>,
}

impl AppState {
    pub fn new(
        ledger_path: String,
        ledger_db: LedgerDB,
        _chain_path: String,
        chain_db: RocksDBStore,
    ) -> Result<Self> {
        let ledger_db_arc = Arc::new(ledger_db);
        let chain_db_arc = Arc::new(chain_db);
        Ok(Self {
            ledger_path,
            ledger_db: ledger_db_arc.clone(),
            // chain_path,
            chain_db: chain_db_arc.clone(),
            frame_area: Rect::default(),
            layout: SlotLayout::default(),
            slot_focus: WidgetSlot::StoreOption,
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
            utxos_by_addr_search: SearchState::default(),
            chain_search: SearchState::default(),
        })
    }
}

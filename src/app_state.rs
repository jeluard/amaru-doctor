use crate::{
    controller::SlotLayout,
    model::{chain_view::ChainViewState, cursor::Cursor, ledger_view::LedgerViewState},
    otel::TraceCollector,
    states::{InspectOption, LedgerMode, WidgetSlot},
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use ratatui::layout::Rect;
use std::sync::Arc;

/// Holds ALL the app's state. Does not self-mutate.
pub struct AppState {
    pub ledger_db: Arc<ReadOnlyRocksDB>,
    pub chain_db: Arc<ReadOnlyChainDB>,

    pub frame_area: Rect,
    pub layout: SlotLayout,
    pub slot_focus: WidgetSlot,
    pub inspect_option: Cursor<InspectOption>,
    pub ledger_mode: Cursor<LedgerMode>,

    pub ledger_view: LedgerViewState,
    pub chain_view: ChainViewState,

    pub collector: Arc<TraceCollector>,
}

impl AppState {
    pub fn new(
        ledger_db: ReadOnlyRocksDB,
        chain_db: ReadOnlyChainDB,
        collector: Arc<TraceCollector>,
    ) -> Result<Self> {
        let ledger_db_arc = Arc::new(ledger_db);
        let chain_db_arc = Arc::new(chain_db);
        Ok(Self {
            ledger_db: ledger_db_arc.clone(),
            chain_db: chain_db_arc.clone(),
            frame_area: Rect::default(),
            layout: SlotLayout::default(),
            slot_focus: WidgetSlot::InspectOption,
            inspect_option: Cursor::default(),
            ledger_mode: Cursor::default(),
            ledger_view: LedgerViewState::new(ledger_db_arc),
            chain_view: ChainViewState::new(),
            collector,
        })
    }
}

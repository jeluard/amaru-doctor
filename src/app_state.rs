use crate::{
    controller::SlotLayout,
    model::{
        button::InputEvent, chain_view::ChainViewState, cursor::Cursor,
        ledger_view::LedgerViewState, otel_view::OtelViewState, prom_metrics::PromMetricsViewState,
    },
    otel::graph::TraceGraph,
    prometheus::model::NodeMetrics,
    states::{InspectOption, LedgerMode, WidgetSlot},
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use arc_swap::ArcSwap;
use ratatui::layout::Rect;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

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

    pub otel_view: OtelViewState,
    pub prom_metrics: PromMetricsViewState,

    pub button_events: std::sync::mpsc::Receiver<InputEvent>,
}

impl AppState {
    pub fn new(
        ledger_db: ReadOnlyRocksDB,
        chain_db: ReadOnlyChainDB,
        trace_graph: Arc<ArcSwap<TraceGraph>>,
        prom_metrics: Receiver<NodeMetrics>,
        button_events: std::sync::mpsc::Receiver<InputEvent>,
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
            chain_view: ChainViewState::default(),
            otel_view: OtelViewState::new(trace_graph),
            prom_metrics: PromMetricsViewState::new(prom_metrics),
            button_events,
        })
    }
}

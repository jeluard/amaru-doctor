use crate::{
    ScreenMode,
    model::{
        button::InputEvent, chain_view::ChainViewState, layout::LayoutModel,
        ledger_view::LedgerModelViewState, otel_view::OtelViewState,
        prom_metrics::PromMetricsViewState,
    },
    otel::graph::TraceGraph,
    prometheus::model::NodeMetrics,
    states::{InspectOption, LedgerMode, WidgetSlot},
    update::mouse::MouseState,
    view::tabs::TabsState,
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use arc_swap::ArcSwap;
use ratatui::layout::Rect;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

/// Holds ALL the app's state. Does not self-mutate.
pub struct AppState {
    pub screen_mode: ScreenMode,

    pub ledger_db: Arc<ReadOnlyRocksDB>,
    pub chain_db: Arc<ReadOnlyChainDB>,

    pub frame_area: Rect,
    pub layout_model: LayoutModel,

    pub inspect_tabs: TabsState<InspectOption>,
    pub ledger_tabs: TabsState<LedgerMode>,

    pub ledger_mvs: LedgerModelViewState,
    pub chain_view: ChainViewState,

    pub otel_view: OtelViewState,
    pub prom_metrics: PromMetricsViewState,

    pub button_events: std::sync::mpsc::Receiver<InputEvent>,

    pub mouse_state: MouseState,
}

impl AppState {
    pub fn new(
        ledger_db: ReadOnlyRocksDB,
        chain_db: ReadOnlyChainDB,
        trace_graph: Arc<ArcSwap<TraceGraph>>,
        prom_metrics: Receiver<NodeMetrics>,
        button_events: std::sync::mpsc::Receiver<InputEvent>,
        frame_area: Rect,
        screen_mode: ScreenMode,
    ) -> Result<Self> {
        let ledger_db_arc = Arc::new(ledger_db);
        let chain_db_arc = Arc::new(chain_db);
        let layout_model = LayoutModel::new(
            screen_mode,
            InspectOption::default(),
            LedgerMode::default(),
            frame_area,
        );
        let options_height: usize = layout_model
            .get_layout()
            .get(&WidgetSlot::LedgerOptions)
            .ok_or(anyhow::anyhow!("No rect for LedgerOptions"))?
            .height
            .into();

        let list_height: usize = layout_model
            .get_layout()
            .get(&WidgetSlot::List)
            .ok_or(anyhow::anyhow!("No rect for List"))?
            .height
            .into();
        Ok(Self {
            screen_mode,
            ledger_db: ledger_db_arc.clone(),
            chain_db: chain_db_arc.clone(),
            frame_area: Rect::default(),
            layout_model,
            inspect_tabs: TabsState::new("Inspect Options")?,
            ledger_tabs: TabsState::new("Ledger Options")?,
            ledger_mvs: LedgerModelViewState::new(ledger_db_arc, options_height, list_height),
            chain_view: ChainViewState::default(),
            otel_view: OtelViewState::new(trace_graph),
            prom_metrics: PromMetricsViewState::new(prom_metrics),
            button_events,
            mouse_state: MouseState::default(),
        })
    }
}

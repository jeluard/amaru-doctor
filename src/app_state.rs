use crate::{
    ScreenMode,
    components::{Component, tabs::TabsComponent},
    model::{
        button::InputEvent, chain_view::ChainViewState, layout::LayoutModel,
        ledger_view::LedgerModelViewState, otel_view::OtelViewState,
        prom_metrics::PromMetricsViewState,
    },
    otel::graph::TraceGraph,
    prometheus::model::NodeMetrics,
    states::{ComponentId, InspectOption, LedgerMode, WidgetSlot},
    update::mouse::MouseState,
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use arc_swap::ArcSwap;
use ratatui::layout::Rect;
use std::{
    collections::HashMap,
    sync::{Arc, mpsc},
};
use tokio::sync::mpsc::Receiver;

/// Holds ALL the app's state. Does not self-mutate.
pub struct AppState {
    pub screen_mode: ScreenMode,

    pub ledger_db: Arc<ReadOnlyRocksDB>,
    pub chain_db: Arc<ReadOnlyChainDB>,

    pub frame_area: Rect,
    pub layout_model: LayoutModel,

    pub ledger_mvs: LedgerModelViewState,
    pub chain_view: ChainViewState,

    pub otel_view: OtelViewState,
    pub prom_metrics: PromMetricsViewState,

    pub button_events: mpsc::Receiver<InputEvent>,

    pub mouse_state: MouseState,

    pub component_registry: HashMap<ComponentId, Box<dyn Component + Send + Sync>>,
    pub focused_component: ComponentId,
}

impl AppState {
    pub fn new(
        ledger_db: ReadOnlyRocksDB,
        chain_db: ReadOnlyChainDB,
        trace_graph: Arc<ArcSwap<TraceGraph>>,
        prom_metrics: Receiver<NodeMetrics>,
        button_events: mpsc::Receiver<InputEvent>,
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

        let mut component_registry: HashMap<ComponentId, Box<dyn Component + Send + Sync>> =
            HashMap::new();

        let inspect_tabs: TabsComponent<InspectOption> =
            TabsComponent::new(ComponentId::InspectTabs, WidgetSlot::InspectOption);
        component_registry.insert(inspect_tabs.id(), Box::new(inspect_tabs));

        let ledger_mode_tabs: TabsComponent<LedgerMode> =
            TabsComponent::new(ComponentId::LedgerModeTabs, WidgetSlot::LedgerMode);
        component_registry.insert(ledger_mode_tabs.id(), Box::new(ledger_mode_tabs));

        Ok(Self {
            screen_mode,
            ledger_db: ledger_db_arc.clone(),
            chain_db: chain_db_arc.clone(),
            frame_area: Rect::default(),
            layout_model,
            ledger_mvs: LedgerModelViewState::new(ledger_db_arc, options_height, list_height),
            chain_view: ChainViewState::default(),
            otel_view: OtelViewState::new(trace_graph),
            prom_metrics: PromMetricsViewState::new(prom_metrics),
            button_events,
            mouse_state: MouseState::default(),
            component_registry,
            focused_component: ComponentId::InspectTabs,
        })
    }

    pub fn get_inspect_tabs(&self) -> &TabsComponent<InspectOption> {
        self.component_registry
            .get(&ComponentId::InspectTabs)
            .and_then(|c| {
                c.as_ref()
                    .as_any()
                    .downcast_ref::<TabsComponent<InspectOption>>()
            })
            .expect("InspectTabs component not in registry or wrong type")
    }

    pub fn get_inspect_tabs_mut(&mut self) -> &mut TabsComponent<InspectOption> {
        self.component_registry
            .get_mut(&ComponentId::InspectTabs)
            .and_then(|c| {
                c.as_mut()
                    .as_any_mut()
                    .downcast_mut::<TabsComponent<InspectOption>>()
            })
            .expect("InspectTabs component not in registry or wrong type")
    }

    pub fn get_ledger_mode_tabs(&self) -> &TabsComponent<LedgerMode> {
        self.component_registry
            .get(&ComponentId::LedgerModeTabs)
            .and_then(|c| {
                c.as_ref()
                    .as_any()
                    .downcast_ref::<TabsComponent<LedgerMode>>()
            })
            .expect("LedgerModeTabs component not in registry or wrong type")
    }

    pub fn get_ledger_mode_tabs_mut(&mut self) -> &mut TabsComponent<LedgerMode> {
        self.component_registry
            .get_mut(&ComponentId::LedgerModeTabs)
            .and_then(|c| {
                c.as_mut()
                    .as_any_mut()
                    .downcast_mut::<TabsComponent<LedgerMode>>()
            })
            .expect("LedgerModeTabs component not in registry or wrong type")
    }
}

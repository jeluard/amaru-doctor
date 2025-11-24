use crate::{
    ScreenMode,
    components::{
        Component, chain_search::ChainSearchComponent, details::DetailsComponent,
        flame_graph::FlameGraphComponent, prom_metrics::PromMetricsComponent,
        search_bar::SearchBarComponent, tabs::TabsComponent, trace_list::TraceListComponent,
    },
    model::{
        button::InputEvent, chain_view::ChainViewState, layout::LayoutModel,
        ledger_view::LedgerModelViewState, otel_view::OtelViewState,
    },
    otel::graph::TraceGraph,
    prometheus::model::NodeMetrics,
    states::{ComponentId, InspectOption, LedgerMode},
    update::mouse::MouseState,
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use arc_swap::ArcSwap;
use opentelemetry_proto::tonic::trace::v1::Span;
use ratatui::layout::Rect;
use std::{
    collections::HashMap,
    sync::{Arc, mpsc},
};
use tokio::sync::mpsc::Receiver;

macro_rules! register_component {
    ($registry:ident, $component_expr:expr) => {
        let component = $component_expr;
        $registry.insert(component.id(), Box::new(component));
    };
}

macro_rules! define_component_getter {
    (
        $fn_name:ident,
        $fn_name_mut:ident,
        $ComponentType:ty,
        $ComponentId:path,
        $ErrorMsg:literal
    ) => {
        pub fn $fn_name(&self) -> &$ComponentType {
            self.component_registry
                .get(&$ComponentId)
                .and_then(|c| c.as_ref().as_any().downcast_ref::<$ComponentType>())
                .expect(concat!(
                    $ErrorMsg,
                    " component not in registry or wrong type"
                ))
        }

        pub fn $fn_name_mut(&mut self) -> &mut $ComponentType {
            self.component_registry
                .get_mut(&$ComponentId)
                .and_then(|c| c.as_mut().as_any_mut().downcast_mut::<$ComponentType>())
                .expect(concat!(
                    $ErrorMsg,
                    " component not in registry or wrong type"
                ))
        }
    };
}

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

        let options_height = 0;
        let list_height = 0;

        let mut component_registry: HashMap<ComponentId, Box<dyn Component + Send + Sync>> =
            HashMap::new();

        register_component!(
            component_registry,
            crate::components::root::RootComponent::default()
        );
        register_component!(
            component_registry,
            crate::components::ledger_page::LedgerPageComponent::new(ledger_db_arc.clone())
        );
        register_component!(
            component_registry,
            crate::components::chain_page::ChainPageComponent::default()
        );
        register_component!(
            component_registry,
            crate::components::otel_page::OtelPageComponent::default()
        );
        register_component!(
            component_registry,
            crate::components::prometheus_page::PrometheusPageComponent::default()
        );

        register_component!(
            component_registry,
            TabsComponent::<LedgerMode>::new(ComponentId::LedgerModeTabs, true)
        );

        register_component!(
            component_registry,
            SearchBarComponent::new(ComponentId::SearchBar)
        );

        register_component!(
            component_registry,
            ChainSearchComponent::new(ComponentId::ChainSearch, chain_db_arc.clone())
        );

        register_component!(
            component_registry,
            TraceListComponent::new(ComponentId::OtelTraceList)
        );

        register_component!(
            component_registry,
            DetailsComponent::<Span>::new(
                ComponentId::OtelSpanDetails,
                "Span Details",
                Box::new(|s: &AppState| s.otel_view.focused_span.as_deref()),
            )
        );

        register_component!(
            component_registry,
            FlameGraphComponent::new(ComponentId::OtelFlameGraph)
        );

        register_component!(
            component_registry,
            PromMetricsComponent::new(ComponentId::PrometheusMetrics, prom_metrics)
        );

        Ok(Self {
            screen_mode,
            ledger_db: ledger_db_arc.clone(),
            chain_db: chain_db_arc.clone(),
            frame_area: Rect::default(),
            layout_model,
            ledger_mvs: LedgerModelViewState::new(options_height, list_height),
            chain_view: ChainViewState::default(),
            otel_view: OtelViewState::new(trace_graph),
            button_events,
            mouse_state: MouseState::default(),
            component_registry,
            focused_component: ComponentId::InspectTabs,
        })
    }

    define_component_getter!(
        get_ledger_mode_tabs,
        get_ledger_mode_tabs_mut,
        TabsComponent<LedgerMode>,
        ComponentId::LedgerModeTabs,
        "LedgerModeTabs"
    );

    define_component_getter!(
        get_trace_list,
        get_trace_list_mut,
        TraceListComponent,
        ComponentId::OtelTraceList,
        "OtelTraceList"
    );

    define_component_getter!(
        get_prom_metrics,
        get_prom_metrics_mut,
        PromMetricsComponent,
        ComponentId::PrometheusMetrics,
        "PromMetrics"
    );
}

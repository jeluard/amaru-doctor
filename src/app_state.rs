use crate::{
    ScreenMode,
    components::{
        Component, details::DetailsComponent, flame_graph::FlameGraphComponent,
        list::ListComponent, prom_metrics::PromMetricsComponent, search_bar::SearchBarComponent,
        tabs::TabsComponent, trace_list::TraceListComponent,
    },
    controller::{LayoutContext, compute_component_layout},
    model::{
        button::InputEvent, chain_view::ChainViewState, layout::LayoutModel,
        ledger_view::LedgerModelViewState, otel_view::OtelViewState,
    },
    otel::graph::TraceGraph,
    prometheus::model::NodeMetrics,
    states::{ComponentId, InspectOption, LedgerBrowse, LedgerMode, LedgerSearch, WidgetSlot},
    store::owned_iter::{
        OwnedAccountIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter, OwnedProposalIter,
        OwnedUtxoIter,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
    update::mouse::MouseState,
};
use amaru_consensus::{BlockHeader, Nonces};
use amaru_kernel::RawBlock;
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use anyhow::Result;
use arc_swap::ArcSwap;
use opentelemetry_proto::tonic::trace::v1::Span;
use ratatui::layout::Rect;
use std::{
    collections::HashMap,
    sync::{Arc, mpsc},
};
use strum::IntoEnumIterator;
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

        let ctx = LayoutContext {
            screen_mode,
            inspect_option: InspectOption::default(),
            ledger_mode: LedgerMode::default(),
            ledger_browse: LedgerBrowse::default(),
            ledger_search: LedgerSearch::default(),
        };

        let initial_layout = compute_component_layout(ctx, frame_area);

        let options_height: usize = initial_layout
            .get(&ComponentId::LedgerBrowseOptions)
            .ok_or(anyhow::anyhow!(
                "No rect for LedgerBrowseOptions in initial layout"
            ))?
            .height
            .into();

        let list_height: usize = initial_layout
            .get(&ComponentId::LedgerAccountsList)
            .ok_or(anyhow::anyhow!(
                "No rect for LedgerAccountsList in initial layout"
            ))?
            .height
            .into();

        let mut component_registry: HashMap<ComponentId, Box<dyn Component + Send + Sync>> =
            HashMap::new();

        register_component!(
            component_registry,
            TabsComponent::<InspectOption>::new(ComponentId::InspectTabs,)
        );

        register_component!(
            component_registry,
            TabsComponent::<LedgerMode>::new(ComponentId::LedgerModeTabs)
        );

        register_component!(
            component_registry,
            SearchBarComponent::new(ComponentId::SearchBar)
        );

        register_component!(
            component_registry,
            ListComponent::<LedgerBrowse>::new(
                ComponentId::LedgerBrowseOptions,
                "Browse Options",
                LedgerBrowse::iter(),
                options_height,
            )
        );

        register_component!(
            component_registry,
            ListComponent::<LedgerSearch>::new(
                ComponentId::LedgerSearchOptions,
                "Search Options",
                LedgerSearch::iter(),
                options_height,
            )
        );

        register_component!(
            component_registry,
            ListComponent::<AccountItem>::new(
                ComponentId::LedgerAccountsList,
                "Accounts",
                OwnedAccountIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<AccountItem>::new(
                ComponentId::LedgerAccountDetails,
                "Account Details",
                Box::new(|s: &AppState| s.get_accounts_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<BlockIssuerItem>::new(
                ComponentId::LedgerBlockIssuersList,
                "Block Issuers",
                OwnedBlockIssuerIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<BlockIssuerItem>::new(
                ComponentId::LedgerBlockIssuerDetails,
                "Block Issuer Details",
                Box::new(|s: &AppState| s.get_block_issuers_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<DRepItem>::new(
                ComponentId::LedgerDRepsList,
                "DReps",
                OwnedDRepIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<DRepItem>::new(
                ComponentId::LedgerDRepDetails,
                "DRep Details",
                Box::new(|s: &AppState| s.get_dreps_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<PoolItem>::new(
                ComponentId::LedgerPoolsList,
                "Pools",
                OwnedPoolIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<PoolItem>::new(
                ComponentId::LedgerPoolDetails,
                "Pool Details",
                Box::new(|s: &AppState| s.get_pools_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<ProposalItem>::new(
                ComponentId::LedgerProposalsList,
                "Proposals",
                OwnedProposalIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<ProposalItem>::new(
                ComponentId::LedgerProposalDetails,
                "Proposal Details",
                Box::new(|s: &AppState| s.get_proposals_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<UtxoItem>::new(
                ComponentId::LedgerUtxosList,
                "Utxos",
                OwnedUtxoIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<UtxoItem>::new(
                ComponentId::LedgerUtxoDetails,
                "Utxo Details",
                Box::new(|s: &AppState| s.get_utxos_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            DetailsComponent::<BlockHeader>::new(
                ComponentId::ChainSearchHeader,
                "Header Details",
                Box::new(|s: &AppState| {
                    s.chain_view
                        .chain_search
                        .get_current_res()
                        .and_then(|res| res.as_ref().map(|(h, _, _)| h))
                }),
            )
        );

        register_component!(
            component_registry,
            DetailsComponent::<RawBlock>::new(
                ComponentId::ChainSearchBlock,
                "Block Details",
                Box::new(|s: &AppState| {
                    s.chain_view
                        .chain_search
                        .get_current_res()
                        .and_then(|res| res.as_ref().map(|(_, b, _)| b))
                }),
            )
        );

        register_component!(
            component_registry,
            DetailsComponent::<Nonces>::new(
                ComponentId::ChainSearchNonces,
                "Nonces Details",
                Box::new(|s: &AppState| {
                    s.chain_view
                        .chain_search
                        .get_current_res()
                        .and_then(|res| res.as_ref().map(|(_, _, n)| n))
                }),
            )
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
        get_inspect_tabs,
        get_inspect_tabs_mut,
        TabsComponent<InspectOption>,
        ComponentId::InspectTabs,
        "InspectTabs"
    );

    define_component_getter!(
        get_ledger_mode_tabs,
        get_ledger_mode_tabs_mut,
        TabsComponent<LedgerMode>,
        ComponentId::LedgerModeTabs,
        "LedgerModeTabs"
    );

    define_component_getter!(
        get_ledger_browse_options,
        get_ledger_browse_options_mut,
        ListComponent<LedgerBrowse>,
        ComponentId::LedgerBrowseOptions,
        "LedgerBrowseOptions"
    );

    define_component_getter!(
        get_ledger_search_options,
        get_ledger_search_options_mut,
        ListComponent<LedgerSearch>,
        ComponentId::LedgerSearchOptions,
        "LedgerSearchOptions"
    );

    define_component_getter!(
        get_accounts_list,
        get_accounts_list_mut,
        ListComponent<AccountItem>,
        ComponentId::LedgerAccountsList,
        "LedgerAccountsList"
    );

    define_component_getter!(
        get_block_issuers_list,
        get_block_issuers_list_mut,
        ListComponent<BlockIssuerItem>,
        ComponentId::LedgerBlockIssuersList,
        "LedgerBlockIssuersList"
    );

    define_component_getter!(
        get_dreps_list,
        get_dreps_list_mut,
        ListComponent<DRepItem>,
        ComponentId::LedgerDRepsList,
        "LedgerDRepsList"
    );

    define_component_getter!(
        get_pools_list,
        get_pools_list_mut,
        ListComponent<PoolItem>,
        ComponentId::LedgerPoolsList,
        "LedgerPoolsList"
    );

    define_component_getter!(
        get_proposals_list,
        get_proposals_list_mut,
        ListComponent<ProposalItem>,
        ComponentId::LedgerProposalsList,
        "LedgerProposalsList"
    );

    define_component_getter!(
        get_utxos_list,
        get_utxos_list_mut,
        ListComponent<UtxoItem>,
        ComponentId::LedgerUtxosList,
        "LedgerUtxosList"
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

    pub fn component_id_to_widget_slot(&self, component_id: ComponentId) -> Option<WidgetSlot> {
        match component_id {
            ComponentId::InspectTabs => Some(WidgetSlot::InspectOption),
            ComponentId::LedgerModeTabs => Some(WidgetSlot::LedgerMode),
            ComponentId::SearchBar => Some(WidgetSlot::SearchBar),
            ComponentId::LedgerBrowseOptions => Some(WidgetSlot::LedgerOptions),
            ComponentId::LedgerSearchOptions => Some(WidgetSlot::LedgerOptions),
            ComponentId::LedgerAccountsList => Some(WidgetSlot::List),
            ComponentId::LedgerBlockIssuersList => Some(WidgetSlot::List),
            ComponentId::LedgerDRepsList => Some(WidgetSlot::List),
            ComponentId::LedgerPoolsList => Some(WidgetSlot::List),
            ComponentId::LedgerProposalsList => Some(WidgetSlot::List),
            ComponentId::LedgerUtxosList => Some(WidgetSlot::List),
            ComponentId::LedgerUtxosByAddrList => Some(WidgetSlot::List),
            ComponentId::LedgerAccountDetails => Some(WidgetSlot::Details),
            ComponentId::LedgerBlockIssuerDetails => Some(WidgetSlot::Details),
            ComponentId::LedgerDRepDetails => Some(WidgetSlot::Details),
            ComponentId::LedgerPoolDetails => Some(WidgetSlot::Details),
            ComponentId::LedgerProposalDetails => Some(WidgetSlot::Details),
            ComponentId::LedgerUtxoDetails => Some(WidgetSlot::Details),
            ComponentId::LedgerUtxosByAddrDetails => Some(WidgetSlot::Details),
            ComponentId::ChainSearchHeader => Some(WidgetSlot::LedgerHeaderDetails),
            ComponentId::ChainSearchBlock => Some(WidgetSlot::LedgerBlockDetails),
            ComponentId::ChainSearchNonces => Some(WidgetSlot::LedgerNoncesDetails),
            ComponentId::OtelTraceList => Some(WidgetSlot::List),
            ComponentId::OtelFlameGraph => Some(WidgetSlot::Details),
            ComponentId::OtelSpanDetails => Some(WidgetSlot::SubDetails),
            ComponentId::PrometheusMetrics => Some(WidgetSlot::Details),
            _ => None,
        }
    }

    pub fn widget_slot_to_component_id(&self, widget_slot: WidgetSlot) -> Option<ComponentId> {
        match widget_slot {
            WidgetSlot::InspectOption => Some(ComponentId::InspectTabs),
            WidgetSlot::LedgerMode => Some(ComponentId::LedgerModeTabs),
            WidgetSlot::SearchBar => Some(ComponentId::SearchBar),
            WidgetSlot::LedgerOptions => match self.get_ledger_mode_tabs().selected() {
                LedgerMode::Browse => Some(ComponentId::LedgerBrowseOptions),
                LedgerMode::Search => Some(ComponentId::LedgerSearchOptions),
            },
            WidgetSlot::List => {
                match self.get_inspect_tabs().selected() {
                    InspectOption::Ledger => {
                        match self.get_ledger_browse_options().model_view.selected_item() {
                            Some(LedgerBrowse::Accounts) => Some(ComponentId::LedgerAccountsList),
                            Some(LedgerBrowse::BlockIssuers) => {
                                Some(ComponentId::LedgerBlockIssuersList)
                            }
                            Some(LedgerBrowse::DReps) => Some(ComponentId::LedgerDRepsList),
                            Some(LedgerBrowse::Pools) => Some(ComponentId::LedgerPoolsList),
                            Some(LedgerBrowse::Proposals) => Some(ComponentId::LedgerProposalsList),
                            Some(LedgerBrowse::Utxos) => Some(ComponentId::LedgerUtxosList),
                            _ => None,
                        }
                    }
                    InspectOption::Chain => None, // No direct list mapping for Chain
                    InspectOption::Otel => Some(ComponentId::OtelTraceList),
                    InspectOption::Prometheus => None, // No direct list mapping for Prometheus
                }
            }
            WidgetSlot::Details => {
                match self.get_inspect_tabs().selected() {
                    InspectOption::Ledger => {
                        match self.get_ledger_browse_options().model_view.selected_item() {
                            Some(LedgerBrowse::Accounts) => Some(ComponentId::LedgerAccountDetails),
                            Some(LedgerBrowse::BlockIssuers) => {
                                Some(ComponentId::LedgerBlockIssuerDetails)
                            }
                            Some(LedgerBrowse::DReps) => Some(ComponentId::LedgerDRepDetails),
                            Some(LedgerBrowse::Pools) => Some(ComponentId::LedgerPoolDetails),
                            Some(LedgerBrowse::Proposals) => {
                                Some(ComponentId::LedgerProposalDetails)
                            }
                            Some(LedgerBrowse::Utxos) => Some(ComponentId::LedgerUtxoDetails),
                            _ => None,
                        }
                    }
                    InspectOption::Chain => {
                        // This is ambiguous since there's 3 options, will need to refine later
                        Some(ComponentId::ChainSearchHeader)
                    }
                    InspectOption::Otel => Some(ComponentId::OtelFlameGraph),
                    InspectOption::Prometheus => Some(ComponentId::PrometheusMetrics),
                }
            }
            WidgetSlot::SubDetails => Some(ComponentId::OtelSpanDetails),
            WidgetSlot::LedgerHeaderDetails => Some(ComponentId::ChainSearchHeader),
            WidgetSlot::LedgerBlockDetails => Some(ComponentId::ChainSearchBlock),
            WidgetSlot::LedgerNoncesDetails => Some(ComponentId::ChainSearchNonces),
            _ => None,
        }
    }
}

use crate::{
    ScreenMode,
    components::{
        Component, details::DetailsComponent, list::ListComponent, search_bar::SearchBarComponent,
        tabs::TabsComponent, trace_list::TraceListComponent,
    },
    model::{
        button::InputEvent, chain_view::ChainViewState, layout::LayoutModel,
        ledger_view::LedgerModelViewState, otel_view::OtelViewState,
        prom_metrics::PromMetricsViewState,
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

        register_component!(
            component_registry,
            TabsComponent::<InspectOption>::new(
                ComponentId::InspectTabs,
                WidgetSlot::InspectOption
            )
        );

        register_component!(
            component_registry,
            TabsComponent::<LedgerMode>::new(ComponentId::LedgerModeTabs, WidgetSlot::LedgerMode)
        );

        register_component!(
            component_registry,
            SearchBarComponent::new(ComponentId::SearchBar, WidgetSlot::SearchBar)
        );

        register_component!(
            component_registry,
            ListComponent::<LedgerBrowse>::new(
                ComponentId::LedgerBrowseOptions,
                WidgetSlot::LedgerOptions,
                "Browse Options",
                LedgerBrowse::iter(),
                options_height,
            )
        );

        register_component!(
            component_registry,
            ListComponent::<LedgerSearch>::new(
                ComponentId::LedgerSearchOptions,
                WidgetSlot::LedgerOptions,
                "Search Options",
                LedgerSearch::iter(),
                options_height,
            )
        );

        register_component!(
            component_registry,
            ListComponent::<AccountItem>::new(
                ComponentId::LedgerAccountsList,
                WidgetSlot::List,
                "Accounts",
                OwnedAccountIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<AccountItem>::new(
                ComponentId::LedgerAccountDetails,
                WidgetSlot::Details,
                "Account Details",
                Box::new(|s: &AppState| s.get_accounts_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<BlockIssuerItem>::new(
                ComponentId::LedgerBlockIssuersList,
                WidgetSlot::List,
                "Block Issuers",
                OwnedBlockIssuerIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<BlockIssuerItem>::new(
                ComponentId::LedgerBlockIssuerDetails,
                WidgetSlot::Details,
                "Block Issuer Details",
                Box::new(|s: &AppState| s.get_block_issuers_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<DRepItem>::new(
                ComponentId::LedgerDRepsList,
                WidgetSlot::List,
                "DReps",
                OwnedDRepIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<DRepItem>::new(
                ComponentId::LedgerDRepDetails,
                WidgetSlot::Details,
                "DRep Details",
                Box::new(|s: &AppState| s.get_dreps_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<PoolItem>::new(
                ComponentId::LedgerPoolsList,
                WidgetSlot::List,
                "Pools",
                OwnedPoolIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<PoolItem>::new(
                ComponentId::LedgerPoolDetails,
                WidgetSlot::Details,
                "Pool Details",
                Box::new(|s: &AppState| s.get_pools_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<ProposalItem>::new(
                ComponentId::LedgerProposalsList,
                WidgetSlot::List,
                "Proposals",
                OwnedProposalIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<ProposalItem>::new(
                ComponentId::LedgerProposalDetails,
                WidgetSlot::Details,
                "Proposal Details",
                Box::new(|s: &AppState| s.get_proposals_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            ListComponent::<UtxoItem>::new(
                ComponentId::LedgerUtxosList,
                WidgetSlot::List,
                "Utxos",
                OwnedUtxoIter::new(ledger_db_arc.clone()),
                list_height,
            )
        );
        register_component!(
            component_registry,
            DetailsComponent::<UtxoItem>::new(
                ComponentId::LedgerUtxoDetails,
                WidgetSlot::Details,
                "Utxo Details",
                Box::new(|s: &AppState| s.get_utxos_list().model_view.selected_item()),
            )
        );

        register_component!(
            component_registry,
            DetailsComponent::<BlockHeader>::new(
                ComponentId::ChainSearchHeader,
                WidgetSlot::LedgerHeaderDetails,
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
                WidgetSlot::LedgerBlockDetails,
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
                WidgetSlot::LedgerNoncesDetails,
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
            TraceListComponent::new(ComponentId::OtelTraceList, WidgetSlot::List)
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
            prom_metrics: PromMetricsViewState::new(prom_metrics),
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
}

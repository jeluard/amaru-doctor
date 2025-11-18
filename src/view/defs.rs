use crate::components::Component;
use crate::states::{ComponentId, WidgetSlot};
use crate::view::adapter::ComponentViewAdapter;
use crate::view::empty_list::draw_empty_list;
use crate::view::item_details::draw_details;
use crate::{
    app_state::AppState,
    states::{InspectOption, LedgerBrowse, LedgerMode, LedgerSearch},
    view::View,
};
use ratatui::{Frame, layout::Rect};
use std::collections::HashMap;

pub static INSPECT_TABS_VIEW: ComponentViewAdapter =
    ComponentViewAdapter::always_visible(ComponentId::InspectTabs, WidgetSlot::InspectOption);
pub static LEDGER_MODE_TABS_VIEW: ComponentViewAdapter =
    ComponentViewAdapter::always_visible(ComponentId::LedgerModeTabs, WidgetSlot::LedgerMode);

pub static SEARCH_BAR_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::SearchBar,
    WidgetSlot::SearchBar,
    |s: &AppState| match s.get_inspect_tabs().selected() {
        InspectOption::Ledger => true,
        // InspectOption::Chain => true,
        InspectOption::Otel => false,
        InspectOption::Prometheus => false,
        InspectOption::Chain => true,
    },
);

pub static LEDGER_BROWSE_OPTIONS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerBrowseOptions,
    WidgetSlot::LedgerOptions,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
    },
);

pub static CHAIN_SEARCH_HEADER_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::ChainSearchHeader,
    WidgetSlot::LedgerHeaderDetails,
    |s: &AppState| s.get_inspect_tabs().selected() == InspectOption::Chain,
);

pub static CHAIN_SEARCH_BLOCK_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::ChainSearchBlock,
    WidgetSlot::LedgerBlockDetails,
    |s: &AppState| s.get_inspect_tabs().selected() == InspectOption::Chain,
);

pub static CHAIN_SEARCH_NONCES_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::ChainSearchNonces,
    WidgetSlot::LedgerNoncesDetails,
    |s: &AppState| s.get_inspect_tabs().selected() == InspectOption::Chain,
);

pub static LEDGER_SEARCH_OPTIONS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerSearchOptions,
    WidgetSlot::LedgerOptions,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Search
    },
);

pub static LEDGER_ACCOUNTS_LIST_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerAccountsList,
    WidgetSlot::List,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::Accounts)
    },
);

pub static LEDGER_ACCOUNT_DETAILS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerAccountDetails,
    WidgetSlot::Details,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::Accounts)
    },
);

pub static LEDGER_BLOCK_ISSUERS_LIST_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerBlockIssuersList,
    WidgetSlot::List,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::BlockIssuers)
    },
);
pub static LEDGER_BLOCK_ISSUER_DETAILS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerBlockIssuerDetails,
    WidgetSlot::Details,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::BlockIssuers)
    },
);

pub static LEDGER_DREPS_LIST_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerDRepsList,
    WidgetSlot::List,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::DReps)
    },
);
pub static LEDGER_DREP_DETAILS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerDRepDetails,
    WidgetSlot::Details,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::DReps)
    },
);

pub static LEDGER_POOLS_LIST_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerPoolsList,
    WidgetSlot::List,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::Pools)
    },
);
pub static LEDGER_POOL_DETAILS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerPoolDetails,
    WidgetSlot::Details,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::Pools)
    },
);

pub static LEDGER_PROPOSALS_LIST_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerProposalsList,
    WidgetSlot::List,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::Proposals)
    },
);
pub static LEDGER_PROPOSAL_DETAILS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerProposalDetails,
    WidgetSlot::Details,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::Proposals)
    },
);

pub static LEDGER_UTXOS_LIST_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerUtxosList,
    WidgetSlot::List,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::Utxos)
    },
);
pub static LEDGER_UTXO_DETAILS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::LedgerUtxoDetails,
    WidgetSlot::Details,
    |s: &AppState| {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Browse
            && s.get_ledger_browse_options().model_view.selected_item()
                == Some(&LedgerBrowse::Utxos)
    },
);

#[derive(Default)]
pub struct LedgerUtxosByAddr;
impl View for LedgerUtxosByAddr {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::List
    }

    fn is_visible(&self, s: &AppState) -> bool {
        s.get_inspect_tabs().selected() == InspectOption::Ledger
            && s.get_ledger_mode_tabs().selected() == LedgerMode::Search
            && s.get_ledger_search_options().model_view.selected_item()
                == Some(LedgerSearch::UtxosByAddress).as_ref()
    }

    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        let is_focused = s.layout_model.is_focused(s, self.slot());
        if let Some(res) = s.ledger_mvs.utxos_by_addr_search.get_current_res() {
            let mut layout = HashMap::new();
            layout.insert(res.id(), area);
            res.render(f, s, &layout);
        } else {
            draw_empty_list(f, area, "Utxos by Addr", "No results", is_focused);
        }
    }
}

pub struct LedgerSearchUtxoDetails;
impl View for LedgerSearchUtxoDetails {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Details
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.get_inspect_tabs().cursor.current() == InspectOption::Ledger
            && *s.get_ledger_mode_tabs().cursor.current() == LedgerMode::Search
            && s.get_ledger_search_options().model_view.selected_item()
                == Some(LedgerSearch::UtxosByAddress).as_ref()
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        let is_focused = s.layout_model.is_focused(s, self.slot());
        let res_opt = s.ledger_mvs.utxos_by_addr_search.get_current_res();
        let item_opt = res_opt.and_then(|res| res.selected_item());
        draw_details(f, area, "Utxo Details".to_owned(), item_opt, is_focused);
    }
}

pub static TRACE_LIST_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::OtelTraceList,
    WidgetSlot::List,
    |s: &AppState| s.get_inspect_tabs().selected() == InspectOption::Otel,
);

pub static OTEL_SPAN_DETAILS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::OtelSpanDetails,
    WidgetSlot::SubDetails,
    |s: &AppState| s.get_inspect_tabs().selected() == InspectOption::Otel,
);

pub static OTEL_FLAME_GRAPH_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::OtelFlameGraph,
    WidgetSlot::Details,
    |s: &AppState| s.get_inspect_tabs().selected() == InspectOption::Otel,
);

pub static PROM_METRICS_VIEW: ComponentViewAdapter = ComponentViewAdapter::new(
    ComponentId::PrometheusMetrics,
    WidgetSlot::Details,
    |s: &AppState| s.get_inspect_tabs().selected() == InspectOption::Prometheus,
);

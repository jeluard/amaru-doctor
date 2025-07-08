use crate::view::block::render_block;
use crate::view::nonces::render_nonces;
use crate::view::otel::render_otel_snapshot;
use crate::view::metrics::render_metrics_snapshot;
use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    states::{BrowseOption, InspectOption, LedgerMode, LedgerSearchOption, WidgetSlot},
    view::{
        View, details::render_details, header::render_header, line::render_line, list::render_list,
        search::render_search_query, tabs::render_tabs,
    },
};
use amaru_consensus::Nonces;
use amaru_kernel::{Header, RawBlock};
use color_eyre::Result;
use ratatui::{Frame, layout::Rect};

pub struct InspectTabs;
impl View for InspectTabs {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::InspectOption
    }
    fn is_visible(&self, _s: &AppState) -> bool {
        true
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_tabs(
            f,
            area,
            "Inspect Option",
            &s.inspect_option,
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct LedgerModeTabs;
impl View for LedgerModeTabs {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerMode
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Ledger
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_tabs(
            f,
            area,
            "Ledger Mode",
            &s.ledger_mode,
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct SearchBar;
impl View for SearchBar {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::SearchBar
    }
    fn is_visible(&self, s: &AppState) -> bool {
        match s.inspect_option.current() {
            InspectOption::Ledger => true,
            InspectOption::Chain => true,
            InspectOption::Otel => false,
        }
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_search_query(
            f,
            area,
            "Search",
            match s.inspect_option.current() {
                InspectOption::Ledger => match s.ledger_search_options.selected() {
                    Some(LedgerSearchOption::UtxosByAddress) => &s.utxos_by_addr_search.builder,
                    None => "",
                },
                InspectOption::Chain => &s.chain_search.builder,
                InspectOption::Otel => "",
            },
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct LedgerBrowseOptions;
impl View for LedgerBrowseOptions {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Options
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Ledger
            && *s.ledger_mode.current() == LedgerMode::Browse
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_list(
            f,
            area,
            "Browse Options",
            Some(&s.ledger_browse_options),
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct ChainSearchHeader;
impl View for ChainSearchHeader {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerHeaderDetails
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Chain
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        let header_opt: Option<Option<&Header>> = s
            .chain_search
            .get_current_res()
            .map(|res| res.as_ref().map(|(h, _, _)| h));
        render_header(
            f,
            area,
            "Header Details",
            header_opt,
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct ChainSearchBlock;
impl View for ChainSearchBlock {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerBlockDetails
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Chain
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        let block_opt_opt: Option<Option<&RawBlock>> = s
            .chain_search
            .get_current_res()
            .map(|res| res.as_ref().map(|(_, b, _)| b));
        render_block(
            f,
            area,
            "Block Details",
            block_opt_opt,
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct ChainSearchNonces;
impl View for ChainSearchNonces {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerNoncesDetails
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Chain
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        let nonces_opt_opt: Option<Option<&Nonces>> = s
            .chain_search
            .get_current_res()
            .map(|res| res.as_ref().map(|(_, _, n)| n));
        render_nonces(
            f,
            area,
            "Nonces Details",
            nonces_opt_opt,
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct LedgerSearchOptions;
impl View for LedgerSearchOptions {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Options
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Ledger
            && *s.ledger_mode.current() == LedgerMode::Search
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_list(
            f,
            area,
            "Search Options",
            Some(&s.ledger_search_options),
            is_widget_focused(s, self.slot()),
        )
    }
}

macro_rules! browse_views {
    ($(($variant:ident, $list_struct:ident, $details_struct:ident, $field:ident, $label:expr)),* $(,)?) => {
        $(
            pub struct $list_struct;
            impl View for $list_struct {
                fn slot(&self) -> WidgetSlot { WidgetSlot::List }
                fn is_visible(&self, s: &AppState) -> bool {
                    *s.inspect_option.current() == InspectOption::Ledger &&
                    *s.ledger_mode.current() == LedgerMode::Browse &&
                    s.ledger_browse_options.selected() == Some(&BrowseOption::$variant)
                }
                fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
                    render_list(
                        f,
                        area,
                        $label,
                        Some(&s.$field),
                        is_widget_focused(s, WidgetSlot::List),
                    )
                }
            }

            pub struct $details_struct;
            impl View for $details_struct {
                fn slot(&self) -> WidgetSlot { WidgetSlot::Details }
                fn is_visible(&self, s: &AppState) -> bool {
                    *s.inspect_option.current() == InspectOption::Ledger &&
                    *s.ledger_mode.current() == LedgerMode::Browse &&
                    s.ledger_browse_options.selected() == Some(&BrowseOption::$variant)
                }
                fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
                    render_details(
                        f,
                        area,
                        $label,
                        Some(&s.$field),
                        is_widget_focused(s, WidgetSlot::Details),
                    )
                }
            }
        )*
    }
}

browse_views!(
    (
        Accounts,
        LedgerAccounts,
        LedgerAccountDetails,
        accounts,
        "Accounts"
    ),
    (
        BlockIssuers,
        LedgerBlockIssuers,
        LedgerBlockIssuerDetails,
        block_issuers,
        "Block Issuers"
    ),
    (DReps, LedgerDReps, LedgerDRepDetails, dreps, "DReps"),
    (Pools, LedgerPools, LedgerPoolDetails, pools, "Pools"),
    (
        Proposals,
        LedgerProposals,
        LedgerProposalDetails,
        proposals,
        "Proposals"
    ),
    (Utxos, LedgerUtxos, LedgerUtxoDetails, utxos, "Utxos"),
);

pub struct LedgerUtxosByAddr;
impl View for LedgerUtxosByAddr {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::List
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Ledger
            && *s.ledger_mode.current() == LedgerMode::Search
            && s.ledger_search_options.selected()
                == Some(LedgerSearchOption::UtxosByAddress).as_ref()
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_list(
            f,
            area,
            "Utxos by Address",
            s.utxos_by_addr_search.get_current_res(),
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct LedgerSearchUtxoDetails;
impl View for LedgerSearchUtxoDetails {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Details
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Ledger
            && *s.ledger_mode.current() == LedgerMode::Search
            && s.ledger_search_options.selected()
                == Some(LedgerSearchOption::UtxosByAddress).as_ref()
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_details(
            f,
            area,
            "Utxo Details",
            s.utxos_by_addr_search.get_current_res(),
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct OtelDetails;
impl View for OtelDetails {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Details
    }

    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Otel
    }

    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        let spans = s.collector.snapshot();
        render_otel_snapshot(
            frame,
            area,
            "Otel",
            spans,
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct MetricsDetails;
impl View for MetricsDetails {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Details
    }

    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Metrics
    }

    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        let metrics = s.metrics_collector.snapshot();
        render_metrics_snapshot(
            frame,
            area,
            "Metrics",
            metrics,
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct BottomLine;
impl View for BottomLine {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::BottomLine
    }
    fn is_visible(&self, _s: &AppState) -> bool {
        true
    }
    fn render(&self, f: &mut Frame, area: Rect, _s: &AppState) -> Result<()> {
        render_line(
            f,
            area,
            "Use shift + arrow keys to move focus. Use arrow keys to scroll.".to_owned(),
        )
    }
}

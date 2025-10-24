use crate::view::block::render_block;
use crate::view::empty_list::draw_empty_list;
use crate::view::flame_graph::render_flame_graph;
use crate::view::item_details::draw_details;
use crate::view::nonces::render_nonces;
use crate::view::prom_metrics::render_prom_metrics;
use crate::view::span::render_span;
use crate::view::trace_list::render_traces;
use crate::{
    app_state::AppState,
    states::{InspectOption, LedgerBrowse, LedgerMode, LedgerSearch, WidgetSlot},
    view::{View, header::render_header, line::render_line, search::render_search_query},
};
use amaru_consensus::{BlockHeader, Nonces};
use amaru_kernel::RawBlock;
use ratatui::{Frame, layout::Rect};

pub struct InspectTabs;
impl View for InspectTabs {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::InspectOption
    }
    fn is_visible(&self, _s: &AppState) -> bool {
        true
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        s.inspect_tabs
            .draw(f, area, s.layout_model.is_focused(self.slot()));
    }
}

pub struct LedgerModeTabs;
impl View for LedgerModeTabs {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerMode
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_tabs.cursor.current() == InspectOption::Ledger
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        s.ledger_tabs
            .draw(f, area, s.layout_model.is_focused(self.slot()));
    }
}

pub struct SearchBar;
impl View for SearchBar {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::SearchBar
    }
    fn is_visible(&self, s: &AppState) -> bool {
        match s.inspect_tabs.cursor.current() {
            InspectOption::Ledger => true,
            //InspectOption::Chain => true,
            InspectOption::Otel => false,
            InspectOption::Prometheus => false,
        }
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        render_search_query(
            f,
            area,
            "Search",
            match s.inspect_tabs.cursor.current() {
                InspectOption::Ledger => match s.ledger_mvs.search_options.selected_item() {
                    Some(LedgerSearch::UtxosByAddress) => {
                        &s.ledger_mvs.utxos_by_addr_search.builder
                    }
                    None => "",
                },
                //InspectOption::Chain => &s.chain_view.chain_search.builder,
                InspectOption::Otel => "",
                InspectOption::Prometheus => "",
            },
            s.layout_model.is_focused(self.slot()),
        );
    }
}

pub struct LedgerBrowseOptions;
impl View for LedgerBrowseOptions {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerOptions
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_tabs.cursor.current() == InspectOption::Ledger
            && *s.ledger_tabs.cursor.current() == LedgerMode::Browse
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        let is_focused = s.layout_model.is_focused(self.slot());
        s.ledger_mvs.browse_options.draw(f, area, is_focused);
    }
}

pub struct ChainSearchHeader;
impl View for ChainSearchHeader {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerHeaderDetails
    }
    fn is_visible(&self, s: &AppState) -> bool {
        true//*s.inspect_tabs.cursor.current() == InspectOption::Chain
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        let header_opt: Option<Option<&BlockHeader>> = s
            .chain_view
            .chain_search
            .get_current_res()
            .map(|res| res.as_ref().map(|(h, _, _)| h));
        render_header(
            f,
            area,
            "Header Details",
            header_opt,
            s.layout_model.is_focused(self.slot()),
        );
    }
}

pub struct ChainSearchBlock;
impl View for ChainSearchBlock {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerBlockDetails
    }
    fn is_visible(&self, s: &AppState) -> bool {
        true//*s.inspect_tabs.cursor.current() == InspectOption::Chain
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        let block_opt_opt: Option<Option<&RawBlock>> = s
            .chain_view
            .chain_search
            .get_current_res()
            .map(|res| res.as_ref().map(|(_, b, _)| b));
        render_block(
            f,
            area,
            "Block Details",
            block_opt_opt,
            s.layout_model.is_focused(self.slot()),
        );
    }
}

pub struct ChainSearchNonces;
impl View for ChainSearchNonces {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerNoncesDetails
    }
    fn is_visible(&self, s: &AppState) -> bool {
        true//*s.inspect_tabs.cursor.current() == InspectOption::Chain
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        let nonces_opt_opt: Option<Option<&Nonces>> = s
            .chain_view
            .chain_search
            .get_current_res()
            .map(|res| res.as_ref().map(|(_, _, n)| n));
        render_nonces(
            f,
            area,
            "Nonces Details",
            nonces_opt_opt,
            s.layout_model.is_focused(self.slot()),
        );
    }
}

pub struct LedgerSearchOptions;
impl View for LedgerSearchOptions {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::LedgerOptions
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_tabs.cursor.current() == InspectOption::Ledger
            && *s.ledger_tabs.cursor.current() == LedgerMode::Search
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        let is_focused = s.layout_model.is_focused(self.slot());
        s.ledger_mvs.search_options.draw(f, area, is_focused);
    }
}

macro_rules! browse_views {
    ($(($variant:ident, $list_struct:ident, $details_struct:ident, $field:ident, $label:expr)),* $(,)?) => {
        $(
            pub struct $list_struct;
            impl View for $list_struct {
                fn slot(&self) -> WidgetSlot { WidgetSlot::List }
                fn is_visible(&self, s: &AppState) -> bool {
                    *s.inspect_tabs.cursor.current() == InspectOption::Ledger &&
                    *s.ledger_tabs.cursor.current() == LedgerMode::Browse &&
                    s.ledger_mvs.browse_options.selected_item() == Some(&LedgerBrowse::$variant)
                }
                fn render(&self, f: &mut Frame, area: Rect, s: &AppState)  {
                    let is_focused = s.layout_model.is_focused(self.slot());
                    s.ledger_mvs.$field.draw(f, area, is_focused);
                }
            }

            pub struct $details_struct;
            impl View for $details_struct {
                fn slot(&self) -> WidgetSlot { WidgetSlot::Details }
                fn is_visible(&self, s: &AppState) -> bool {
                    let visible = *s.inspect_tabs.cursor.current() == InspectOption::Ledger &&
                    *s.ledger_tabs.cursor.current() == LedgerMode::Browse &&
                    s.ledger_mvs.browse_options.selected_item() == Some(&LedgerBrowse::$variant);
                    visible
                }
                fn render(&self, f: &mut Frame, area: Rect, s: &AppState)  {
                    let is_focused = s.layout_model.is_focused(self.slot());
                    draw_details(f, area, format!("{} Details", $label.to_owned()), s.ledger_mvs.$field.selected_item(), is_focused);
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
        *s.inspect_tabs.cursor.current() == InspectOption::Ledger
            && *s.ledger_tabs.cursor.current() == LedgerMode::Search
            && s.ledger_mvs.search_options.selected_item()
                == Some(LedgerSearch::UtxosByAddress).as_ref()
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        let is_focused = s.layout_model.is_focused(self.slot());
        if let Some(res) = s.ledger_mvs.utxos_by_addr_search.get_current_res() {
            res.draw(f, area, is_focused);
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
        *s.inspect_tabs.cursor.current() == InspectOption::Ledger
            && *s.ledger_tabs.cursor.current() == LedgerMode::Search
            && s.ledger_mvs.search_options.selected_item()
                == Some(LedgerSearch::UtxosByAddress).as_ref()
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        let is_focused = s.layout_model.is_focused(self.slot());
        let res_opt = s.ledger_mvs.utxos_by_addr_search.get_current_res();
        draw_details(
            f,
            area,
            "Utxo Details".to_owned(),
            res_opt.and_then(|res| res.selected_item()),
            is_focused,
        );
    }
}

pub struct TraceList;
impl View for TraceList {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::List
    }

    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_tabs.cursor.current() == InspectOption::Otel
    }

    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState) {
        render_traces(
            frame,
            area,
            &s.otel_view.trace_list,
            s.layout_model.is_focused(self.slot()),
        );
    }
}

pub struct FlameGraphDetails;
impl View for FlameGraphDetails {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Details
    }

    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_tabs.cursor.current() == InspectOption::Otel
    }

    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState) {
        render_flame_graph(
            frame,
            area,
            &s.otel_view,
            s.layout_model.is_focused(self.slot()),
        );
    }
}

pub struct SpanDetails;
impl View for SpanDetails {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::SubDetails
    }

    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_tabs.cursor.current() == InspectOption::Otel
    }

    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState) {
        render_span(
            frame,
            area,
            &s.otel_view,
            s.layout_model.is_focused(self.slot()),
        );
    }
}

pub struct PromMetrics;
impl View for PromMetrics {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Details
    }

    fn is_visible(&self, s: &AppState) -> bool {
        *s.inspect_tabs.cursor.current() == InspectOption::Prometheus
    }

    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState) {
        render_prom_metrics(
            frame,
            area,
            &s.prom_metrics,
            s.layout_model.is_focused(self.slot()),
        );
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
    fn render(&self, f: &mut Frame, area: Rect, _s: &AppState) {
        render_line(
            f,
            area,
            "Use shift + arrow keys to move focus. Use arrow keys to scroll.".to_owned(),
        );
    }
}

use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    states::{BrowseOption, LedgerMode, LedgerSearchOption, StoreOption, WidgetSlot},
    types::chain::ChainSearchOption,
    view::{
        View, details::render_details, header::render_header, line::render_line, list::render_list,
        search::render_search_query, tabs::render_tabs,
    },
};
use amaru_kernel::Header;
use color_eyre::Result;
use ratatui::{Frame, layout::Rect};

pub struct TopLine;
impl View for TopLine {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::TopLine
    }
    fn is_visible(&self, _s: &AppState) -> bool {
        true
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_line(f, area, format!("Reading amaru dir at {}", s.ledger_path))
    }
}

pub struct StoreTabs;
impl View for StoreTabs {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::StoreOption
    }
    fn is_visible(&self, _s: &AppState) -> bool {
        true
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_tabs(
            f,
            area,
            "Store",
            &s.store_option,
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
        *s.store_option.current() == StoreOption::Ledger
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
    fn is_visible(&self, _s: &AppState) -> bool {
        true
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_search_query(
            f,
            area,
            "Search",
            match s.store_option.current() {
                StoreOption::Ledger => &s.ledger_search_query_bldr,
                StoreOption::Chain => &s.chain_search_query_bldr,
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
        *s.store_option.current() == StoreOption::Ledger
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

pub struct ChainSearchOptions;
impl View for ChainSearchOptions {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Options
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.store_option.current() == StoreOption::Chain
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_list(
            f,
            area,
            "Search Options",
            Some(&s.chain_search_options),
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct ChainSearchHeader;
impl View for ChainSearchHeader {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Details
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.store_option.current() == StoreOption::Chain
            && s.chain_search_options.selected() == Some(&ChainSearchOption::Header)
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        let header_opt: Option<&Header> = s
            .chain_search_query_hash
            .as_ref()
            .and_then(|h| s.headers_by_hash_search_res.get(h))
            .and_then(|opt_hdr| opt_hdr.as_ref());
        render_header(
            f,
            area,
            "Header Details",
            header_opt,
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
        *s.store_option.current() == StoreOption::Ledger
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
                    *s.store_option.current()   == StoreOption::Ledger &&
                    *s.ledger_mode.current()    == LedgerMode::Browse &&
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
                    *s.store_option.current()   == StoreOption::Ledger &&
                    *s.ledger_mode.current()    == LedgerMode::Browse &&
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
        *s.store_option.current() == StoreOption::Ledger
            && *s.ledger_mode.current() == LedgerMode::Search
            && s.ledger_search_options.selected()
                == Some(LedgerSearchOption::UtxosByAddress).as_ref()
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_list(
            f,
            area,
            "Utxos by Address",
            s.ledger_search_query_addr
                .as_ref()
                .and_then(|a| s.utxos_by_addr_search_res.get(a)),
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
        *s.store_option.current() == StoreOption::Ledger
            && *s.ledger_mode.current() == LedgerMode::Search
            && s.ledger_search_options.selected()
                == Some(LedgerSearchOption::UtxosByAddress).as_ref()
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_details(
            f,
            area,
            "Utxo Details",
            s.ledger_search_query_addr
                .as_ref()
                .and_then(|a| s.utxos_by_addr_search_res.get(a)),
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

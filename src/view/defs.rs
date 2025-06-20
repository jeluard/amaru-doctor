use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    states::{BrowseOption, LedgerMode, SearchOption, StoreOption, WidgetSlot},
    view::{
        View, details::render_details, line::render_line, list::render_list,
        search::render_search_query, tabs::render_tabs,
    },
};
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
    fn is_visible(&self, s: &AppState) -> bool {
        *s.store_option.current() == StoreOption::Ledger
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_search_query(
            f,
            area,
            "Search",
            &s.search_query_bldr,
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct BrowseOptions;
impl View for BrowseOptions {
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
            Some(&s.browse_options),
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct SearchOptions;
impl View for SearchOptions {
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
            Some(&s.search_options),
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
                    s.browse_options.selected() == Some(&BrowseOption::$variant)
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
                    s.browse_options.selected() == Some(&BrowseOption::$variant)
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
    (Accounts, Accounts, AccountDetails, accounts, "Accounts"),
    (
        BlockIssuers,
        BlockIssuers,
        BlockIssuerDetails,
        block_issuers,
        "Block Issuers"
    ),
    (DReps, DReps, DRepDetails, dreps, "DReps"),
    (Pools, Pools, PoolDetails, pools, "Pools"),
    (
        Proposals,
        Proposals,
        ProposalDetails,
        proposals,
        "Proposals"
    ),
    (Utxos, Utxos, UtxoDetails, utxos, "Utxos"),
);

pub struct UtxosByAddrList;
impl View for UtxosByAddrList {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::List
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.store_option.current() == StoreOption::Ledger
            && *s.ledger_mode.current() == LedgerMode::Search
            && s.search_options.selected() == Some(SearchOption::UtxosByAddress).as_ref()
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_list(
            f,
            area,
            "Utxos by Address",
            s.search_query_addr
                .as_ref()
                .and_then(|a| s.utxos_by_addr_search_res.get(a)),
            is_widget_focused(s, self.slot()),
        )
    }
}

pub struct SearchUtxoDetails;
impl View for SearchUtxoDetails {
    fn slot(&self) -> WidgetSlot {
        WidgetSlot::Details
    }
    fn is_visible(&self, s: &AppState) -> bool {
        *s.store_option.current() == StoreOption::Ledger
            && *s.ledger_mode.current() == LedgerMode::Search
            && s.search_options.selected() == Some(SearchOption::UtxosByAddress).as_ref()
    }
    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) -> Result<()> {
        render_details(
            f,
            area,
            "Utxo Details",
            s.search_query_addr
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

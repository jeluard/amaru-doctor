use crate::{
    app_state::AppState,
    controller::resolve_placed_widget_id,
    states::{
        WidgetId::{self, *},
        WidgetSlot,
    },
    view::{
        details::{DetailsView, OptDetailsView},
        empty::EmptyView,
        line::LineView,
        list::{ListView, OptListView},
        search::SearchQueryView,
        tabs::TabsView,
    },
};
use color_eyre::Result;
use ratatui::{Frame, layout::Rect};

pub mod details;
pub mod empty;
pub mod line;
pub mod list;
pub mod search;
pub mod tabs;

pub trait View {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()>;
}

// TODO: Cache this rather than query it every draw
pub fn view_for(widget_id: WidgetId) -> Box<dyn View> {
    match widget_id {
        Empty => Box::new(EmptyView::new(widget_id)),
        TopInfo => Box::new(LineView::new(widget_id, |s: &AppState| {
            format!("Reading amaru dir at {}", &s.ledger_path)
        })),
        BottomInfo => Box::new(LineView::new(widget_id, |_s: &AppState| {
            "Use shift + arrow keys to move focus. Use arrow keys to scroll.".to_owned()
        })),
        StoreOption => Box::new(TabsView::new(StoreOption, |s: &AppState| &s.store_option)),
        LedgerMode => Box::new(TabsView::new(LedgerMode, |s: &AppState| &s.ledger_mode)),
        BrowseOptions => Box::new(ListView::new(BrowseOptions, |s: &AppState| {
            &s.browse_options
        })),
        SearchOptions => Box::new(ListView::new(SearchOptions, |s: &AppState| {
            &s.search_options
        })),
        SearchQuery => Box::new(SearchQueryView::new(SearchQuery, |s: &AppState| {
            &s.search_query_bldr
        })),
        ListUtxosByAddr => Box::new(OptListView::new(ListUtxosByAddr, |s: &AppState| {
            s.search_query_addr
                .as_ref()
                .and_then(|addr| s.utxos_by_addr_search_res.get(addr))
        })),
        ListAccounts => Box::new(ListView::new(ListAccounts, |s: &AppState| &s.accounts)),
        ListBlockIssuers => Box::new(ListView::new(ListBlockIssuers, |s: &AppState| {
            &s.block_issuers
        })),
        ListDReps => Box::new(ListView::new(ListDReps, |s: &AppState| &s.dreps)),
        ListPools => Box::new(ListView::new(ListPools, |s: &AppState| &s.pools)),
        ListProposals => Box::new(ListView::new(ListProposals, |s: &AppState| &s.proposals)),
        ListUtxos => Box::new(ListView::new(ListUtxos, |s: &AppState| &s.utxos)),

        DetailsAccount => Box::new(DetailsView::new(DetailsAccount, |s: &AppState| &s.accounts)),
        DetailsBlockIssuer => Box::new(DetailsView::new(DetailsBlockIssuer, |s: &AppState| {
            &s.block_issuers
        })),
        DetailsDRep => Box::new(DetailsView::new(DetailsDRep, |s: &AppState| &s.dreps)),
        DetailsPool => Box::new(DetailsView::new(DetailsPool, |s: &AppState| &s.pools)),
        DetailsProposal => Box::new(DetailsView::new(DetailsProposal, |s: &AppState| {
            &s.proposals
        })),
        DetailsUtxo => {
            Box::new(OptDetailsView::new(
                DetailsUtxo,
                |s: &AppState| match resolve_placed_widget_id(s, WidgetSlot::List) {
                    ListUtxosByAddr => s
                        .search_query_addr
                        .as_ref()
                        .and_then(|a| s.utxos_by_addr_search_res.get(a)),
                    _ => Some(&s.utxos),
                },
            ))
        }
    }
}

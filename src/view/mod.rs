use crate::{
    app_state::AppState,
    states::WidgetId::{self, *},
    view::{details::DetailsView, empty::EmptyView, list::ListView, tabs::TabsView},
};
use color_eyre::Result;
use ratatui::{Frame, layout::Rect};
use std::collections::HashMap;

pub mod details;
pub mod empty;
pub mod list;
pub mod tabs;

pub type ViewMap = HashMap<WidgetId, Box<dyn View>>;
pub trait View {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()>;
}

pub fn get_views() -> ViewMap {
    let mut views: ViewMap = HashMap::new();

    views.insert(Empty, Box::new(EmptyView {}));

    views.insert(
        CursorTabs,
        Box::new(TabsView {
            widget_id: CursorTabs,
            get_tabs: |s: &AppState| &s.tabs,
        }),
    );

    views.insert(
        ListBrowseOptions,
        Box::new(ListView {
            widget_id: ListBrowseOptions,
            get_list: |s: &AppState| &s.browse_options,
        }),
    );

    views.insert(
        ListSearchOptions,
        Box::new(ListView {
            widget_id: ListSearchOptions,
            get_list: |s: &AppState| &s.search_options,
        }),
    );

    views.insert(
        ListAccounts,
        Box::new(ListView {
            widget_id: ListAccounts,
            get_list: |s: &AppState| &s.accounts,
        }),
    );

    views.insert(
        ListBlockIssuers,
        Box::new(ListView {
            widget_id: ListBlockIssuers,
            get_list: |s: &AppState| &s.block_issuers,
        }),
    );

    views.insert(
        ListDReps,
        Box::new(ListView {
            widget_id: ListDReps,
            get_list: |s: &AppState| &s.dreps,
        }),
    );

    views.insert(
        ListPools,
        Box::new(ListView {
            widget_id: ListPools,
            get_list: |s: &AppState| &s.pools,
        }),
    );

    views.insert(
        ListProposals,
        Box::new(ListView {
            widget_id: ListProposals,
            get_list: |s: &AppState| &s.proposals,
        }),
    );

    views.insert(
        ListUtxos,
        Box::new(ListView {
            widget_id: ListUtxos,
            get_list: |s: &AppState| &s.utxos,
        }),
    );

    views.insert(
        DetailsAccount,
        Box::new(DetailsView {
            widget_id: DetailsAccount,
            get_list: |s: &AppState| &s.accounts,
        }),
    );

    views.insert(
        DetailsBlockIssuer,
        Box::new(DetailsView {
            widget_id: DetailsBlockIssuer,
            get_list: |s: &AppState| &s.block_issuers,
        }),
    );

    views.insert(
        DetailsDRep,
        Box::new(DetailsView {
            widget_id: DetailsDRep,
            get_list: |s: &AppState| &s.dreps,
        }),
    );

    views.insert(
        DetailsPool,
        Box::new(DetailsView {
            widget_id: DetailsPool,
            get_list: |s: &AppState| &s.pools,
        }),
    );

    views.insert(
        DetailsProposal,
        Box::new(DetailsView {
            widget_id: DetailsProposal,
            get_list: |s: &AppState| &s.proposals,
        }),
    );

    views.insert(
        DetailsUtxo,
        Box::new(DetailsView {
            widget_id: DetailsUtxo,
            get_list: |s: &AppState| &s.utxos,
        }),
    );

    views
}

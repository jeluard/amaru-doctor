use crate::{
    app_state::AppState,
    shared::Shared,
    states::WidgetId,
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
    fn render(&self, frame: &mut Frame, area: Rect, app_state: Shared<AppState>) -> Result<()>;
}

pub fn get_views(app_state: Shared<AppState>) -> ViewMap {
    use WidgetId::*;

    let mut views: ViewMap = HashMap::new();

    views.insert(Empty, Box::new(EmptyView {}));

    views.insert(
        CursorTabs,
        Box::new(TabsView {
            widget_id: CursorTabs,
            tabs: app_state.borrow().tabs.clone(),
        }),
    );

    views.insert(
        ListBrowseOptions,
        Box::new(ListView {
            widget_id: ListBrowseOptions,
            list: app_state.borrow().browse_options.clone(),
        }),
    );

    views.insert(
        ListSearchOptions,
        Box::new(ListView {
            widget_id: ListSearchOptions,
            list: app_state.borrow().search_options.clone(),
        }),
    );

    views.insert(
        ListAccounts,
        Box::new(ListView {
            widget_id: ListAccounts,
            list: app_state.borrow().accounts.clone(),
        }),
    );

    views.insert(
        ListBlockIssuers,
        Box::new(ListView {
            widget_id: ListBlockIssuers,
            list: app_state.borrow().block_issuers.clone(),
        }),
    );

    views.insert(
        ListDReps,
        Box::new(ListView {
            widget_id: ListDReps,
            list: app_state.borrow().dreps.clone(),
        }),
    );

    views.insert(
        ListPools,
        Box::new(ListView {
            widget_id: ListPools,
            list: app_state.borrow().pools.clone(),
        }),
    );

    views.insert(
        ListProposals,
        Box::new(ListView {
            widget_id: ListProposals,
            list: app_state.borrow().proposals.clone(),
        }),
    );

    views.insert(
        ListUtxos,
        Box::new(ListView {
            widget_id: ListUtxos,
            list: app_state.borrow().utxos.clone(),
        }),
    );

    views.insert(
        DetailsAccount,
        Box::new(DetailsView {
            widget_id: DetailsAccount,
            list: app_state.borrow().accounts.clone(),
        }),
    );

    views.insert(
        DetailsBlockIssuer,
        Box::new(DetailsView {
            widget_id: DetailsBlockIssuer,
            list: app_state.borrow().block_issuers.clone(),
        }),
    );

    views.insert(
        DetailsDRep,
        Box::new(DetailsView {
            widget_id: DetailsDRep,
            list: app_state.borrow().dreps.clone(),
        }),
    );

    views.insert(
        DetailsPool,
        Box::new(DetailsView {
            widget_id: DetailsPool,
            list: app_state.borrow().pools.clone(),
        }),
    );

    views.insert(
        DetailsProposal,
        Box::new(DetailsView {
            widget_id: DetailsProposal,
            list: app_state.borrow().proposals.clone(),
        }),
    );

    views.insert(
        DetailsUtxo,
        Box::new(DetailsView {
            widget_id: DetailsUtxo,
            list: app_state.borrow().utxos.clone(),
        }),
    );

    views
}

use crate::{
    app_state::AppState,
    states::{Action, WidgetId::*},
    update::{details::DetailsUpdate, focus::FocusUpdate, list::ListUpdate, tabs::TabsUpdate},
};

pub mod details;
pub mod focus;
pub mod list;
pub mod tabs;

pub type UpdateList = Vec<Box<dyn Update>>;
pub trait Update {
    fn update(&self, action: &Action, app_state: &mut AppState);
}

pub fn get_updates() -> UpdateList {
    let updates: UpdateList = vec![
        Box::new(FocusUpdate {
            get_focus: |s: &mut AppState| &mut s.slot_focus,
        }),
        Box::new(TabsUpdate {
            widget_id: CursorTabs,
            get_tabs: |s: &mut AppState| &mut s.tabs,
        }),
        Box::new(ListUpdate {
            widget_id: ListBrowseOptions,
            get_list: |s: &mut AppState| &mut s.browse_options,
        }),
        Box::new(ListUpdate {
            widget_id: ListSearchOptions,
            get_list: |s: &mut AppState| &mut s.search_options,
        }),
        Box::new(ListUpdate {
            widget_id: ListAccounts,
            get_list: |s: &mut AppState| &mut s.accounts,
        }),
        Box::new(ListUpdate {
            widget_id: ListBlockIssuers,
            get_list: |s: &mut AppState| &mut s.block_issuers,
        }),
        Box::new(ListUpdate {
            widget_id: ListDReps,
            get_list: |s: &mut AppState| &mut s.dreps,
        }),
        Box::new(ListUpdate {
            widget_id: ListPools,
            get_list: |s: &mut AppState| &mut s.pools,
        }),
        Box::new(ListUpdate {
            widget_id: ListProposals,
            get_list: |s: &mut AppState| &mut s.proposals,
        }),
        Box::new(ListUpdate {
            widget_id: ListUtxos,
            get_list: |s: &mut AppState| &mut s.utxos,
        }),
        Box::new(DetailsUpdate {
            widget_id: DetailsAccount,
            get_details: |s: &mut AppState| &mut s.accounts,
        }),
        Box::new(DetailsUpdate {
            widget_id: DetailsBlockIssuer,
            get_details: |s: &mut AppState| &mut s.block_issuers,
        }),
        Box::new(DetailsUpdate {
            widget_id: DetailsDRep,
            get_details: |s: &mut AppState| &mut s.dreps,
        }),
        Box::new(DetailsUpdate {
            widget_id: DetailsPool,
            get_details: |s: &mut AppState| &mut s.pools,
        }),
        Box::new(DetailsUpdate {
            widget_id: DetailsProposal,
            get_details: |s: &mut AppState| &mut s.proposals,
        }),
        Box::new(DetailsUpdate {
            widget_id: DetailsUtxo,
            get_details: |s: &mut AppState| &mut s.utxos,
        }),
    ];

    updates
}

use crate::{
    app_state::AppState,
    components::{
        Component, empty::EmptyComponent, fps::FpsCounter, group::layout::LayoutComponent,
        list::ListComponent, list_and_details::new_list_detail_components, message::Message,
        tab::TabComponent,
    },
    shared::{Shared, SharedComp, shared},
    states::{BrowseOptions, Tab, WidgetId},
};
use ratatui::layout::{Constraint, Direction};
use std::collections::HashMap;

pub fn build_widget_map(app_state: Shared<AppState>) -> HashMap<WidgetId, SharedComp> {
    let mut map = HashMap::<WidgetId, SharedComp>::new();

    map.insert(WidgetId::Empty, shared(EmptyComponent::default()));

    map.insert(
        WidgetId::ListTabs,
        shared(TabComponent::new(
            WidgetId::ListTabs,
            app_state.borrow().tabs.clone(),
            app_state.clone(),
        )),
    );

    map.insert(
        WidgetId::ListBrowseOptions,
        shared(ListComponent::from_iter(
            WidgetId::ListBrowseOptions,
            app_state.borrow().browse_options.clone(),
            app_state.clone(),
        )),
    );

    map.insert(
        WidgetId::ListSearchOptions,
        shared(ListComponent::from_iter(
            WidgetId::ListSearchOptions,
            app_state.borrow().search_options.clone(),
            app_state.clone(),
        )),
    );

    let (accounts, account_details) = new_list_detail_components(
        WidgetId::ListAccounts,
        WidgetId::DetailAccount,
        app_state.borrow().accounts.clone(),
        app_state.clone(),
    );
    let (block_issuers, block_issuer_details) = new_list_detail_components(
        WidgetId::ListBlockIssuers,
        WidgetId::DetailBlockIssuer,
        app_state.borrow().block_issuers.clone(),
        app_state.clone(),
    );
    let (dreps, drep_details) = new_list_detail_components(
        WidgetId::ListDReps,
        WidgetId::DetailDRep,
        app_state.borrow().dreps.clone(),
        app_state.clone(),
    );
    let (pools, pool_details) = new_list_detail_components(
        WidgetId::ListPools,
        WidgetId::DetailPool,
        app_state.borrow().pools.clone(),
        app_state.clone(),
    );
    let (proposals, proposal_details) = new_list_detail_components(
        WidgetId::ListProposals,
        WidgetId::DetailProposal,
        app_state.borrow().proposals.clone(),
        app_state.clone(),
    );
    let (utxos, utxo_details) = new_list_detail_components(
        WidgetId::ListUtxos,
        WidgetId::DetailUtxo,
        app_state.borrow().utxos.clone(),
        app_state.clone(),
    );

    map.insert(WidgetId::ListAccounts, shared(accounts));
    map.insert(WidgetId::ListBlockIssuers, shared(block_issuers));
    map.insert(WidgetId::ListDReps, shared(dreps));
    map.insert(WidgetId::ListPools, shared(pools));
    map.insert(WidgetId::ListProposals, shared(proposals));
    map.insert(WidgetId::ListUtxos, shared(utxos));

    map.insert(WidgetId::DetailAccount, shared(account_details));
    map.insert(WidgetId::DetailBlockIssuer, shared(block_issuer_details));
    map.insert(WidgetId::DetailDRep, shared(drep_details));
    map.insert(WidgetId::DetailPool, shared(pool_details));
    map.insert(WidgetId::DetailProposal, shared(proposal_details));
    map.insert(WidgetId::DetailUtxo, shared(utxo_details));

    map
}

pub fn resolve_layout_widgets(
    app_state: Shared<AppState>,
) -> (WidgetId, WidgetId, WidgetId, WidgetId) {
    let nav = WidgetId::ListTabs;

    let options = match app_state.borrow().tabs.borrow().current() {
        Some(Tab::Browse) => WidgetId::ListBrowseOptions,
        Some(Tab::Search) => WidgetId::ListSearchOptions,
        None => WidgetId::Empty,
    };

    let (list, detail) = match app_state.borrow().tabs.borrow().current() {
        Some(Tab::Browse) => match app_state.borrow().browse_options.borrow().selected() {
            Some(BrowseOptions::Accounts) => (WidgetId::ListAccounts, WidgetId::DetailAccount),
            Some(BrowseOptions::BlockIssuers) => {
                (WidgetId::ListBlockIssuers, WidgetId::DetailBlockIssuer)
            }
            Some(BrowseOptions::DReps) => (WidgetId::ListDReps, WidgetId::DetailDRep),
            Some(BrowseOptions::Pools) => (WidgetId::ListPools, WidgetId::DetailPool),
            Some(BrowseOptions::Proposals) => (WidgetId::ListProposals, WidgetId::DetailProposal),
            Some(BrowseOptions::Utxos) => (WidgetId::ListUtxos, WidgetId::DetailUtxo),
            _ => (WidgetId::Empty, WidgetId::Empty),
        },
        // TODO: Reintroduce search
        Some(Tab::Search) => (WidgetId::Empty, WidgetId::Empty),
        None => (WidgetId::Empty, WidgetId::Empty),
    };

    (nav, options, list, detail)
}

pub fn make_header(ledger_path_str: &String) -> Shared<dyn Component> {
    shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (
                Constraint::Length(1),
                shared(Message::new(
                    None,
                    format!("Reading amaru ledger at {:?}", ledger_path_str),
                )),
            ),
            (Constraint::Length(1), shared(FpsCounter::default())),
        ],
    ))
}

pub fn make_body(
    nav: WidgetId,
    options: WidgetId,
    list: WidgetId,
    detail: WidgetId,
    widget_map: &HashMap<WidgetId, SharedComp>,
) -> SharedComp {
    let nav = widget_map
        .get(&nav)
        .unwrap_or(&widget_map[&WidgetId::Empty])
        .clone();
    let options = widget_map
        .get(&options)
        .unwrap_or(&widget_map[&WidgetId::Empty])
        .clone();
    let list = widget_map
        .get(&list)
        .unwrap_or(&widget_map[&WidgetId::Empty])
        .clone();
    let detail = widget_map
        .get(&detail)
        .unwrap_or(&widget_map[&WidgetId::Empty])
        .clone();

    let left_column = shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (Constraint::Length(3), nav),
            (Constraint::Length(8), options),
            (Constraint::Fill(1), list),
        ],
    ));

    let right_column = shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (Constraint::Length(3), shared(EmptyComponent::default())),
            (Constraint::Fill(1), detail),
        ],
    ));

    shared(LayoutComponent::new(
        Direction::Horizontal,
        vec![
            (Constraint::Percentage(20), left_column),
            (Constraint::Percentage(80), right_column),
        ],
    ))
}

pub fn make_footer() -> Shared<dyn Component> {
    shared(LayoutComponent::new(
        Direction::Vertical,
        vec![(
            Constraint::Length(1),
            shared(Message::new(
                None,
                "Use Shift + Arrow keys to move focus. Use Arrow keys to scroll.".to_string(),
            )),
        )],
    ))
}

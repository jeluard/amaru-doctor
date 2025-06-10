use crate::{
    app::AppComponents,
    app_state::AppState,
    components::{
        Component, empty::EmptyComponent, fps::FpsCounter, group::layout::LayoutComponent,
        list::ListComponent, list_and_details::new_list_detail_components, message::Message,
        tab::TabComponent,
    },
    shared::{Shared, SharedComp, shared},
    states::{EntityOptions, Nav, SlotSelection},
    ui::to_list_item::UtxoItem,
    window::WindowState,
};
use ratatui::layout::{Constraint, Direction};
use std::{iter, rc::Rc};

pub fn build_layout(ledger_path_str: &String, app_state: Shared<AppState>) -> AppComponents {
    let body_components = make_lists(app_state.clone());
    let header = make_header(ledger_path_str);
    let body = make_body(app_state.clone(), body_components);
    let footer = make_footer();

    AppComponents::new(vec![header, body, footer])
}

#[derive(Clone)]
pub struct NavSlot {
    pub nav: SharedComp,
}

#[derive(Clone)]
pub struct NavTypesSlot {
    pub browse: SharedComp,
    pub searches: SharedComp,
}

#[derive(Clone)]
pub struct ListSlot {
    pub accounts: SharedComp,
    pub block_issuers: SharedComp,
    pub dreps: SharedComp,
    pub pools: SharedComp,
    pub proposals: SharedComp,
    pub utxos: SharedComp,
}

#[derive(Clone)]
pub struct DetailsSlot {
    pub account_details: SharedComp,
    pub block_issuer_details: SharedComp,
    pub drep_details: SharedComp,
    pub pool_details: SharedComp,
    pub proposal_details: SharedComp,
    pub utxo_details: SharedComp,
}

#[derive(Clone)]
pub struct BodyComponents {
    pub nav_slot: NavSlot,
    pub nav_types_slot: NavTypesSlot,
    pub list_slot: ListSlot,
    pub details_slot: DetailsSlot,
}

fn make_lists(app_state: Shared<AppState>) -> Shared<BodyComponents> {
    let nav = shared(TabComponent::new(
        SlotSelection::Nav,
        app_state.borrow().nav.clone(),
        app_state.clone(),
    ));
    let browse = shared(ListComponent::from_iter(
        SlotSelection::NavTypeBrowse,
        app_state.borrow().entity_list.clone(),
        app_state.clone(),
    ));
    let searches = shared(ListComponent::from_iter(
        SlotSelection::NavTypeSearch,
        app_state.borrow().search_list.clone(),
        app_state.clone(),
    ));
    // let search_query = shared(EmptyComponent::default()); //shared(SearchComponent::new("Search".to_string()));
    // let search_components: Vec<(Nav, SharedFC)> = vec![
    //     (
    //         Nav::Browse,
    //         shared(Message::new(
    //             Some("Note".to_string()),
    //             "Search may take time while an index builds, please be patient.".to_string(),
    //         )),
    //     ),
    //     (Nav::Search, search_query.clone()),
    // ];
    // let search_switcher = shared(SwitchComponent::new(
    //     app_state.nav.clone(),
    //     search_components,
    // ));

    let (accounts, account_details) = new_list_detail_components(
        SlotSelection::BrowseAccounts,
        SlotSelection::DetailAccount,
        app_state.borrow().account_list.clone(),
        app_state.clone(),
    );
    let (block_issuers, block_issuer_details) = new_list_detail_components(
        SlotSelection::BrowseBlockIssuers,
        SlotSelection::DetailBlockIssuer,
        app_state.borrow().block_issuer_list.clone(),
        app_state.clone(),
    );
    let (dreps, drep_details) = new_list_detail_components(
        SlotSelection::BrowseDReps,
        SlotSelection::DetailDRep,
        app_state.borrow().drep_list.clone(),
        app_state.clone(),
    );
    let (pools, pool_details) = new_list_detail_components(
        SlotSelection::BrowsePools,
        SlotSelection::DetailPool,
        app_state.borrow().pool_list.clone(),
        app_state.clone(),
    );
    let (proposals, proposal_details) = new_list_detail_components(
        SlotSelection::BrowseProposals,
        SlotSelection::DetailProposal,
        app_state.borrow().proposal_list.clone(),
        app_state.clone(),
    );
    let (utxos, utxo_details) = new_list_detail_components(
        SlotSelection::BrowseUtxos,
        SlotSelection::DetailUtxo,
        app_state.borrow().utxo_list.clone(),
        app_state.clone(),
    );

    // let entity_ids_switcher = shared(SwitchComponent::new(
    //     app_state.entity_list.clone(),
    //     vec![
    //         (Entity::Accounts, shared(accounts)),
    //         (Entity::BlockIssuers, shared(block_issuers)),
    //         (Entity::DReps, shared(dreps)),
    //         (Entity::Pools, shared(pools)),
    //         (Entity::Proposals, shared(proposals)),
    //         (Entity::UTXOs, shared(utxos)),
    //     ],
    // ));
    let search_results = shared(EmptyComponent::default()); // shared(SearchResultComponent::new(
    //     db.clone(),
    //     search_types.clone(),
    //     search_query.clone(),
    // ));

    // let nav_list_switcher = shared(SwitchComponent::new(
    //     app_state.nav.clone(),
    //     vec![
    //         (Nav::Browse, entity_ids_switcher),
    //         (Nav::Search, search_results.clone()),
    //     ],
    // ));

    // let entity_details_switcher = shared(SwitchComponent::new(
    //     app_state.entity_list.clone(),
    //     vec![
    //         (Entity::Accounts, shared(account_details)),
    //         (Entity::BlockIssuers, shared(block_issuer_details)),
    //         (Entity::DReps, shared(drep_details)),
    //         (Entity::Pools, shared(pool_details)),
    //         (Entity::Proposals, shared(proposal_details)),
    //         (Entity::UTXOs, shared(utxo_details)),
    //     ],
    // ));

    let empty_window_state = WindowState::new(Box::new(iter::empty::<UtxoItem>()));
    // let search_details = shared(DetailsComponent::new(
    //     "Search Details".to_string(),
    //     shared(empty_window_state),
    // ));
    // let details_switcher = shared(SwitchComponent::new(
    //     app_state.nav.clone(),
    //     vec![
    //         (Nav::Browse, entity_details_switcher),
    //         (Nav::Search, search_details.clone()),
    //     ],
    // ));

    // let nav_components: Vec<(Nav, SharedComp)> = vec![
    //     (Nav::Browse, browse.clone()),
    //     (Nav::Search, searches.clone()),
    // ];
    // let nav_types_switcher = shared(SwitchComponent::new(app_state.nav.clone(), nav_components));

    shared(BodyComponents {
        nav_slot: NavSlot { nav },
        nav_types_slot: NavTypesSlot { browse, searches },
        list_slot: ListSlot {
            accounts: shared(accounts),
            block_issuers: shared(block_issuers),
            dreps: shared(dreps),
            pools: shared(pools),
            proposals: shared(proposals),
            utxos: shared(utxos),
        },
        details_slot: DetailsSlot {
            account_details: shared(account_details),
            block_issuer_details: shared(block_issuer_details),
            drep_details: shared(drep_details),
            pool_details: shared(pool_details),
            proposal_details: shared(proposal_details),
            utxo_details: shared(utxo_details),
        },
    })
}

fn make_header(ledger_path_str: &String) -> Shared<dyn Component> {
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

fn make_body(
    app_state: Shared<AppState>,
    body_comps: Shared<BodyComponents>,
) -> Shared<dyn Component> {
    let body_brw = body_comps.borrow();
    let nav_type_slot_comp = match app_state.borrow().nav.borrow().current() {
        Some(Nav::Browse) => body_brw.nav_types_slot.browse.clone(),
        Some(Nav::Search) => body_brw.nav_types_slot.searches.clone(),
        None => shared(EmptyComponent::default()),
    };
    let list_slot_comp = match app_state.borrow().nav.borrow().current() {
        Some(Nav::Browse) => match app_state.borrow().entity_list.borrow().selected() {
            Some(EntityOptions::Accounts) => body_brw.list_slot.accounts.clone(),
            Some(EntityOptions::BlockIssuers) => body_brw.list_slot.block_issuers.clone(),
            Some(EntityOptions::DReps) => body_brw.list_slot.block_issuers.clone(),
            Some(EntityOptions::Pools) => body_brw.list_slot.block_issuers.clone(),
            Some(EntityOptions::Proposals) => body_brw.list_slot.block_issuers.clone(),
            Some(EntityOptions::Utxos) => body_brw.list_slot.block_issuers.clone(),
            _ => shared(EmptyComponent::default()),
        },
        Some(Nav::Search) => body_comps.borrow().nav_types_slot.searches.clone(),
        None => shared(EmptyComponent::default()),
    };
    let left_column = shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (Constraint::Length(3), body_brw.nav_slot.nav.clone()),
            (Constraint::Length(8), nav_type_slot_comp),
            (Constraint::Fill(1), list_slot_comp),
        ],
    ));
    let details_slot_comp = match app_state.borrow().nav.borrow().current() {
        Some(Nav::Browse) => match app_state.borrow().entity_list.borrow().selected() {
            Some(EntityOptions::Accounts) => body_brw.details_slot.account_details.clone(),
            Some(EntityOptions::BlockIssuers) => body_brw.details_slot.block_issuer_details.clone(),
            Some(EntityOptions::DReps) => body_brw.details_slot.drep_details.clone(),
            Some(EntityOptions::Pools) => body_brw.details_slot.pool_details.clone(),
            Some(EntityOptions::Proposals) => body_brw.details_slot.proposal_details.clone(),
            Some(EntityOptions::Utxos) => body_brw.details_slot.utxo_details.clone(),
            _ => shared(EmptyComponent::default()),
        },
        Some(Nav::Search) => body_comps.borrow().nav_types_slot.searches.clone(),
        None => shared(EmptyComponent::default()),
    };
    let right_column = shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (Constraint::Length(3), shared(EmptyComponent::default())),
            (Constraint::Fill(1), details_slot_comp),
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

fn make_footer() -> Shared<dyn Component> {
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

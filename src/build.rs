use crate::{
    app::AppComponents,
    components::{
        Component,
        details::DetailsComponent,
        fps::FpsCounter,
        group::{layout::LayoutComponent, switch::SwitchComponent},
        list::ListComponent,
        list_and_details::new_list_detail_components,
        message::Message,
        search::SearchComponent,
        search_result::SearchResultComponent,
        r#static::{entity_types::Entity, search_types::SearchType},
        tab::TabComponent,
    },
    focus::FocusManager,
    shared::{Shared, SharedFC, shared},
    states::NavMode,
    store::owned_iter::{
        OwnedAccountsIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter, OwnedProposalIter,
        OwnedUtxoIter,
    },
    store::rocks_db_switch::RocksDBSwitch,
};
use color_eyre::Result;
use ratatui::layout::{Constraint, Direction};
use std::sync::Arc;
use strum::IntoEnumIterator;

pub fn build_layout(
    ledger_path_str: &String,
    db: Arc<RocksDBSwitch>,
) -> Result<(AppComponents, FocusManager)> {
    let body_components = make_lists(db);
    let header = make_header(ledger_path_str);
    let body = make_body(body_components.clone());
    let footer = make_footer();

    let layout = AppComponents::new(body_components.clone(), vec![header, body, footer]);
    let focus = FocusManager::new(body_components.borrow().clone().into());

    Ok((layout, focus))
}

#[derive(Clone)]
pub struct BodyComponents {
    nav_tabs: SharedFC,
    nav_switcher: SharedFC,
    nav_list_switcher: SharedFC,
    search_switcher: SharedFC,
    details_switcher: SharedFC,
}

impl From<BodyComponents> for Vec<SharedFC> {
    fn from(val: BodyComponents) -> Self {
        vec![
            val.nav_tabs,
            val.nav_switcher,
            val.nav_list_switcher,
            val.search_switcher,
            val.details_switcher,
        ]
    }
}

fn make_lists(db: Arc<RocksDBSwitch>) -> Shared<BodyComponents> {
    let nav_tabs = shared(TabComponent::new(
        "Nav Mode".to_string(),
        NavMode::iter().collect(),
    ));
    let entity_types = shared(ListComponent::from_iter(
        "Entity Types".to_string(),
        Box::new(Entity::iter()),
    ));
    let search_types = shared(ListComponent::from_iter(
        "Search Types".to_string(),
        Box::new(SearchType::iter()),
    ));
    let search_query = shared(SearchComponent::new("Search".to_string()));
    let search_components: Vec<(NavMode, SharedFC)> = vec![
        (
            NavMode::Browse,
            shared(Message::new(
                Some("Note".to_string()),
                "Search may take time while an index builds, please be patient.".to_string(),
            )),
        ),
        (NavMode::Search, search_query.clone()),
    ];
    let search_switcher = shared(SwitchComponent::new(nav_tabs.clone(), search_components));

    let (accounts, account_details) =
        new_list_detail_components("Account", OwnedAccountsIter::new(db.clone()));
    let (block_issuers, block_issuer_details) =
        new_list_detail_components("Block Issuer", OwnedBlockIssuerIter::new(db.clone()));
    let (dreps, drep_details) = new_list_detail_components("DRep", OwnedDRepIter::new(db.clone()));
    let (pools, pool_details) = new_list_detail_components("Pool", OwnedPoolIter::new(db.clone()));
    let (proposals, proposal_details) =
        new_list_detail_components("Proposal", OwnedProposalIter::new(db.clone()));
    let (utxos, utxo_details) = new_list_detail_components("UTXO", OwnedUtxoIter::new(db.clone()));

    let entity_ids_switcher = shared(SwitchComponent::new(
        entity_types.clone(),
        vec![
            (Entity::Accounts, accounts),
            (Entity::BlockIssuers, block_issuers),
            (Entity::DReps, dreps),
            (Entity::Pools, pools),
            (Entity::Proposals, proposals),
            (Entity::UTXOs, utxos),
        ],
    ));
    let search_results = shared(SearchResultComponent::new(
        db.clone(),
        search_types.clone(),
        search_query.clone(),
    ));

    let nav_list_switcher = shared(SwitchComponent::new(
        nav_tabs.clone(),
        vec![
            (NavMode::Browse, entity_ids_switcher),
            (NavMode::Search, search_results.clone()),
        ],
    ));

    let entity_details_switcher = shared(SwitchComponent::new(
        entity_types.clone(),
        vec![
            (Entity::Accounts, account_details),
            (Entity::BlockIssuers, block_issuer_details),
            (Entity::DReps, drep_details),
            (Entity::Pools, pool_details),
            (Entity::Proposals, proposal_details),
            (Entity::UTXOs, utxo_details),
        ],
    ));

    let search_details = shared(DetailsComponent::new(
        "Search Details".to_string(),
        search_results.clone(),
    ));
    let details_switcher = shared(SwitchComponent::new(
        nav_tabs.clone(),
        vec![
            (NavMode::Browse, entity_details_switcher),
            (NavMode::Search, search_details.clone()),
        ],
    ));

    let nav_components: Vec<(NavMode, SharedFC)> = vec![
        (NavMode::Browse, entity_types),
        (NavMode::Search, search_types.clone()),
    ];
    let nav_types_switcher = shared(SwitchComponent::new(nav_tabs.clone(), nav_components));

    shared(BodyComponents {
        nav_tabs,
        nav_switcher: nav_types_switcher,
        nav_list_switcher,
        search_switcher,
        details_switcher,
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

fn make_body(body_comps: Shared<BodyComponents>) -> Shared<dyn Component> {
    let left_column = shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (Constraint::Length(3), body_comps.borrow().nav_tabs.clone()),
            (
                Constraint::Length(8),
                body_comps.borrow().nav_switcher.clone(),
            ),
            (
                Constraint::Fill(1),
                body_comps.borrow().nav_list_switcher.clone(),
            ),
        ],
    ));

    let right_column = shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (
                Constraint::Length(3),
                body_comps.borrow().search_switcher.clone(),
            ),
            (
                Constraint::Fill(1),
                body_comps.borrow().details_switcher.clone(),
            ),
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

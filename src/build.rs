use crate::{
    app::{self, AppComponents},
    app_model::AppModel,
    components::{
        Component,
        details::DetailsComponent,
        empty::EmptyComponent,
        fps::FpsCounter,
        group::{layout::LayoutComponent, switch::SwitchComponent},
        list::ListComponent,
        list_and_details::new_list_detail_components,
        message::Message,
        search::SearchComponent,
        r#static::{entity_types::Entity, search_types::Search},
        tab::TabComponent,
    },
    focus::FocusManager,
    shared::{Shared, SharedFC, shared},
    states::NavMode,
    store::rocks_db_switch::RocksDBSwitch,
    ui::to_list_item::UtxoItem,
    window::WindowState,
};
use color_eyre::Result;
use ratatui::layout::{Constraint, Direction};
use std::{iter, sync::Arc};
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
    let app_model = AppModel::new(db);
    let nav_tabs = shared(TabComponent::new(
        "Nav Mode".to_string(),
        NavMode::iter().collect(),
    ));
    let entity_types = shared(ListComponent::from_iter(
        Entity::Entites,
        app_model.entity_list.clone(),
    ));
    let search_types = shared(ListComponent::from_iter(
        Entity::SearchTypes,
        app_model.search_list,
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
        new_list_detail_components(Entity::Accounts, app_model.account_list);
    let (block_issuers, block_issuer_details) =
        new_list_detail_components(Entity::BlockIssuers, app_model.block_issuer_list);
    let (dreps, drep_details) = new_list_detail_components(Entity::DReps, app_model.drep_list);
    let (pools, pool_details) = new_list_detail_components(Entity::Pools, app_model.pool_list);
    let (proposals, proposal_details) =
        new_list_detail_components(Entity::Proposals, app_model.proposal_list);
    let (utxos, utxo_details) = new_list_detail_components(Entity::UTXOs, app_model.utxo_list);

    let entity_ids_switcher = shared(SwitchComponent::new(
        app_model.entity_list.clone(),
        vec![
            (Entity::Accounts, shared(accounts)),
            (Entity::BlockIssuers, shared(block_issuers)),
            (Entity::DReps, shared(dreps)),
            (Entity::Pools, shared(pools)),
            (Entity::Proposals, shared(proposals)),
            (Entity::UTXOs, shared(utxos)),
        ],
    ));
    let search_results = shared(EmptyComponent::default()); // shared(SearchResultComponent::new(
    //     db.clone(),
    //     search_types.clone(),
    //     search_query.clone(),
    // ));

    let nav_list_switcher = shared(SwitchComponent::new(
        nav_tabs.clone(),
        vec![
            (NavMode::Browse, entity_ids_switcher),
            (NavMode::Search, search_results.clone()),
        ],
    ));

    let entity_details_switcher = shared(SwitchComponent::new(
        app_model.entity_list,
        vec![
            (Entity::Accounts, shared(account_details)),
            (Entity::BlockIssuers, shared(block_issuer_details)),
            (Entity::DReps, shared(drep_details)),
            (Entity::Pools, shared(pool_details)),
            (Entity::Proposals, shared(proposal_details)),
            (Entity::UTXOs, shared(utxo_details)),
        ],
    ));

    let empty_window_state = WindowState::new(Box::new(iter::empty::<UtxoItem>()));
    let search_details = shared(DetailsComponent::new(
        "Search Details".to_string(),
        shared(empty_window_state),
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

use crate::{
    app::AppComponents,
    components::{
        Component,
        fps::FpsCounter,
        group::{layout::LayoutComponent, switch::SwitchComponent},
        list_and_details::new_list_detail_components,
        message::Message,
        search::SearchComponent,
        search_result::SearchResultComponent,
        r#static::{
            entity_types::{Entity, new_entity_types_list},
            search_types::new_search_types_list,
        },
        tab::TabComponent,
    },
    focus::{FocusManager, FocusableComponent},
    nav::NavMode,
    shared::{Shared, shared},
};
use amaru_ledger::store::ReadOnlyStore;
use color_eyre::Result;
use ratatui::layout::{Constraint, Direction};
use std::sync::Arc;

type SharedFC<'a> = Shared<'a, dyn FocusableComponent + 'a>;

pub fn build_layout<'a>(
    ledger_path_str: &String,
    db: &'a Arc<impl ReadOnlyStore>,
) -> Result<(AppComponents<'a>, FocusManager<'a>)> {
    let body_components = make_lists(db);
    let header = make_header(ledger_path_str);
    let body = make_body(&body_components);
    let footer = make_footer();

    let layout = AppComponents::new(vec![header, body, footer]);
    let focus = FocusManager::new(body_components.into());

    Ok((layout, focus))
}

struct BodyComponents<'a> {
    nav_tabs: SharedFC<'a>,
    nav_switcher: SharedFC<'a>,
    entity_ids_switcher: SharedFC<'a>,
    search_switcher: SharedFC<'a>,
    entity_details_switcher: SharedFC<'a>,
}

impl<'a> From<BodyComponents<'a>> for Vec<SharedFC<'a>> {
    fn from(val: BodyComponents<'a>) -> Self {
        vec![
            val.nav_tabs,
            val.nav_switcher,
            val.entity_ids_switcher,
            val.search_switcher,
            val.entity_details_switcher,
        ]
    }
}

fn make_lists<'a>(db: &'a Arc<impl ReadOnlyStore>) -> BodyComponents<'a> {
    let nav_tabs = shared(TabComponent::new(
        "Nav Mode",
        vec![NavMode::Browse, NavMode::Search],
    ));
    let entity_types = shared(new_entity_types_list());
    let search_types = shared(new_search_types_list());
    let search_query = shared(SearchComponent::new("Search".to_string()));
    let search_components: Vec<(NavMode, SharedFC<'a>)> = vec![
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
    let nav_components: Vec<(NavMode, SharedFC<'a>)> = vec![
        (NavMode::Browse, entity_types.clone()),
        (NavMode::Search, search_types.clone()),
    ];
    let nav_types_switcher = shared(SwitchComponent::new(nav_tabs.clone(), nav_components));

    let (accounts, account_details) =
        new_list_detail_components("Account", db.iter_accounts().unwrap());
    let (block_issuers, block_issuer_details) =
        new_list_detail_components("Block Issuer", db.iter_block_issuers().unwrap());
    let (dreps, drep_details) = new_list_detail_components("DRep", db.iter_dreps().unwrap());
    let (pools, pool_details) = new_list_detail_components("Pool", db.iter_pools().unwrap());
    let (proposals, proposal_details) =
        new_list_detail_components("Proposal", db.iter_proposals().unwrap());
    let (utxos, utxo_details) = new_list_detail_components("UTXO", db.iter_utxos().unwrap());

    let entity_id_components: Vec<(Entity, Shared<dyn FocusableComponent>)> = vec![
        (Entity::Accounts, accounts),
        (Entity::BlockIssuers, block_issuers),
        (Entity::DReps, dreps),
        (Entity::Pools, pools),
        (Entity::Proposals, proposals),
        (Entity::UTXOs, utxos),
    ];
    let entity_ids_switcher = shared(SwitchComponent::new(
        entity_types.clone(),
        entity_id_components,
    ));
    let search_results = shared(SearchResultComponent::new(
        db,
        search_types.clone(),
        search_query.clone(),
    ));
    let list_components: Vec<(NavMode, Shared<dyn FocusableComponent>)> = vec![
        (NavMode::Browse, entity_ids_switcher.clone()),
        (NavMode::Search, search_results.clone()),
    ];
    let nav_list_switcher = shared(SwitchComponent::new(nav_tabs.clone(), list_components));

    let entity_detail_components: Vec<(Entity, Shared<dyn FocusableComponent>)> = vec![
        (Entity::Accounts, account_details),
        (Entity::BlockIssuers, block_issuer_details),
        (Entity::DReps, drep_details),
        (Entity::Pools, pool_details),
        (Entity::Proposals, proposal_details),
        (Entity::UTXOs, utxo_details),
    ];
    let entity_details_switcher = shared(SwitchComponent::new(
        entity_types.clone(),
        entity_detail_components,
    ));

    BodyComponents {
        nav_tabs,
        nav_switcher: nav_types_switcher,
        entity_ids_switcher: nav_list_switcher,
        search_switcher,
        entity_details_switcher,
    }
}

fn make_header<'a>(ledger_path_str: &String) -> Shared<'a, dyn Component + 'a> {
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

fn make_body<'a>(
    BodyComponents {
        nav_tabs,
        nav_switcher,
        entity_ids_switcher,
        search_switcher,
        entity_details_switcher,
    }: &BodyComponents<'a>,
) -> Shared<'a, dyn Component + 'a> {
    let left_column = shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (Constraint::Length(3), nav_tabs.clone()),
            (Constraint::Length(8), nav_switcher.clone()),
            (Constraint::Fill(1), entity_ids_switcher.clone()),
        ],
    ));

    let right_column = shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (Constraint::Length(3), search_switcher.clone()),
            (Constraint::Fill(1), entity_details_switcher.clone()),
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

fn make_footer<'a>() -> Shared<'a, dyn Component> {
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

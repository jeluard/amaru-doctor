use crate::{
    action::Entity,
    app::AppComponents,
    components::{
        Component,
        entity_types::new_entity_types_list,
        fps::FpsCounter,
        group::{layout::LayoutComponent, switch::SwitchComponent},
        list_and_details::new_list_detail_components,
        message::Message,
    },
    focus::{FocusManager, FocusableComponent},
    shared::{Shared, shared},
};
use amaru_ledger::store::ReadOnlyStore;
use color_eyre::Result;
use ratatui::layout::{Constraint, Direction};
use std::{collections::HashMap, sync::Arc};

pub fn build_layout<'a>(
    ledger_path_str: &String,
    db: &'a Arc<impl ReadOnlyStore>,
) -> Result<(AppComponents<'a>, FocusManager<'a>)> {
    let (entity_types, ids_switcher, details_switcher) = make_entity_lists(db);
    let header = make_header(ledger_path_str);
    let body = make_body(
        entity_types.clone(),
        ids_switcher.clone(),
        details_switcher.clone(),
    );
    let footer = make_footer();

    let layout = AppComponents::new(vec![header, body, footer]);
    let focus = FocusManager::new(vec![entity_types, ids_switcher, details_switcher]);

    Ok((layout, focus))
}

fn make_entity_lists<'a>(
    db: &'a Arc<impl ReadOnlyStore>,
) -> (
    Shared<'a, dyn FocusableComponent + 'a>,
    Shared<'a, dyn FocusableComponent + 'a>,
    Shared<'a, dyn FocusableComponent + 'a>,
) {
    let entity_types = shared(new_entity_types_list());

    let (accounts, account_details) =
        new_list_detail_components("Account", db.iter_accounts().unwrap());
    let (block_issuers, block_issuer_details) =
        new_list_detail_components("Block Issuer", db.iter_block_issuers().unwrap());
    let (dreps, drep_details) = new_list_detail_components("DRep", db.iter_dreps().unwrap());
    let (pools, pool_details) = new_list_detail_components("Pool", db.iter_pools().unwrap());
    let (proposals, proposal_details) =
        new_list_detail_components("Proposal", db.iter_proposals().unwrap());
    let (utxos, utxo_details) = new_list_detail_components("UTXO", db.iter_utxos().unwrap());

    let mut id_components: HashMap<Entity, Shared<dyn FocusableComponent>> = HashMap::new();
    id_components.insert(Entity::Accounts, accounts);
    id_components.insert(Entity::BlockIssuers, block_issuers);
    id_components.insert(Entity::DReps, dreps);
    id_components.insert(Entity::Pools, pools);
    id_components.insert(Entity::Proposals, proposals);
    id_components.insert(Entity::UTXOs, utxos);

    let mut detail_components: HashMap<Entity, Shared<dyn FocusableComponent>> = HashMap::new();
    detail_components.insert(Entity::Accounts, account_details);
    detail_components.insert(Entity::BlockIssuers, block_issuer_details);
    detail_components.insert(Entity::DReps, drep_details);
    detail_components.insert(Entity::Proposals, proposal_details);
    detail_components.insert(Entity::Pools, pool_details);
    detail_components.insert(Entity::UTXOs, utxo_details);

    let ids_switcher = shared(SwitchComponent::new(entity_types.clone(), id_components));
    let details_switcher = shared(SwitchComponent::new(
        entity_types.clone(),
        detail_components,
    ));

    (entity_types, ids_switcher, details_switcher)
}

fn make_header<'a>(ledger_path_str: &String) -> Shared<'a, dyn Component + 'a> {
    shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (
                Constraint::Length(1),
                shared(Message::new(format!(
                    "Reading amaru ledger at {:?}",
                    ledger_path_str
                ))),
            ),
            (Constraint::Length(1), shared(FpsCounter::default())),
        ],
    ))
}

fn make_body<'a>(
    entity_types: Shared<'a, dyn Component + 'a>,
    ids_switcher: Shared<'a, dyn Component + 'a>,
    details_switcher: Shared<'a, dyn Component + 'a>,
) -> Shared<'a, dyn Component + 'a> {
    let left_column = shared(LayoutComponent::new(
        Direction::Vertical,
        vec![
            (Constraint::Percentage(50), entity_types),
            (Constraint::Percentage(50), ids_switcher),
        ],
    ));

    shared(LayoutComponent::new(
        Direction::Horizontal,
        vec![
            (Constraint::Percentage(30), left_column),
            (Constraint::Percentage(70), details_switcher),
        ],
    ))
}

fn make_footer<'a>() -> Shared<'a, dyn Component> {
    shared(LayoutComponent::new(
        Direction::Vertical,
        vec![(
            Constraint::Length(1),
            shared(Message::new(
                "Use Shift + Arrow keys to move focus...".to_string(),
            )),
        )],
    ))
}

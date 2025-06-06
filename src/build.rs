use crate::{
    action::Entity,
    components::{
        entity_types::new_entity_types_list,
        fps::FpsCounter,
        group::{
            ComponentGroup,
            layout::RootLayout,
            split::{Axis, SplitComponent},
            switch::SwitchComponent,
        },
        list_and_details::new_list_detail_components,
        message::Message,
    },
    focus::{FocusManager, FocusableComponent},
    shared::{Shared, shared},
};
use amaru_ledger::store::ReadOnlyStore;
use color_eyre::Result;
use std::{collections::HashMap, sync::Arc};

pub fn build_layout<'a>(
    ledger_path_str: &String,
    db: &'a Arc<impl ReadOnlyStore>,
) -> Result<(RootLayout<'a>, FocusManager<'a>)> {
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
    let mut entity_id_components: HashMap<Entity, Shared<dyn FocusableComponent>> = HashMap::new();
    entity_id_components.insert(Entity::Accounts, accounts);
    entity_id_components.insert(Entity::BlockIssuers, block_issuers);
    entity_id_components.insert(Entity::DReps, dreps);
    entity_id_components.insert(Entity::Pools, pools);
    entity_id_components.insert(Entity::Proposals, proposals);
    entity_id_components.insert(Entity::UTXOs, utxos);
    let entity_ids_switcher = shared(SwitchComponent::new(
        entity_types.clone(),
        entity_id_components,
    ));

    let mut entity_detail_components: HashMap<Entity, Shared<dyn FocusableComponent>> =
        HashMap::new();
    entity_detail_components.insert(Entity::Accounts, account_details);
    entity_detail_components.insert(Entity::BlockIssuers, block_issuer_details);
    entity_detail_components.insert(Entity::DReps, drep_details);
    entity_detail_components.insert(Entity::Proposals, proposal_details);
    entity_detail_components.insert(Entity::Pools, pool_details);
    entity_detail_components.insert(Entity::UTXOs, utxo_details);
    let entity_details_switcher = shared(SwitchComponent::new(
        entity_types.clone(),
        entity_detail_components,
    ));

    let body = shared(SplitComponent::new_2(
        Axis::Vertical,
        30,
        shared(SplitComponent::new_2_evenly(
            Axis::Horizontal,
            entity_types.clone(),
            entity_ids_switcher.clone(),
        )),
        70,
        entity_details_switcher.clone(),
    ));

    let layout = RootLayout::new(
        shared(ComponentGroup::new(vec![
            shared(Message::new(format!(
                "Reading amaru ledger at {:?}",
                ledger_path_str
            ))),
            shared(FpsCounter::default()),
        ])),
        body,
        shared(ComponentGroup::new(vec![shared(Message::new(
            "Use Shift + Left/Right/Up/Down (←↑→↓) to move focus. Use Left/Right/Up/Down to scroll within focus.".to_string(),
        ))])),
    );

    Ok((
        layout,
        FocusManager::new(vec![
            entity_types.clone(),
            entity_ids_switcher.clone(),
            entity_details_switcher.clone(),
        ]),
    ))
}

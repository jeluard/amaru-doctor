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
        list_and_details::{
            account::{new_account_details_component, new_account_list_component},
            drep::{new_drep_details_component, new_drep_list_component},
            pool::{new_pool_details_component, new_pool_list_component},
            proposal::{new_proposal_details_component, new_proposal_list_component},
            utxo::{new_utxo_details_component, new_utxo_list_component},
        },
        message::Message,
    },
    focus::{FocusManager, FocusableComponent},
    shared::{Shared, shared},
};
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use std::{collections::HashMap, sync::Arc};

pub fn build_layout<'a>(
    ledger_path_str: &String,
    db: &'a Arc<RocksDB>,
) -> Result<(RootLayout<'a>, FocusManager<'a>)> {
    let entity_types = shared(new_entity_types_list());

    let accounts = shared(new_account_list_component(db));
    let dreps = shared(new_drep_list_component(db));
    let pools = shared(new_pool_list_component(db));
    let proposals = shared(new_proposal_list_component(db));
    let utxos = shared(new_utxo_list_component(db));
    let mut entity_id_components: HashMap<Entity, Shared<dyn FocusableComponent>> = HashMap::new();
    entity_id_components.insert(Entity::Accounts, accounts.clone());
    entity_id_components.insert(Entity::DReps, dreps.clone());
    entity_id_components.insert(Entity::Pools, pools.clone());
    entity_id_components.insert(Entity::Proposals, proposals.clone());
    entity_id_components.insert(Entity::UTXOs, utxos.clone());
    let entity_ids_switcher = shared(SwitchComponent::new(
        entity_types.clone(),
        |s| serde_plain::from_str(s).unwrap(),
        entity_id_components,
    ));

    let account_details = shared(new_account_details_component(accounts.clone()));
    let drep_details = shared(new_drep_details_component(dreps.clone()));
    let pool_details = shared(new_pool_details_component(pools.clone()));
    let proposal_details = shared(new_proposal_details_component(proposals.clone()));
    let utxo_details = shared(new_utxo_details_component(utxos.clone()));
    let mut entity_detail_components: HashMap<Entity, Shared<dyn FocusableComponent>> =
        HashMap::new();
    entity_detail_components.insert(Entity::Accounts, account_details);
    entity_detail_components.insert(Entity::DReps, drep_details);
    entity_detail_components.insert(Entity::Proposals, proposal_details);
    entity_detail_components.insert(Entity::Pools, pool_details);
    entity_detail_components.insert(Entity::UTXOs, utxo_details);
    let entity_details_switcher = shared(SwitchComponent::new(
        entity_types.clone(),
        |s| serde_plain::from_str(s).unwrap(),
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

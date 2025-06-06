use crate::{
    components::{group::scroll::ScrollableListComponent, list_and_details::IteratorSource},
    to_list_item::ToListItem,
};
use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Entity {
    Accounts,
    #[serde(rename = "block issuers")]
    BlockIssuers,
    DReps,
    Pools,
    Proposals,
    UTXOs,
}

impl ToListItem for Entity {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

pub fn new_entity_types_list() -> ScrollableListComponent<'static, Entity> {
    let source = Rc::new(IteratorSource::new(
        vec![
            Entity::Accounts,
            Entity::BlockIssuers,
            Entity::DReps,
            Entity::Pools,
            Entity::Proposals,
            Entity::UTXOs,
        ]
        .into_iter(),
    ));

    ScrollableListComponent::new("Entity Types".to_string(), source, 10)
}

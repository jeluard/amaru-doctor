use ratatui::widgets::ListItem;

use crate::{action::Entity, to_list_item::ToListItem};

use super::group::scroll::ScrollableListComponent;

impl ToListItem for Entity {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(self.to_string())
    }
}

pub fn new_entity_types_list() -> ScrollableListComponent<'static, Entity> {
    let items = vec![
        Entity::Accounts,
        Entity::BlockIssuers,
        Entity::DReps,
        Entity::Pools,
        Entity::Proposals,
        Entity::UTXOs,
    ]
    .into_iter();

    ScrollableListComponent::new("Entity Types".to_string(), items, 10)
}

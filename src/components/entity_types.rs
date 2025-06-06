use super::group::scroll::ScrollableListComponent;
use crate::{action::Entity, to_list_item::ToListItem, window::VecSource};
use ratatui::widgets::ListItem;
use std::rc::Rc;

impl ToListItem for Entity {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(self.to_string())
    }
}

pub fn new_entity_types_list() -> ScrollableListComponent<'static, Entity> {
    let source = Rc::new(VecSource {
        data: vec![
            Entity::Accounts,
            Entity::BlockIssuers,
            Entity::DReps,
            Entity::Pools,
            Entity::Proposals,
            Entity::UTXOs,
        ]
        .into(),
    });

    ScrollableListComponent::new("Entity Types".to_string(), source, 10)
}

use crate::{
    components::group::scroll::ScrollableListComponent, to_list_item::ToListItem,
    window::IteratorSource,
};
use ratatui::widgets::ListItem;
use serde::Serialize;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Serialize)]
pub enum SearchType {
    #[serde(rename = "utxo by address")]
    UtxoByAddress,
}

impl ToListItem for SearchType {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

pub fn new_search_types_list() -> ScrollableListComponent<SearchType> {
    let source = Rc::new(IteratorSource::new(
        vec![SearchType::UtxoByAddress].into_iter(),
    ));

    ScrollableListComponent::new("Search Types".to_string(), source, 10)
}

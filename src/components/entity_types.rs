use super::group::scroll::ScrollableListComponent;
use crate::action::SelectedItem;
use ratatui::widgets::ListItem;

pub fn new_entity_types_list() -> ScrollableListComponent<
    String,
    std::vec::IntoIter<String>,
    fn(&String) -> Option<SelectedItem>,
    fn(&String) -> ListItem,
> {
    fn select(s: &String) -> Option<SelectedItem> {
        Some(SelectedItem::EntityType(serde_plain::from_str(s).unwrap()))
    }

    fn render(item: &String) -> ListItem {
        ListItem::new(item.clone())
    }

    let items = vec![
        "accounts".to_string(),
        "pools".to_string(),
        "utxos".to_string(),
    ]
    .into_iter();

    ScrollableListComponent::new("Resources".to_string(), items, 10, select, render)
}

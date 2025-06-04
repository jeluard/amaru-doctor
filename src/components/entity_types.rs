use super::group::scroll::ScrollableListComponent;
use ratatui::widgets::ListItem;

pub fn new_entity_types_list()
-> ScrollableListComponent<String, std::vec::IntoIter<String>, fn(&String) -> ListItem> {
    fn render(item: &String) -> ListItem {
        ListItem::new(item.to_owned())
    }

    let items = vec![
        "accounts".to_string(),
        "dreps".to_string(),
        "pools".to_string(),
        "proposals".to_string(),
        "utxos".to_string(),
    ]
    .into_iter();

    ScrollableListComponent::new("Entity Types".to_string(), items, 10, render)
}

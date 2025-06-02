use super::group::scroll::ScrollableListComponent;
use crate::action::SelectedItem;
use ratatui::widgets::ListItem;

#[allow(clippy::type_complexity)]
pub fn new_entity_types_list() -> ScrollableListComponent<
    String,
    std::vec::IntoIter<String>,
    // fn(&String) -> Option<SelectedItem>,
    fn(&String) -> ListItem,
> {
    // #[allow(clippy::ptr_arg)]
    // fn select(s: &String) -> Option<SelectedItem> {
    //     Some(SelectedItem::EntityType(serde_plain::from_str(s).unwrap()))
    // }

    #[allow(clippy::ptr_arg)]
    fn render(item: &String) -> ListItem {
        ListItem::new(item.to_owned())
    }

    let items = vec![
        "accounts".to_string(),
        "pools".to_string(),
        "utxos".to_string(),
    ]
    .into_iter();

    ScrollableListComponent::new("Entity Types".to_string(), items, 10, render)
}

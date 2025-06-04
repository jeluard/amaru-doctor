use ratatui::widgets::ListItem;

pub trait ToListItem {
    fn to_list_item(&self) -> ListItem<'static>;
}

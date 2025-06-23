use ratatui::widgets::ListItem;
use serde::Serialize;
use strum::EnumIter;

use crate::ui::to_list_item::ToListItem;

#[derive(PartialEq, Eq, EnumIter, Serialize)]
pub enum ChainSearchOption {
    Header,
    Block,
    Nonces,
}

impl ToListItem for ChainSearchOption {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

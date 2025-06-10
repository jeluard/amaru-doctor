use crate::ui::to_list_item::ToListItem;
use ratatui::widgets::ListItem;
use serde::Serialize;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, Serialize)]
pub enum SearchOptions {
    #[serde(rename = "utxos by address")]
    UtxosByAddress,
}

impl ToListItem for SearchOptions {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

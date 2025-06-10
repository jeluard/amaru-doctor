use crate::to_list_item::ToListItem;
use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Display, Serialize, Deserialize, Hash)]
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

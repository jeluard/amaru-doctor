use crate::ui::to_list_item::ToListItem;
use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
    FocusPrev,
    FocusNext,
    SearchRequest,
    ScrollUp,
    ScrollDown,
}

#[derive(Clone, Debug, Default, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BrowseOptions {
    #[default]
    Accounts,
    #[serde(rename = "block issuers")]
    BlockIssuers,
    DReps,
    Pools,
    Proposals,
    Utxos,
}

impl ToListItem for BrowseOptions {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum Tab {
    Browse,
    Search,
}

#[derive(Clone, Debug, Display, EnumIter, PartialEq, Eq)]
pub enum WidgetSlot {
    Nav,
    NavType,
    List,
    Details,
}

#[derive(Clone, Debug, Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetId {
    Empty,
    CursorTabs,
    ListBrowseOptions,
    ListSearchOptions,
    ListAccounts,
    ListBlockIssuers,
    ListDReps,
    ListPools,
    ListProposals,
    ListUtxos,
    // SearchAccounts,
    // SearchBlockIssuers,
    // SearchDReps,
    // SearchPools,
    // SearchProposals,
    // SearchUTXOs,
    DetailAccount,
    DetailBlockIssuer,
    DetailDRep,
    DetailPool,
    DetailProposal,
    DetailUtxo,
}

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
    // TODO: These need to be somewhere else
    Entites,
    SearchTypes,
    Nav,
}

impl ToListItem for Entity {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

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

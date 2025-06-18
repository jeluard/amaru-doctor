use crate::ui::to_list_item::ToListItem;
use ratatui::{
    crossterm::event::KeyCode,
    prelude::Line,
    text::ToLine,
    widgets::{ListItem, block::Title},
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    SetWindowSize(WidgetSlot, usize),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
    FocusPrev,
    FocusNext,
    ScrollUp,
    ScrollDown,
    Key(KeyCode),
    SearchUtxosByAddr,
}

#[derive(Clone, Debug, Default, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BrowseOption {
    #[default]
    Accounts,
    #[serde(rename = "block issuers")]
    BlockIssuers,
    DReps,
    Pools,
    Proposals,
    Utxos,
}

impl ToListItem for BrowseOption {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

#[derive(Clone, Debug, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LedgerMode {
    Browse,
    Search,
}

impl ToLine for LedgerMode {
    fn to_line(&self) -> Line<'_> {
        Line::from(serde_plain::to_string(self).unwrap().to_owned())
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetSlot {
    TopLine,
    StoreOption,
    LedgerMode,
    SearchBar,
    Options,
    List,
    Details,
    BottomLine,
}

#[derive(Clone, Debug, Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetId {
    Empty,
    TopInfo,
    BottomInfo,
    #[serde(rename = "Store")]
    StoreOption,
    #[serde(rename = "Ledger Explore Mode")]
    LedgerMode,
    #[serde(rename = "Search Query")]
    SearchQuery,
    #[serde(rename = "Browse Options")]
    BrowseOptions,
    #[serde(rename = "Queries")]
    SearchOptions,
    #[serde(rename = "Accounts")]
    ListAccounts,
    #[serde(rename = "Block Issuers")]
    ListBlockIssuers,
    #[serde(rename = "Dreps")]
    ListDReps,
    #[serde(rename = "Pools")]
    ListPools,
    #[serde(rename = "Proposals")]
    ListProposals,
    #[serde(rename = "Utxos")]
    ListUtxos,
    #[serde(rename = "Utxos by Address")]
    ListUtxosByAddr,
    #[serde(rename = "Account Details")]
    DetailsAccount,
    #[serde(rename = "Block Issuer Details")]
    DetailsBlockIssuer,
    #[serde(rename = "DRep Details")]
    DetailsDRep,
    #[serde(rename = "Pool Details")]
    DetailsPool,
    #[serde(rename = "Proposal Details")]
    DetailsProposal,
    #[serde(rename = "Utxo Details")]
    DetailsUtxo,
}

impl From<WidgetId> for Title<'_> {
    fn from(wid: WidgetId) -> Self {
        Title::from(serde_plain::to_string(&wid).unwrap())
    }
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, Serialize)]
pub enum SearchOption {
    #[serde(rename = "utxos by address")]
    UtxosByAddress,
}

impl ToListItem for SearchOption {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

#[derive(Clone, Debug, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StoreOption {
    Ledger,
    Chain,
}

impl ToLine for StoreOption {
    fn to_line(&self) -> Line<'_> {
        Line::from(serde_plain::to_string(self).unwrap().to_owned())
    }
}

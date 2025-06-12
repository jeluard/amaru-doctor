use crate::ui::to_list_item::ToListItem;
use ratatui::{prelude::Line, text::ToLine, widgets::ListItem};
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
    SearchRequest,
    ScrollUp,
    ScrollDown,
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
pub enum TabOption {
    Browse,
    Search,
}

impl ToLine for TabOption {
    fn to_line(&self) -> Line<'_> {
        Line::from(serde_plain::to_string(self).unwrap().to_owned())
    }
}

#[derive(Clone, Debug, Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetSlot {
    Tabs,
    Options,
    List,
    Details,
}

#[derive(Clone, Debug, Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetId {
    Empty,
    #[serde(rename = "Nav")]
    CursorTabs,
    #[serde(rename = "Resources")]
    ListBrowseOptions,
    #[serde(rename = "Queries")]
    ListSearchOptions,
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
    // SearchAccounts,
    // SearchBlockIssuers,
    // SearchDReps,
    // SearchPools,
    // SearchProposals,
    // SearchUTXOs,
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

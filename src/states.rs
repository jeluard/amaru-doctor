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
    ScrollUp(WidgetId),
    ScrollDown(WidgetId),
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
pub enum Slot {
    Nav,
    NavType,
    List,
    Details,
}

#[derive(Clone, Debug, Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetId {
    Empty,
    ListTabs,
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

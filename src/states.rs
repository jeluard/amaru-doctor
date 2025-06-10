use crate::{components::r#static::entity_types::Entity, ui::to_list_item::ToListItem};
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
    ScrollUp(SlotSelection),
    ScrollDown(SlotSelection),
}

#[derive(Clone, Debug, Default, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityOptions {
    #[default]
    Accounts,
    #[serde(rename = "block issuers")]
    BlockIssuers,
    DReps,
    Pools,
    Proposals,
    Utxos,
}

impl ToListItem for EntityOptions {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum Nav {
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

#[derive(Clone, Debug, Display, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotSelection {
    Nav,
    NavTypeBrowse,
    NavTypeSearch,
    BrowseAccounts,
    BrowseBlockIssuers,
    BrowseDReps,
    BrowsePools,
    BrowseProposals,
    BrowseUtxos,
    SearchAccounts,
    SearchBlockIssuers,
    SearchDReps,
    SearchPools,
    SearchProposals,
    SearchUTXOs,
    DetailAccount,
    DetailBlockIssuer,
    DetailDRep,
    DetailPool,
    DetailProposal,
    DetailUtxo,
}

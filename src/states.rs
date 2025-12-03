use crate::ui::to_list_item::ToListItem;
use crossterm::event::KeyCode;
use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Default, Display, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentId {
    // --- Containers ---
    #[default]
    Root,
    LedgerPage,
    OtelPage,
    MetricsPage,
    ChainPage,

    // --- Global / Reusable ---
    InspectTabs,
    SearchBar,

    // --- Ledger Page ---
    LedgerModeTabs,
    LedgerBrowseOptions,
    LedgerSearchOptions,
    LedgerAccountsList,
    LedgerAccountDetails,
    LedgerBlockIssuersList,
    LedgerBlockIssuerDetails,
    LedgerDRepsList,
    LedgerDRepDetails,
    LedgerPoolsList,
    LedgerPoolDetails,
    LedgerProposalsList,
    LedgerProposalDetails,
    LedgerUtxosList,
    LedgerUtxoDetails,
    LedgerUtxosByAddrList,
    LedgerUtxosByAddrDetails,

    // --- Chain Page ---
    ChainSearch,
    ChainSearchHeader,
    ChainSearchBlock,
    ChainSearchNonces,

    // --- Otel Page ---
    OtelTraceList,
    OtelFlameGraph,
    OtelSpanDetails,

    // --- Metrics Page ---
    Metrics,
}

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
    FocusUp,
    FocusDown,
    FocusLeft,
    FocusRight,
    ScrollUp,
    ScrollDown,
    Up,
    Down,
    Forward,
    Back,
    Key(KeyCode),
    SubmitSearch(String),
    SetFocus(ComponentId),
    FocusNext,
    FocusPrev,
}

impl Action {
    pub fn is_noisy(&self) -> bool {
        matches!(self, Self::Tick | Self::Render)
    }
}

#[derive(Clone, Debug, Default, Display, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LedgerBrowse {
    #[default]
    Accounts,
    #[serde(rename = "block issuers")]
    BlockIssuers,
    DReps,
    Pools,
    Proposals,
    Utxos,
}

impl ToListItem for LedgerBrowse {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

#[derive(Clone, Copy, Debug, Default, Display, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LedgerMode {
    #[default]
    Browse,
    Search,
}

#[derive(Clone, Copy, Default, Debug, EnumIter, PartialEq, Eq, Serialize)]
pub enum LedgerSearch {
    #[default]
    #[serde(rename = "utxos by address")]
    UtxosByAddress,
}

impl ToListItem for LedgerSearch {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

#[derive(Clone, Copy, Debug, Default, Display, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InspectOption {
    #[default]
    Ledger,
    Chain,
    Otel,
    Metrics,
}

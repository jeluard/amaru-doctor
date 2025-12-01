use crate::ui::to_list_item::ToListItem;
use crossterm::event::{KeyCode, MouseEvent};
use ratatui::{layout::Rect, widgets::ListItem};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Default, Display, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentId {
    // --- Containers ---
    #[default]
    Root,
    LegacyRoot,
    LedgerPage,
    OtelPage,
    PrometheusPage,
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

    // --- Prometheus Page ---
    PrometheusMetrics,
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    SetWindowSize(ComponentId, usize),
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
    MouseDragDown,
    MouseDragUp,
    Up,
    Down,
    Forward,
    Back,
    Key(KeyCode),
    UpdateLayout(Rect),
    MouseEvent(MouseEvent),
    MouseClick(u16, u16),
    GetButtonEvents,
    SubmitSearch(String),
    SetFocus(ComponentId),
    FocusNext,
    FocusPrev,
}

impl Action {
    pub fn is_noisy(&self) -> bool {
        matches!(
            self,
            Self::Tick
                | Self::Render
                | Self::GetButtonEvents
                | Self::MouseEvent(_)
                | Self::SetWindowSize(_, _)
        )
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
    Prometheus,
}

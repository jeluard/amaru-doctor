use crate::ui::to_list_item::ToListItem;
use crossterm::event::{KeyCode, MouseEvent};
use ratatui::{layout::Rect, widgets::ListItem};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use strum::{Display, EnumIter, IntoEnumIterator};

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
    SyncTraceGraph,
    SyncPromMetrics,
    GetButtonEvents,
}

impl Action {
    pub fn is_noisy(&self) -> bool {
        matches!(
            self,
            Self::Tick
                | Self::Render
                | Self::SyncPromMetrics
                | Self::SyncTraceGraph
                | Self::GetButtonEvents
                | Self::MouseEvent(_)
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

#[derive(
    Clone, Copy, Debug, Default, Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum WidgetSlot {
    TopLine,
    #[default] // Default focused slot
    InspectOption,
    LedgerMode,
    SearchBar,
    // Either the LedgerBrowse (accounts, block issuers, etc.) or LedgerSearch (utxos by
    // addr) options
    LedgerOptions,
    // Listed items
    List,
    Details,
    // Used for span details today
    SubDetails,
    LedgerHeaderDetails,
    LedgerBlockDetails,
    LedgerNoncesDetails,
}

impl WidgetSlot {
    pub fn focusable() -> HashSet<WidgetSlot> {
        WidgetSlot::iter()
            .filter(|s| !matches!(s, WidgetSlot::TopLine))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, Serialize)]
pub enum LedgerSearch {
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
    //Chain,
    Otel,
    Prometheus,
}

use std::collections::HashSet;

use crate::ui::to_list_item::ToListItem;
use ratatui::{
    crossterm::event::KeyCode, layout::Rect, prelude::Line, text::ToLine, widgets::ListItem,
};
use serde::{Deserialize, Serialize};
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
    Key(KeyCode),
    UpdateLayout(Rect),
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
    InspectOption,
    LedgerMode,
    SearchBar,
    Options,
    List,
    Details,
    LedgerHeaderDetails,
    LedgerBlockDetails,
    LedgerNoncesDetails,
    BottomLine,
}

impl WidgetSlot {
    pub fn focusable() -> HashSet<WidgetSlot> {
        WidgetSlot::iter()
            .filter(|s| !matches!(s, WidgetSlot::TopLine | WidgetSlot::BottomLine))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, Serialize)]
pub enum LedgerSearchOption {
    #[serde(rename = "utxos by address")]
    UtxosByAddress,
}

impl ToListItem for LedgerSearchOption {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(serde_plain::to_string(self).unwrap())
    }
}

#[derive(Clone, Debug, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InspectOption {
    Ledger,
    Chain,
    Otel,
}

impl ToLine for InspectOption {
    fn to_line(&self) -> Line<'_> {
        Line::from(serde_plain::to_string(self).unwrap().to_owned())
    }
}

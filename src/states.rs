use crate::components::r#static::entity_types::Entity;
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
    ScrollUp(Entity),
    ScrollDown(Entity),
}

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Eq)]
pub enum NavMode {
    Browse,
    Search,
}

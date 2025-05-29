use amaru_ledger::store::columns::utxo;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum SelectedItem {
    Utxo(utxo::Key),
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
    FocusPrev,
    FocusNext,
    SelectItem(SelectedItem),
}

pub struct SelectedState<T> {
    pub value: Option<T>,
    matcher: fn(&SelectedItem) -> Option<T>,
}

impl<T: PartialEq + Clone> SelectedState<T> {
    pub fn new(matcher: fn(&SelectedItem) -> Option<T>) -> Self {
        Self {
            value: None,
            matcher,
        }
    }

    pub fn update(&mut self, action: &Action) -> bool {
        if let Action::SelectItem(selected) = action {
            if let Some(new_val) = (self.matcher)(selected) {
                if self.value.as_ref() != Some(&new_val) {
                    self.value = Some(new_val);
                    return true;
                }
            }
        }
        false
    }
}

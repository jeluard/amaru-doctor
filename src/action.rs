use amaru_kernel::TransactionInput;
use amaru_ledger::store::columns::utxo;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum SelectedItem {
    Utxo(TransactionInput),
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

pub trait SelectsFrom {
    fn from_selected(item: &SelectedItem) -> Option<Self>
    where
        Self: Sized;
}

impl SelectsFrom for TransactionInput {
    fn from_selected(item: &SelectedItem) -> Option<Self> {
        match item {
            SelectedItem::Utxo(k) => Some(k.clone()),
            _ => None,
        }
    }
}

pub struct SelectedState<T> {
    pub value: Option<T>,
}

impl<T> SelectedState<T>
where
    T: SelectsFrom + PartialEq + Clone,
{
    pub fn new() -> Self {
        Self { value: None }
    }

    pub fn update(&mut self, action: &Action) -> bool {
        if let Action::SelectItem(selected) = action {
            if let Some(new_val) = T::from_selected(selected) {
                if self.value.as_ref() != Some(&new_val) {
                    self.value = Some(new_val);
                    return true;
                }
            }
        }
        false
    }
}

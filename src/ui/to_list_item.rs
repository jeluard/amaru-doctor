use amaru_ledger::store::columns::{accounts, dreps, pools, proposals, slots, utxo};
use ratatui::widgets::ListItem;

use crate::ui::to_rich::{
    account::StakeCredentialDisplay, proposal::ProposalIdDisplay, utxo::TransactionInputDisplay,
};

pub trait ToListItem {
    fn to_list_item(&self) -> ListItem<'static>;
}

pub type AccountItem = (accounts::Key, accounts::Row);

impl ToListItem for AccountItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(StakeCredentialDisplay(&self.0).to_string())
    }
}

pub type BlockIssuerItem = (slots::Key, slots::Row);

impl ToListItem for BlockIssuerItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(self.0.to_string())
    }
}

pub type DRepItem = (dreps::Key, dreps::Row);

impl ToListItem for DRepItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(StakeCredentialDisplay(&self.0).to_string())
    }
}

pub type PoolItem = (pools::Key, pools::Row);

impl ToListItem for PoolItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(self.0.to_string())
    }
}

pub type ProposalItem = (proposals::Key, proposals::Row);

impl ToListItem for ProposalItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(ProposalIdDisplay(&self.0).to_string())
    }
}

pub type UtxoItem = (utxo::Key, utxo::Value);

impl ToListItem for UtxoItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(TransactionInputDisplay(&self.0).to_string())
    }
}

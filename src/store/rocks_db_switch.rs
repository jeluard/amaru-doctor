use crate::ui::to_list_item::{
    AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem,
};
use amaru_kernel::{PoolId, StakeCredential, TransactionInput, TransactionOutput};
use amaru_ledger::{
    store::{
        ReadOnlyStore, StoreError,
        columns::{accounts::Row, pools},
    },
    summary::Pots,
};
use amaru_stores::rocksdb::{RocksDB, RocksDBSnapshot};

pub enum RocksDBSwitch {
    Store(RocksDB),
    Snapshot(RocksDBSnapshot),
}

impl ReadOnlyStore for RocksDBSwitch {
    fn pool(&self, pool: &PoolId) -> Result<Option<pools::Row>, StoreError> {
        match self {
            RocksDBSwitch::Store(db) => db.pool(pool),
            RocksDBSwitch::Snapshot(db) => db.pool(pool),
        }
    }

    fn account(&self, credential: &StakeCredential) -> Result<Option<Row>, StoreError> {
        match self {
            RocksDBSwitch::Store(db) => db.account(credential),
            RocksDBSwitch::Snapshot(db) => db.account(credential),
        }
    }

    fn utxo(&self, input: &TransactionInput) -> Result<Option<TransactionOutput>, StoreError> {
        match self {
            RocksDBSwitch::Store(db) => db.utxo(input),
            RocksDBSwitch::Snapshot(db) => db.utxo(input),
        }
    }

    fn pots(&self) -> Result<Pots, StoreError> {
        match self {
            RocksDBSwitch::Store(db) => db.pots(),
            RocksDBSwitch::Snapshot(db) => db.pots(),
        }
    }

    fn iter_utxos(&self) -> Result<impl Iterator<Item = UtxoItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = UtxoItem>> = match self {
            RocksDBSwitch::Store(db) => Box::new(db.iter_utxos()?),
            RocksDBSwitch::Snapshot(db) => Box::new(db.iter_utxos()?),
        };
        Ok(boxed)
    }

    fn iter_block_issuers(&self) -> Result<impl Iterator<Item = BlockIssuerItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = BlockIssuerItem>> = match self {
            RocksDBSwitch::Store(db) => Box::new(db.iter_block_issuers()?),
            RocksDBSwitch::Snapshot(db) => Box::new(db.iter_block_issuers()?),
        };
        Ok(boxed)
    }

    fn iter_pools(&self) -> Result<impl Iterator<Item = PoolItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = PoolItem>> = match self {
            RocksDBSwitch::Store(db) => Box::new(db.iter_pools()?),
            RocksDBSwitch::Snapshot(db) => Box::new(db.iter_pools()?),
        };
        Ok(boxed)
    }

    fn iter_accounts(&self) -> Result<impl Iterator<Item = AccountItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = AccountItem>> = match self {
            RocksDBSwitch::Store(db) => Box::new(db.iter_accounts()?),
            RocksDBSwitch::Snapshot(db) => Box::new(db.iter_accounts()?),
        };
        Ok(boxed)
    }

    fn iter_dreps(&self) -> Result<impl Iterator<Item = DRepItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = DRepItem>> = match self {
            RocksDBSwitch::Store(db) => Box::new(db.iter_dreps()?),
            RocksDBSwitch::Snapshot(db) => Box::new(db.iter_dreps()?),
        };
        Ok(boxed)
    }

    fn iter_proposals(&self) -> Result<impl Iterator<Item = ProposalItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = ProposalItem>> = match self {
            RocksDBSwitch::Store(db) => Box::new(db.iter_proposals()?),
            RocksDBSwitch::Snapshot(db) => Box::new(db.iter_proposals()?),
        };
        Ok(boxed)
    }
}

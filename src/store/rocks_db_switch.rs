use crate::ui::to_list_item::{
    AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem,
};
use amaru_kernel::{
    PoolId, StakeCredential, TransactionInput, TransactionOutput,
    protocol_parameters::ProtocolParameters,
};
use amaru_ledger::{
    store::{
        ReadOnlyStore, StoreError,
        columns::{accounts::Row, pools},
    },
    summary::Pots,
};
use amaru_stores::rocksdb::{RocksDB, RocksDBSnapshot};
use slot_arithmetic::Epoch;

pub enum LedgerDB {
    Store(RocksDB),
    Snapshot(RocksDBSnapshot),
}

impl ReadOnlyStore for LedgerDB {
    fn pool(&self, pool: &PoolId) -> Result<Option<pools::Row>, StoreError> {
        match self {
            LedgerDB::Store(db) => db.pool(pool),
            LedgerDB::Snapshot(db) => db.pool(pool),
        }
    }

    fn account(&self, credential: &StakeCredential) -> Result<Option<Row>, StoreError> {
        match self {
            LedgerDB::Store(db) => db.account(credential),
            LedgerDB::Snapshot(db) => db.account(credential),
        }
    }

    fn utxo(&self, input: &TransactionInput) -> Result<Option<TransactionOutput>, StoreError> {
        match self {
            LedgerDB::Store(db) => db.utxo(input),
            LedgerDB::Snapshot(db) => db.utxo(input),
        }
    }

    fn pots(&self) -> Result<Pots, StoreError> {
        match self {
            LedgerDB::Store(db) => db.pots(),
            LedgerDB::Snapshot(db) => db.pots(),
        }
    }

    fn iter_utxos(&self) -> Result<impl Iterator<Item = UtxoItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = UtxoItem>> = match self {
            LedgerDB::Store(db) => Box::new(db.iter_utxos()?),
            LedgerDB::Snapshot(db) => Box::new(db.iter_utxos()?),
        };
        Ok(boxed)
    }

    fn iter_block_issuers(&self) -> Result<impl Iterator<Item = BlockIssuerItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = BlockIssuerItem>> = match self {
            LedgerDB::Store(db) => Box::new(db.iter_block_issuers()?),
            LedgerDB::Snapshot(db) => Box::new(db.iter_block_issuers()?),
        };
        Ok(boxed)
    }

    fn iter_pools(&self) -> Result<impl Iterator<Item = PoolItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = PoolItem>> = match self {
            LedgerDB::Store(db) => Box::new(db.iter_pools()?),
            LedgerDB::Snapshot(db) => Box::new(db.iter_pools()?),
        };
        Ok(boxed)
    }

    fn iter_accounts(&self) -> Result<impl Iterator<Item = AccountItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = AccountItem>> = match self {
            LedgerDB::Store(db) => Box::new(db.iter_accounts()?),
            LedgerDB::Snapshot(db) => Box::new(db.iter_accounts()?),
        };
        Ok(boxed)
    }

    fn iter_dreps(&self) -> Result<impl Iterator<Item = DRepItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = DRepItem>> = match self {
            LedgerDB::Store(db) => Box::new(db.iter_dreps()?),
            LedgerDB::Snapshot(db) => Box::new(db.iter_dreps()?),
        };
        Ok(boxed)
    }

    fn iter_proposals(&self) -> Result<impl Iterator<Item = ProposalItem>, StoreError> {
        let boxed: Box<dyn Iterator<Item = ProposalItem>> = match self {
            LedgerDB::Store(db) => Box::new(db.iter_proposals()?),
            LedgerDB::Snapshot(db) => Box::new(db.iter_proposals()?),
        };
        Ok(boxed)
    }

    fn get_protocol_parameters_for(&self, epoch: &Epoch) -> Result<ProtocolParameters, StoreError> {
        match self {
            LedgerDB::Store(db) => db.get_protocol_parameters_for(epoch),
            LedgerDB::Snapshot(db) => db.get_protocol_parameters_for(epoch),
        }
    }
}

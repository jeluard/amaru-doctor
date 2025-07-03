use crate::ui::to_list_item::{
    AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem,
};
use amaru_ledger::store::ReadStore;
use amaru_stores::rocksdb::ReadOnlyRocksDB;
use std::sync::{Arc, mpsc};
use std::thread;

pub struct OwnedAccountIter {
    inner: Box<dyn Iterator<Item = AccountItem>>,
}

impl OwnedAccountIter {
    pub fn new(db: Arc<ReadOnlyRocksDB>) -> Self {
        let (tx, rx) = mpsc::sync_channel(1);
        let db_clone = db.clone();
        thread::spawn(move || {
            let iter = db_clone.iter_accounts().unwrap();
            for kv in iter {
                if tx.send(kv).is_err() {
                    break;
                }
            }
        });
        OwnedAccountIter {
            inner: Box::new(rx.into_iter()),
        }
    }
}

impl Iterator for OwnedAccountIter {
    type Item = AccountItem;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct OwnedBlockIssuerIter {
    inner: Box<dyn Iterator<Item = BlockIssuerItem>>,
}

impl OwnedBlockIssuerIter {
    pub fn new(db: Arc<ReadOnlyRocksDB>) -> Self {
        let (tx, rx) = mpsc::sync_channel(1);
        let db_clone = db.clone();
        thread::spawn(move || {
            let iter = db_clone.iter_block_issuers().unwrap();
            for kv in iter {
                if tx.send(kv).is_err() {
                    break;
                }
            }
        });
        OwnedBlockIssuerIter {
            inner: Box::new(rx.into_iter()),
        }
    }
}

impl Iterator for OwnedBlockIssuerIter {
    type Item = BlockIssuerItem;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct OwnedDRepIter {
    inner: Box<dyn Iterator<Item = DRepItem>>,
}

impl OwnedDRepIter {
    pub fn new(db: Arc<ReadOnlyRocksDB>) -> Self {
        let (tx, rx) = mpsc::sync_channel(1);
        let db_clone = db.clone();
        thread::spawn(move || {
            let iter = db_clone.iter_dreps().unwrap();
            for kv in iter {
                if tx.send(kv).is_err() {
                    break;
                }
            }
        });
        OwnedDRepIter {
            inner: Box::new(rx.into_iter()),
        }
    }
}

impl Iterator for OwnedDRepIter {
    type Item = DRepItem;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct OwnedPoolIter {
    inner: Box<dyn Iterator<Item = PoolItem>>,
}

impl OwnedPoolIter {
    pub fn new(db: Arc<ReadOnlyRocksDB>) -> Self {
        let (tx, rx) = mpsc::sync_channel(1);
        let db_clone = db.clone();
        thread::spawn(move || {
            let iter = db_clone.iter_pools().unwrap();
            for kv in iter {
                if tx.send(kv).is_err() {
                    break;
                }
            }
        });
        OwnedPoolIter {
            inner: Box::new(rx.into_iter()),
        }
    }
}

impl Iterator for OwnedPoolIter {
    type Item = PoolItem;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct OwnedProposalIter {
    inner: Box<dyn Iterator<Item = ProposalItem>>,
}

impl OwnedProposalIter {
    pub fn new(db: Arc<ReadOnlyRocksDB>) -> Self {
        let (tx, rx) = mpsc::sync_channel(1);
        let db_clone = db.clone();
        thread::spawn(move || {
            let iter = db_clone.iter_proposals().unwrap();
            for kv in iter {
                if tx.send(kv).is_err() {
                    break;
                }
            }
        });
        OwnedProposalIter {
            inner: Box::new(rx.into_iter()),
        }
    }
}

impl Iterator for OwnedProposalIter {
    type Item = ProposalItem;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct OwnedUtxoIter {
    inner: Box<dyn Iterator<Item = UtxoItem>>,
}

impl OwnedUtxoIter {
    pub fn new(db: Arc<ReadOnlyRocksDB>) -> Self {
        let (tx, rx) = mpsc::sync_channel(1);
        let db_clone = db.clone();

        thread::spawn(move || {
            let iter = db_clone.iter_utxos().unwrap();
            for kv in iter {
                if tx.send(kv).is_err() {
                    break;
                }
            }
        });

        OwnedUtxoIter {
            inner: Box::new(rx.into_iter()),
        }
    }
}

impl Iterator for OwnedUtxoIter {
    type Item = UtxoItem;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

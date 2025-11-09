use crate::ui::to_list_item::{
    AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem,
};
use amaru_ledger::store::ReadStore;
use amaru_stores::rocksdb::ReadOnlyRocksDB;
use std::sync::{Arc, mpsc};
use std::thread;

pub struct OwnedDbIter<T> {
    rx: mpsc::Receiver<T>,
}

impl<T: Send + 'static> OwnedDbIter<T> {
    pub fn new<F>(iter_logic_fn: F) -> Self
    where
        F: FnOnce(mpsc::SyncSender<T>) + Send + 'static,
    {
        let (tx, rx) = mpsc::sync_channel(1);
        thread::spawn(move || {
            iter_logic_fn(tx);
        });

        OwnedDbIter { rx }
    }
}

impl<T> Iterator for OwnedDbIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.rx.recv().ok()
    }
}

macro_rules! define_owned_db_iter {
    (
        $(#[$struct_doc:meta])*
        $StructName:ident,
        $ItemType:ty,
        $iter_method:ident
    ) => {
        $(#[$struct_doc])*
        pub struct $StructName {
            iter: OwnedDbIter<$ItemType>,
        }

        impl $StructName {
            pub fn new(db: Arc<ReadOnlyRocksDB>) -> Self {
                Self {
                    iter: OwnedDbIter::new(move |tx: mpsc::SyncSender<$ItemType>| {
                        if let Ok(iter) = db.$iter_method() {
                            for item in iter {
                                if tx.send(item).is_err() {
                                    break;
                                }
                            }
                        }
                    }),
                }
            }
        }

        impl Iterator for $StructName {
            type Item = $ItemType;
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }
        }
    };
}

define_owned_db_iter!(OwnedAccountIter, AccountItem, iter_accounts);
define_owned_db_iter!(OwnedBlockIssuerIter, BlockIssuerItem, iter_block_issuers);
define_owned_db_iter!(OwnedDRepIter, DRepItem, iter_dreps);
define_owned_db_iter!(OwnedPoolIter, PoolItem, iter_pools);
define_owned_db_iter!(OwnedProposalIter, ProposalItem, iter_proposals);
define_owned_db_iter!(OwnedUtxoIter, UtxoItem, iter_utxos);

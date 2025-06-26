use amaru_consensus::{
    IsHeader, Nonces,
    consensus::{self, store::ChainStore},
};
use amaru_kernel::{EraHistory, Hash, RawBlock};
use amaru_ledger::store::StoreError;
use rocksdb::{DB, Options};
use std::path::PathBuf;

const NONCES_PREFIX: [u8; 5] = [0x6e, 0x6f, 0x6e, 0x63, 0x65];
const BLOCK_PREFIX: [u8; 5] = [0x62, 0x6c, 0x6f, 0x63, 0x6b];

pub struct ROChainDB {
    db: DB,
}

impl ROChainDB {
    pub fn open(basedir: PathBuf) -> Result<Self, StoreError> {
        DB::open_for_read_only(&Options::default(), basedir, false)
            .map(|db| Self { db })
            .map_err(|e| StoreError::Internal(e.into()))
    }
}

fn load_from_db<T: for<'d> minicbor::Decode<'d, ()>>(db: &DB, key: &[u8]) -> Option<T> {
    db.get_pinned(key)
        .ok()
        .flatten()
        .as_deref()
        .and_then(amaru_kernel::from_cbor)
}

impl<H: IsHeader + for<'d> minicbor::Decode<'d, ()>> ChainStore<H> for ROChainDB {
    fn load_header(&self, hash: &Hash<32>) -> Option<H> {
        load_from_db(&self.db, hash.as_ref())
    }

    fn store_header(
        &mut self,
        _hash: &Hash<32>,
        _header: &H,
    ) -> Result<(), consensus::store::StoreError> {
        panic!("Illegal method call, ROChainDB is readonly")
    }

    fn load_block(&self, hash: &Hash<32>) -> Result<RawBlock, consensus::store::StoreError> {
        let key = [&BLOCK_PREFIX[..], &hash[..]].concat();
        self.db
            .get_pinned(key)
            .map_err(|e| consensus::store::StoreError::ReadError {
                error: e.to_string(),
            })?
            .ok_or(consensus::store::StoreError::NotFound { hash: *hash })
            .map(|bytes| bytes.as_ref().into())
    }

    fn store_block(
        &mut self,
        _hash: &Hash<32>,
        _block: &RawBlock,
    ) -> Result<(), consensus::store::StoreError> {
        panic!("Illegal method call, ROChainDB is readonly")
    }

    fn get_nonces(&self, header: &Hash<32>) -> Option<Nonces> {
        let key = [&NONCES_PREFIX[..], &header[..]].concat();
        load_from_db(&self.db, &key)
    }

    fn put_nonces(
        &mut self,
        _header: &Hash<32>,
        _nonces: &Nonces,
    ) -> Result<(), consensus::store::StoreError> {
        panic!("Illegal method call, ROChainDB is readonly")
    }

    fn era_history(&self) -> &EraHistory {
        todo!()
    }
}

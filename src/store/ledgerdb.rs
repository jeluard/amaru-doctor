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
        columns::{
            accounts::{self},
            dreps, pools, pots, proposals, slots, utxo,
        },
    },
    summary::Pots,
};
use amaru_stores::rocksdb::common::{PREFIX_LEN, as_key};
use pallas_codec::minicbor::decode::Decode;
use rocksdb::{Direction, Options, SliceTransform};
use slot_arithmetic::Epoch;
use std::{
    fmt,
    path::{Path, PathBuf},
};

// Special key where we store the protocol parameters
const PROTOCOL_PARAMETERS_PREFIX: &str = "ppar";

/// Name prefixed used for storing Pool entries. UTF-8 encoding for "pool"
pub const POOL_PREFIX: [u8; PREFIX_LEN] = [0x70, 0x6f, 0x6f, 0x6c];

/// Name prefixed used for storing Account entries. UTF-8 encoding for "acct"
pub const ACCOUNT_PREFIX: [u8; PREFIX_LEN] = [0x61, 0x63, 0x63, 0x74];

/// Name prefixed used for storing UTxO entries. UTF-8 encoding for "utxo"
pub const UTXO_PREFIX: [u8; PREFIX_LEN] = [0x75, 0x74, 0x78, 0x6f];

/// Name prefixed used for storing protocol pots. UTF-8 encoding for "pots"
pub const POTS_PREFIX: [u8; PREFIX_LEN] = [0x70, 0x6f, 0x74, 0x73];

/// Name prefixed used for storing Pool entries. UTF-8 encoding for "slot"
pub const SLOT_PREFIX: [u8; PREFIX_LEN] = [0x73, 0x6c, 0x6f, 0x74];

/// Name prefixed used for storing DReps entries. UTF-8 encoding for "drep"
pub const DREP_PREFIX: [u8; PREFIX_LEN] = [0x64, 0x72, 0x65, 0x70];

/// Name prefixed used for storing Proposals entries. UTF-8 encoding for "prop"
pub const PROPOSAL_PREFIX: [u8; PREFIX_LEN] = [0x70, 0x72, 0x6F, 0x70];

pub struct ROLedgerDB {
    db: rocksdb::DB,
}

impl ROLedgerDB {
    pub fn open_live(base_dir: &Path) -> Result<Self, StoreError> {
        Self::open(base_dir.join("live"))
    }

    pub fn open_snapshot(base_dir: &Path, epoch: Epoch) -> Result<Self, StoreError> {
        Self::open(base_dir.join(epoch.to_string()))
    }

    fn open(path: PathBuf) -> Result<Self, StoreError> {
        let mut opts = Options::default();
        opts.set_prefix_extractor(SliceTransform::create_fixed_prefix(PREFIX_LEN));
        rocksdb::DB::open_for_read_only(&opts, path, false)
            .map(|db| Self { db })
            .map_err(|err| StoreError::Internal(err.into()))
    }
}

fn get_with_prefix<K, T>(
    db: &rocksdb::DB,
    prefix: &[u8; PREFIX_LEN],
    key: &K,
) -> Result<Option<T>, StoreError>
where
    T: for<'d> Decode<'d, ()>,
    K: fmt::Debug + minicbor::Encode<()>,
{
    let raw_key = as_key(prefix, key);
    get_cbor::<T>(db, &raw_key)
}

fn get_cbor<T: for<'d> Decode<'d, ()>>(
    db: &rocksdb::DB,
    key: &[u8],
) -> Result<Option<T>, StoreError> {
    db.get(key)
        .map_err(|err| StoreError::Internal(err.into()))?
        .map(|bytes| minicbor::decode(&bytes))
        .transpose()
        .map_err(StoreError::Undecodable)
}

fn iter<'a, K, V>(
    db: &'a rocksdb::DB,
    prefix: [u8; PREFIX_LEN],
    direction: rocksdb::Direction,
) -> Result<impl Iterator<Item = (K, V)> + 'a, StoreError>
where
    K: for<'d> Decode<'d, ()> + 'a,
    V: for<'d> Decode<'d, ()> + 'a,
{
    let mut opts = rocksdb::ReadOptions::default();
    opts.set_prefix_same_as_start(true);

    let iter = db
        .iterator_opt(rocksdb::IteratorMode::From(&prefix, direction), opts)
        .map(|entry| {
            let (key, value) = entry.unwrap();
            let k = minicbor::decode(&key[PREFIX_LEN..])
                .unwrap_or_else(|e| panic!("decode key failed ({}): {e:?}", hex::encode(&key)));
            let v = minicbor::decode(&value)
                .unwrap_or_else(|e| panic!("decode value failed ({}): {e:?}", hex::encode(&value)));
            (k, v)
        });

    Ok(iter)
}

impl ReadOnlyStore for ROLedgerDB {
    fn get_protocol_parameters_for(&self, epoch: &Epoch) -> Result<ProtocolParameters, StoreError> {
        let key = format!("{PROTOCOL_PARAMETERS_PREFIX}:{epoch}").into_bytes();
        get_cbor(&self.db, &key).map(|row| row.unwrap_or_default())
    }

    fn pool(&self, pool: &PoolId) -> Result<Option<pools::Row>, StoreError> {
        get_with_prefix(&self.db, &POOL_PREFIX, pool)
    }

    fn account(&self, credential: &StakeCredential) -> Result<Option<accounts::Row>, StoreError> {
        get_with_prefix(&self.db, &ACCOUNT_PREFIX, credential)
    }

    fn utxo(&self, input: &TransactionInput) -> Result<Option<TransactionOutput>, StoreError> {
        get_with_prefix(&self.db, &UTXO_PREFIX, input)
    }

    fn pots(&self) -> Result<Pots, StoreError> {
        let row = get_cbor::<pots::Row>(&self.db, &POTS_PREFIX)?.unwrap_or_default();
        Ok(Pots::from(&row))
    }

    fn iter_utxos(&self) -> Result<impl Iterator<Item = UtxoItem>, StoreError> {
        iter::<utxo::Key, utxo::Value>(&self.db, UTXO_PREFIX, Direction::Forward)
    }

    fn iter_block_issuers(&self) -> Result<impl Iterator<Item = BlockIssuerItem>, StoreError> {
        iter::<slots::Key, slots::Value>(&self.db, SLOT_PREFIX, Direction::Forward)
    }

    fn iter_pools(&self) -> Result<impl Iterator<Item = PoolItem>, StoreError> {
        iter::<pools::Key, pools::Row>(&self.db, POOL_PREFIX, Direction::Forward)
    }

    fn iter_accounts(&self) -> Result<impl Iterator<Item = AccountItem>, StoreError> {
        iter::<accounts::Key, accounts::Row>(&self.db, ACCOUNT_PREFIX, Direction::Forward)
    }

    fn iter_dreps(&self) -> Result<impl Iterator<Item = DRepItem>, StoreError> {
        iter::<dreps::Key, dreps::Row>(&self.db, DREP_PREFIX, Direction::Forward)
    }

    fn iter_proposals(&self) -> Result<impl Iterator<Item = ProposalItem>, StoreError> {
        iter::<proposals::Key, proposals::Row>(&self.db, PROPOSAL_PREFIX, Direction::Forward)
    }
}

use std::path::PathBuf;

use crate::detection::{AMARU_CHAIN_DB_ENV, AMARU_LEDGER_DB_ENV, detect_amaru_process};
use amaru_kernel::network::NetworkName;
use amaru_stores::rocksdb::{
    ReadOnlyRocksDB, RocksDbConfig,
    consensus::{ReadOnlyChainDB, RocksDBStore},
};
use anyhow::Result;

pub mod app;
mod app_state;
pub mod cli;
mod config;
mod controller;
pub mod detection;
pub mod logging;
pub mod model;
pub mod otel;
pub mod prometheus;
mod states;
mod store;
pub mod tui;
mod ui;
mod update;
mod view;

#[derive(Clone, Copy)]
pub enum ScreenMode {
    Large,
    Small,
}

fn default_db_name(name: &str, network: &NetworkName) -> String {
    format!("{}.{}.db", name, network)
}

fn prepend_path(opt_base: Option<PathBuf>, name: &str) -> PathBuf {
    opt_base
        .map(|base| base.join(name))
        .unwrap_or_else(|| PathBuf::from(name))
}

pub fn open_ledger_db(
    ledger_db: &Option<PathBuf>,
    network: &NetworkName,
) -> Result<ReadOnlyRocksDB> {
    if let Some(path) = ledger_db {
        ReadOnlyRocksDB::new(RocksDbConfig::new(path.into())).map_err(Into::into)
    } else {
        if let Some((cwd, envs)) = detect_amaru_process() {
            let path = envs
                .get(AMARU_LEDGER_DB_ENV)
                .cloned()
                .unwrap_or_else(|| default_db_name("ledger", network));
            return ReadOnlyRocksDB::new(RocksDbConfig::new(prepend_path(cwd, &path)))
                .map_err(Into::into);
        }
        panic!("No ledger db provided, either through env or args");
    }
}

pub fn open_chain_db(chain_db: &Option<PathBuf>, network: &NetworkName) -> Result<ReadOnlyChainDB> {
    if let Some(path) = chain_db {
        RocksDBStore::open_for_readonly(RocksDbConfig::new(path.into())).map_err(Into::into)
    } else {
        if let Some((cwd, envs)) = detect_amaru_process() {
            let path = envs
                .get(AMARU_CHAIN_DB_ENV)
                .cloned()
                .unwrap_or_else(|| default_db_name("chain", network));
            return RocksDBStore::open_for_readonly(RocksDbConfig::new(prepend_path(cwd, &path)))
                .map_err(Into::into);
        }
        panic!("No chain db provided, either through env or args");
    }
}

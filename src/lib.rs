use std::path::PathBuf;

use amaru_kernel::network::NetworkName;
use amaru_stores::rocksdb::{consensus::{ReadOnlyChainDB, RocksDBStore}, ReadOnlyRocksDB};
use anyhow::Result;

use crate::{cli::Cli, detection::{detect_amaru_process, AMARU_CHAIN_DB_ENV, AMARU_LEDGER_DB_ENV}};

pub mod app;
mod app_state;
pub mod cli;
mod config;
mod controller;
pub mod detection;
pub mod logging;
mod model;
pub mod otel;
mod states;
mod store;
pub mod tui;
mod ui;
mod update;
mod view;

fn default_db_name(name: &str, network: NetworkName) -> String {
    format!("{}.{}.db", name, network)
}

fn prepend_path(opt_base: Option<PathBuf>, name: &str) -> PathBuf {
    opt_base
        .map(|base| base.join(name))
        .unwrap_or_else(|| PathBuf::from(name))
}

pub fn open_ledger_db(args: &Cli) -> Result<ReadOnlyRocksDB> {
    if let Some(path) = args.ledger_db.as_deref() {
        ReadOnlyRocksDB::new(path).map_err(Into::into)
    } else {
        if let Some((cwd, envs)) = detect_amaru_process() {
            let path = envs
                .get(AMARU_LEDGER_DB_ENV)
                .cloned()
                .unwrap_or_else(|| default_db_name("ledger", args.network));
            return ReadOnlyRocksDB::new(&prepend_path(cwd, &path)).map_err(Into::into);
        }
        panic!("No ledger db provided, either through env or args");
    }
}

pub fn open_chain_db(args: &Cli) -> Result<ReadOnlyChainDB> {
    if let Some(path) = args.chain_db.as_deref() {
        RocksDBStore::open_for_readonly(&path.into()).map_err(Into::into)
    } else {
        if let Some((cwd, envs)) = detect_amaru_process() {
            let path = envs
                .get(AMARU_CHAIN_DB_ENV)
                .cloned()
                .unwrap_or_else(|| default_db_name("chain", args.network));
            return RocksDBStore::open_for_readonly(&prepend_path(cwd, &path)).map_err(Into::into);
        }
        panic!("No chain db provided, either through env or args");
    }
}
use crate::{app::App, store::rocks_db_switch::LedgerDB::*, tui::Tui};
use amaru_kernel::{EraHistory, network::NetworkName};
use amaru_stores::rocksdb::{RocksDB, RocksDBHistoricalStores, consensus::RocksDBStore};
use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use std::{env, path::PathBuf, str::FromStr};
use tracing::trace;

mod app;
mod app_state;
mod cli;
mod config;
mod controller;
mod errors;
mod logging;
mod model;
mod states;
mod store;
mod tui;
mod ui;
mod update;
mod view;

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let ledger_path_str = env::var("AMARU_LEDGER_DB").unwrap_or_else(|_| "ledgerdb".to_string());
    trace!("Using ledger path: {}", ledger_path_str);
    let ledger_path = PathBuf::from_str(&ledger_path_str)?;

    let chain_path_str = env::var("AMARU_CHAIN_DB").unwrap_or_else(|_| "chaindb".to_string());
    trace!("Using chain path: {}", chain_path_str);
    let chain_path = PathBuf::from_str(&chain_path_str)?;

    let era_history: &EraHistory = NetworkName::Preprod.into();
    let ledger_db = if let Ok(epoch) = env::var("AMARU_LEDGER_EPOCH") {
        trace!("Using epoch: {}", epoch);
        Snapshot(RocksDBHistoricalStores::for_epoch_with(
            ledger_path.as_path(),
            epoch.parse::<u64>()?.into(),
        )?)
    } else {
        Store(RocksDB::new(ledger_path.as_path(), era_history)?)
    };
    let chain_db = RocksDBStore::new(&chain_path, era_history)?;

    let args = Cli::parse();
    let mut tui = Tui::new()?
        .tick_rate(args.tick_rate)
        .frame_rate(args.frame_rate);
    let mut app: App = App::new(
        ledger_path_str,
        ledger_db,
        chain_path_str,
        chain_db,
        tui.get_frame().area(),
    )?;
    app.init()?;
    app.run(&mut tui).await?;
    Ok(())
}

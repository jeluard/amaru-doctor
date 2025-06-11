use crate::{app::App, store::rocks_db_switch::RocksDBSwitch};
use amaru_kernel::network::NetworkName;
use amaru_stores::rocksdb::{RocksDB, RocksDBHistoricalStores};
use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use std::{env, path::Path, sync::Arc};

mod app;
mod app_state;
mod cli;
mod config;
mod cursor;
mod errors;
mod logging;
mod mutator;
mod render;
mod shared;
mod states;
mod store;
mod tui;
mod ui;
mod window;

#[tokio::main]
async fn main() -> Result<()> {
    let ledger_path_str = &env::var("AMARU_LEDGER_DB").unwrap_or_else(|_| "ledgerdb".to_string());
    eprintln!("Using ledger path: {}", ledger_path_str);

    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let path = Path::new(ledger_path_str);
    if let Ok(epoch) = env::var("AMARU_LEDGER_EPOCH") {
        eprintln!("Using epoch: {}", epoch);
        let db_arc = Arc::new(RocksDBSwitch::Snapshot(
            RocksDBHistoricalStores::for_epoch_with(path, epoch.parse::<u64>()?)?,
        ));
        let mut app: App = App::new(ledger_path_str, args.tick_rate, args.frame_rate, db_arc)?;
        app.run().await?;
    } else {
        let db_arc = Arc::new(RocksDBSwitch::Store(RocksDB::new(
            path,
            NetworkName::Preprod.into(),
        )?));
        let mut app: App = App::new(ledger_path_str, args.tick_rate, args.frame_rate, db_arc)?;
        app.run().await?;
    };
    Ok(())
}

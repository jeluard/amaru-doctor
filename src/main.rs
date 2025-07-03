use crate::{app::App, tui::Tui};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::RocksDBStore};
use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use std::{path::PathBuf, str::FromStr};

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

    let args = Cli::parse();

    let ledger_path = PathBuf::from_str(&args.ledger_db)?;
    let chain_path = PathBuf::from_str(&args.chain_db)?;

    let ledger_db = ReadOnlyRocksDB::new(&ledger_path)?;
    let chain_db = RocksDBStore::open_for_readonly(&chain_path)?;

    let mut tui = Tui::new()?;
    let mut app: App = App::new(ledger_db, chain_db, tui.get_frame().area())?;
    app.run(&mut tui).await?;
    Ok(())
}

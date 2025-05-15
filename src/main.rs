use std::{env, path::Path};

use amaru_kernel::network::NetworkName;
use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::app::App;

mod action;
mod app;
mod cli;
mod components;
mod config;
mod errors;
mod logging;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    let db = RocksDB::new(
        Path::new(&env::var("AMARU_LEDGER_DB")?),
        NetworkName::Preprod.into(),
    )?;

    let pots = db.pots()?;
    println!("pots: {:?}", pots);

    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}

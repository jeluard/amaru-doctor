use std::{env, path::Path, sync::Arc};

use amaru_kernel::network::NetworkName;
use amaru_stores::rocksdb::RocksDB;
use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::app::App;

mod action;
mod app;
mod build;
mod cli;
mod components;
mod config;
mod errors;
mod focus;
mod logging;
mod shared;
mod to_list_item;
mod to_rich;
mod tui;
mod window;

#[tokio::main]
async fn main() -> Result<()> {
    let ledger_path_str = &env::var("AMARU_LEDGER_DB").unwrap_or_else(|_| {
        "ledger.db".to_string()
    });
    eprintln!("Using ledger path: {}", ledger_path_str);
    let db = RocksDB::new(Path::new(ledger_path_str), NetworkName::Preprod.into())?;
    let db_arc: Arc<RocksDB> = Arc::new(db);

    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(ledger_path_str, args.tick_rate, args.frame_rate, &db_arc)?;
    app.run().await?;
    Ok(())
}

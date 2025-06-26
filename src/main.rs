use crate::{
    app::App,
    store::{ROLedgerDB, chaindb::ROChainDB},
    tui::Tui,
};
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

    let args = Cli::parse();

    let ledger_path = PathBuf::from_str(&args.ledger_db)?;
    let chain_path = PathBuf::from_str(&args.chain_db)?;

    // let era_history: &EraHistory = NetworkName::Preprod.into();
    let ledger_db = if let Ok(epoch) = env::var("AMARU_LEDGER_EPOCH") {
        trace!("Using epoch: {}", epoch);
        ROLedgerDB::open_snapshot(&ledger_path, epoch.parse::<u64>()?.into())?
    } else {
        ROLedgerDB::open_live(&ledger_path)?
    };
    let chain_db = ROChainDB::open(chain_path)?;

    let mut tui = Tui::new()?;
    let mut app: App = App::new(ledger_db, chain_db, tui.get_frame().area())?;
    app.run(&mut tui).await?;
    Ok(())
}

use crate::{
    app::App,
    detection::{AMARU_CHAIN_DB_ENV, AMARU_LEDGER_DB_ENV, detect_amaru_process},
    otel::{TraceCollector, TraceReceiver},
    tui::Tui,
};
use amaru_kernel::network::NetworkName;
use amaru_stores::rocksdb::{
    ReadOnlyRocksDB,
    consensus::{ReadOnlyChainDB, RocksDBStore},
};
use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use ratatui_splash_screen::{SplashConfig, SplashScreen};
use core::panic;
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceServiceServer;
use std::{path::PathBuf, sync::Arc, time::Duration};
use tokio::task;
use tonic::transport::Server;

mod app;
mod app_state;
mod cli;
mod config;
mod controller;
mod detection;
mod errors;
mod logging;
mod model;
mod otel;
mod states;
mod store;
mod tui;
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

static SPLASH_CONFIG: SplashConfig = SplashConfig {
    image_data: include_bytes!("../resources/splash.png"),
    sha256sum: None,
    render_steps: 24,
    use_colors: false,
};

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let collector = Arc::new(TraceCollector::new(10_000, 5_000));
    let collector_clone = collector.clone();
    task::spawn(async move {
        let addr = "0.0.0.0:4317".parse().unwrap();
        Server::builder()
            .add_service(TraceServiceServer::new(TraceReceiver::new(collector)))
            .serve(addr)
            .await
            .unwrap();
    });

    let args = Cli::parse();

    let mut tui = Tui::new()?.mouse(true);

    let mut splash_screen = SplashScreen::new(SPLASH_CONFIG)?;
    while !splash_screen.is_rendered() {
        tui.draw(|frame| {
            frame.render_widget(&mut splash_screen, frame.area());
        })?;
        std::thread::sleep(Duration::from_millis(100));
    }

    let mut app: App = App::new(
        open_ledger_db(&args)?,
        open_chain_db(&args)?,
        collector_clone,
        tui.get_frame().area(),
    )?;
    app.run(&mut tui).await?;
    Ok(())
}

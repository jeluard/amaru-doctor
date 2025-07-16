use crate::{
    app::App,
    otel::{MetricsCollector, MetricsReceiver, TraceCollector, TraceReceiver},
    tui::Tui,
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::RocksDBStore};
use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use opentelemetry_proto::tonic::collector::metrics::v1::metrics_service_server::MetricsServiceServer;
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceServiceServer;
use std::{path::PathBuf, str::FromStr, sync::Arc};
use tokio::task;
use tonic::transport::Server;

mod app;
mod app_state;
mod cli;
mod config;
mod controller;
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

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let collector = Arc::new(TraceCollector::new(10_000, 5_000));
    let metrics_collector = Arc::new(MetricsCollector::new(10_000, 5_000));
    let collector_clone = collector.clone();
    let metrics_collector_clone = metrics_collector.clone();
    task::spawn(async move {
        let addr = "0.0.0.0:4317".parse().unwrap();
        Server::builder()
            .add_service(TraceServiceServer::new(TraceReceiver::new(collector)))
            .add_service(MetricsServiceServer::new(MetricsReceiver::new(
                metrics_collector,
            )))
            .serve(addr)
            .await
            .unwrap();
    });

    let args = Cli::parse();

    let ledger_path = PathBuf::from_str(&args.ledger_db)?;
    let chain_path = PathBuf::from_str(&args.chain_db)?;

    let ledger_db = ReadOnlyRocksDB::new(&ledger_path)?;
    let chain_db = RocksDBStore::open_for_readonly(&chain_path)?;

    let mut tui = Tui::new()?.mouse(true);
    let mut app: App = App::new(
        ledger_db,
        chain_db,
        collector_clone,
        metrics_collector_clone,
        tui.get_frame().area(),
    )?;
    app.run(&mut tui).await?;
    Ok(())
}

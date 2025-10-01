use amaru_doctor::{
    app::App,
    cli::Cli,
    open_chain_db, open_ledger_db,
    otel::{ingestor::TraceIngestor, service::AmaruTraceService},
    tui::Tui,
};
use anyhow::Result;
use clap::Parser;
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceServiceServer;
use std::time::Duration;
use tokio::task;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    amaru_doctor::logging::init()?;

    let collector = TraceIngestor::new(10_000, Duration::from_secs(10 * 60));
    let tracegraph_snapshot = collector.snapshot();
    task::spawn(async move {
        let addr = "0.0.0.0:4317".parse().unwrap();
        Server::builder()
            .add_service(TraceServiceServer::new(AmaruTraceService::new(collector)))
            .serve(addr)
            .await
            .unwrap();
    });

    let args = Cli::parse();

    let mut tui = Tui::default().mouse(true);

    let mut app: App = App::new(
        open_ledger_db(&args.ledger_db, &args.network)?,
        open_chain_db(&args.chain_db, &args.network)?,
        tracegraph_snapshot,
        tui.get_frame().area(),
    )?;
    app.run(&mut tui).await?;

    Ok(())
}

use amaru_doctor::{
    app::App,
    cli::Cli,
    open_chain_db, open_ledger_db,
    otel::{TraceCollector, TraceReceiver},
    tui::Tui,
};
use anyhow::Result;
use clap::Parser;
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceServiceServer;
use std::sync::Arc;
use tokio::task;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    amaru_doctor::logging::init()?;

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

    let mut tui = Tui::default().mouse(true);

    let mut app: App = App::new(
        open_ledger_db(&args)?,
        open_chain_db(&args)?,
        collector_clone,
        tui.get_frame().area(),
    )?;
    app.run(&mut tui).await?;
    Ok(())
}

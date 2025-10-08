use std::time::Duration;

use amaru_doctor::{
    app::App, cli::Cli, open_chain_db, open_ledger_db, otel::service::OtelCollectorService,
    prometheus::service::MetricsPoller, tui::Tui,
};
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    amaru_doctor::logging::init()?;

    let otel_service = OtelCollectorService::new("0.0.0.0:4317");
    let otel_handle = otel_service.start();

    let metrics_service =
        MetricsPoller::new("http://0.0.0.0:8889/metrics", Duration::from_millis(100));
    let metrics_handle = metrics_service.start();

    let args = Cli::parse();
    let mut tui = Tui::default().mouse(true);

    let mut app: App = App::new(
        open_ledger_db(&args.ledger_db, &args.network)?,
        open_chain_db(&args.chain_db, &args.network)?,
        otel_handle.snapshot,
        metrics_handle.receiver,
        tui.get_frame().area(),
    )?;
    app.run(&mut tui).await?;

    Ok(())
}

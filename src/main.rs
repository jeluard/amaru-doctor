use amaru_doctor::{
    app::App, cli::Cli, model::button::InputEvent, open_chain_db, open_ledger_db,
    otel::service::OtelCollectorService, tui::Tui,
};
use anyhow::Result;
use clap::Parser;
use std::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    amaru_doctor::logging::init()?;

    let otel_service = OtelCollectorService::new("0.0.0.0:4317");
    let otel_handle = otel_service.start();

    let args = Cli::parse();
    let mut tui = Tui::default().mouse(true);
    let (_, dummy_input_events) = mpsc::channel::<InputEvent>();

    let mut app: App = App::new(
        open_ledger_db(&args.ledger_db, &args.network)?,
        open_chain_db(&args.chain_db, &args.network)?,
        otel_handle.snapshot,
        dummy_input_events,
        tui.get_frame().area(),
    )?;
    app.run(&mut tui).await?;

    Ok(())
}

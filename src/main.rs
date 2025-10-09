use std::{io::stdout, time::Duration};

use amaru_doctor::{
    app::App,
    cli::Cli,
    open_chain_db, open_ledger_db,
    otel::service::{OtelCollectorHandle, OtelCollectorService},
    prometheus::service::{MetricsPoller, MetricsPollerHandle},
    tui::Tui,
};
use anyhow::Result;
use clap::Parser;
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use mousefood::{
    EmbeddedBackend, EmbeddedBackendConfig, embedded_graphics::geometry, prelude::Bgr565,
};
use ratatui::prelude::{Backend, CrosstermBackend};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    amaru_doctor::logging::init()?;
    let otel_service = OtelCollectorService::new("0.0.0.0:4317");
    let otel_handle = otel_service.start();

    let metrics_service =
        MetricsPoller::new("http://0.0.0.0:8889/metrics", Duration::from_millis(100));
    let metrics_handle = metrics_service.start();

    let args = Cli::parse();

    match args.backend.as_deref() {
        Some("sim_emb") => {
            info!("Using simulated embedded backend");
            let mut display = SimulatorDisplay::<Bgr565>::new(geometry::Size::new(512, 256));
            let backend = get_embedded_backend(&mut display);
            run_app(backend, otel_handle, metrics_handle, &args).await?;
        }
        _ => {
            info!("Using crossterm backend");
            let backend = CrosstermBackend::new(stdout());
            run_app(backend, otel_handle, metrics_handle, &args).await?;
        }
    };

    Ok(())
}

fn get_embedded_backend<'a>(
    display: &'a mut SimulatorDisplay<Bgr565>,
) -> EmbeddedBackend<'a, SimulatorDisplay<Bgr565>, Bgr565> {
    let mut simulator_window = Window::new(
        "mousefood simulator",
        &OutputSettings {
            scale: 1,
            max_fps: 30,
            ..Default::default()
        },
    );

    let backend_config = EmbeddedBackendConfig {
        flush_callback: Box::new(move |display| {
            simulator_window.update(display);
            if simulator_window.events().any(|e| e == SimulatorEvent::Quit) {
                panic!("simulator window closed");
            }
        }),
        ..Default::default()
    };
    EmbeddedBackend::new(display, backend_config)
}

async fn run_app<B: Backend>(
    backend: B,
    otel_handle: OtelCollectorHandle,
    metrics_handle: MetricsPollerHandle,
    args: &Cli,
) -> Result<()> {
    let mut tui = Tui::new(backend)?.mouse(true);

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

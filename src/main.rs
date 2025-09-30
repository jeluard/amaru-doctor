use amaru_doctor::{app::App, cli::Cli, open_chain_db, open_ledger_db, tui::Tui};
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    amaru_doctor::logging::init()?;

    let args = Cli::parse();

    let mut tui = Tui::default().mouse(true);

    let mut app: App = App::new(
        open_ledger_db(&args.ledger_db, &args.network)?,
        open_chain_db(&args.chain_db, &args.network)?,
        tui.get_frame().area(),
    )?;
    app.run(&mut tui).await?;
    Ok(())
}

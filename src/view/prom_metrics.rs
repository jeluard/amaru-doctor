use crate::{model::prom_metrics::PromMetricsViewState, view::time_series::render_chart};
use anyhow::Result;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

pub fn render_prom_metrics(
    frame: &mut Frame,
    area: Rect,
    state: &PromMetricsViewState,
    is_focused: bool,
) -> Result<()> {
    let mut block = Block::default()
        .title("Prometheus Metrics")
        .borders(Borders::ALL);
    if is_focused {
        block = block.border_style(Style::default().fg(Color::Blue));
    }

    // TODO: Come back to this and make it pretty
    //
    // let text_lines = match state.metrics() {
    //     Some(metrics) => metrics.to_rich_text().unwrap_lines(),
    //     None => vec!["No metrics.".into()],
    // };
    // let text_height = text_lines.len() as u16;
    // let top_and_bottom = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([Constraint::Length(text_height), Constraint::Fill(1)])
    //     .split(block.inner(area));

    let chart_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(area);

    frame.render_widget(&block, area);
    // frame.render_widget(Paragraph::new(text_lines), top_and_bottom[0]);
    render_chart(
        frame,
        chart_chunks[0],
        "Chain Density",
        Color::Cyan,
        state.density(),
    );
    render_chart(
        frame,
        chart_chunks[1],
        "CPU Utilization %",
        Color::LightYellow,
        state.cpu_util(),
    );
    render_chart(
        frame,
        chart_chunks[2],
        "Live Disk Read",
        Color::LightYellow,
        state.disk_live_read(),
    );
    render_chart(
        frame,
        chart_chunks[3],
        "Live Disk Write",
        Color::LightYellow,
        state.disk_live_write(),
    );

    Ok(())
}

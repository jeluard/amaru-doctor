use crate::{
    model::prom_metrics::PromMetricsViewState,
    view::time_series::{MetricKind, render_chart},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

pub fn render_prom_metrics(
    frame: &mut Frame,
    area: Rect,
    state: &PromMetricsViewState,
    _is_focused: bool,
) {
    let block = Block::default()
        .title("Prometheus Metrics")
        .borders(Borders::ALL);

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
    render_chart(
        frame,
        chart_chunks[0],
        "Memory Usage",
        Color::Cyan,
        state.memory(),
        MetricKind::Bytes,
    );
    render_chart(
        frame,
        chart_chunks[1],
        "CPU Utilization %",
        Color::Green,
        state.cpu_util(),
        MetricKind::Percentage,
    );
    render_chart(
        frame,
        chart_chunks[2],
        "Live Disk Read",
        Color::Magenta,
        state.disk_live_read(),
        MetricKind::Bytes,
    );
    render_chart(
        frame,
        chart_chunks[3],
        "Live Disk Write",
        Color::Yellow,
        state.disk_live_write(),
        MetricKind::Bytes,
    );
}

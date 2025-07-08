use crate::otel::MetricEvent;
use chrono::TimeZone;
use chrono::Utc;
use color_eyre::Result;
use ratatui::layout::Constraint;
use ratatui::widgets::{Cell, Row, Table};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn render_metrics_snapshot(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    metrics: Vec<MetricEvent>,
    is_focused: bool,
) -> Result<()> {
    let mut block = Block::default().title(title).borders(Borders::ALL);
    if is_focused {
        block = block
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::White));
    }

    let rows = metrics.iter().map(|event| {
        let timestamp = if event.timestamp > 0 {
            Utc
                .timestamp_nanos(event.timestamp as i64)
                .format("%Y-%m-%d %H:%M:%S%.3f")
                .to_string()
        } else {
            "N/A".to_string()
        };

        Row::new(vec![
            Cell::from(event.name.clone()),
            Cell::from(event.metric_type.clone()),
            Cell::from(event.value.clone()),
            Cell::from(event.unit.clone()),
            Cell::from(timestamp),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(30),
            Constraint::Length(12),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(26),
        ],
    )
    .header(Row::new(vec!["Name", "Type", "Value", "Unit", "Timestamp"]))
    .block(block)
    .column_spacing(1);

    frame.render_widget(table, area);

    Ok(())
}
use crate::model::time_series::TimeSeries;
use ratatui::{
    prelude::*,
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};

fn format_unix_time_label(ts: f64) -> String {
    let seconds_in_day = 24 * 3600;
    let secs = (ts as u64) % seconds_in_day;
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

const ONE_KB: f64 = 1024.0;
const ONE_MB: f64 = 1024.0 * ONE_KB;
const ONE_GB: f64 = 1024.0 * ONE_MB;

fn format_mib_label(value: f64) -> String {
    if value.abs() >= ONE_GB {
        format!("{:.2} GiB", value / ONE_GB)
    } else if value.abs() >= ONE_MB {
        format!("{:.2} MiB", value / ONE_MB)
    } else if value.abs() >= ONE_KB {
        format!("{:.1} KiB", value / ONE_KB)
    } else {
        format!("{:.1} B", value)
    }
}

pub enum MetricKind {
    Bytes,
    Percentage,
}

fn y_axis_for(kind: &MetricKind, bounds: [f64; 2]) -> Axis<'_> {
    let [min_y, max_y] = bounds;

    let (min_padded, max_padded) = if (max_y - min_y).abs() < f64::EPSILON {
        // If the line is flat, create a small window around it
        (min_y - 1.0, max_y + 1.0)
    } else {
        // Add 10% padding
        let range = max_y - min_y;
        let padding = range * 0.1;
        (min_y - padding, max_y + padding)
    };
    let new_range = max_padded - min_padded;

    let labels = (0..=4)
        .map(|i| {
            let y = min_padded + (i as f64) * new_range / 4.0;
            let label = match kind {
                MetricKind::Bytes => format_mib_label(y),
                MetricKind::Percentage => format!("{:.1}%", y),
            };
            Span::from(label)
        })
        .collect::<Vec<Span>>();

    Axis::default()
        .style(Style::default().fg(Color::Gray))
        .bounds([min_padded, max_padded])
        .labels(labels)
}

pub fn render_chart(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    color: Color,
    time_series: &TimeSeries,
    kind: MetricKind,
) {
    let sma_data = time_series.sma_data();
    let (x_bounds, y_bounds) = time_series.get_bounds();

    let sma_dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(color))
        .data(&sma_data);

    let [min_x, max_x] = x_bounds;
    let x_ticks = (0..=4)
        .map(|i| {
            let x = min_x + (i as f64) * (max_x - min_x) / 4.0;
            Span::from(format_unix_time_label(x))
        })
        .collect::<Vec<Span>>();

    let x_axis = Axis::default()
        .style(Style::default().fg(Color::Gray))
        .bounds(x_bounds)
        .labels(x_ticks);

    let chart = Chart::new(vec![sma_dataset])
        .block(Block::default().title(title).borders(Borders::ALL))
        .x_axis(x_axis)
        .y_axis(y_axis_for(&kind, y_bounds));

    frame.render_widget(chart, area);
}

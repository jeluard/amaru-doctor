use crate::model::time_series::TimeSeries;
use ratatui::{
    prelude::*,
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
    if value >= ONE_GB {
        // Show GiB if large
        format!("{:.2} GiB", value / ONE_GB)
    } else if value >= ONE_MB {
        // Show GiB if large
        format!("{:.2} MiB", value / ONE_MB)
    } else {
        format!("{:.1} B", value / ONE_KB)
    }
}

pub enum MetricKind {
    Bytes,
    Percentage,
}

fn y_axis_for(kind: &MetricKind, bounds: [f64; 2]) -> Axis<'_> {
    let [min_y, max_y] = bounds;
    let axis: Axis<'_> = Axis::default()
        .style(Style::default().fg(Color::Gray))
        .bounds(bounds);
    match kind {
        MetricKind::Bytes => {
            let y_ticks = (0..=4)
                .map(|i| {
                    let y = min_y + (i as f64) * (max_y - min_y) / 4.0;
                    Span::from(format_mib_label(y))
                })
                .collect::<Vec<Span>>();

            axis.labels(y_ticks)
        }
        MetricKind::Percentage => {
            let y_ticks = vec![
                Span::from(format!("{:.2}", min_y)),
                Span::from(format!("{:.2}", max_y)),
            ];

            axis.labels(y_ticks)
        }
    }
}

pub fn render_chart(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    color: Color,
    time_series: &TimeSeries,
    kind: MetricKind,
) {
    let data = time_series.data();
    let (x_bounds, y_bounds) = time_series.get_bounds();

    let dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(color))
        .data(&data);
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

    let chart = Chart::new(vec![dataset])
        .block(Block::default().title(title).borders(Borders::ALL))
        .x_axis(x_axis)
        .y_axis(y_axis_for(&kind, y_bounds));

    frame.render_widget(chart, area);
}

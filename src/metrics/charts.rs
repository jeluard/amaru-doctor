use crate::metrics::{
    metric_data::MetricData,
    model::{AmaruMetric, MetricKind},
};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset},
};
use tracing::error;

const ONE_KB: f64 = 1024.0;
const ONE_MB: f64 = 1024.0 * ONE_KB;
const ONE_GB: f64 = 1024.0 * ONE_MB;

pub struct ChartDatasetConfig<'a> {
    pub metric: AmaruMetric,
    pub data: &'a MetricData,
    pub label: &'a str,
    pub color: Color,
}

fn get_padded_y_bounds(bounds: [f64; 2]) -> [f64; 2] {
    let [min, max] = bounds;
    let (min_padded, max_padded) = if (max - min).abs() < f64::EPSILON {
        (min - 1.0, max + 1.0)
    } else {
        let range = max - min;
        let padding = range * 0.1;
        (min - padding, max + padding)
    };

    let min_clamped = if min >= 0.0 {
        // If the actual value is positive, don't let the padded value be negative
        min_padded.max(0.0)
    } else {
        // If the actual value is negative, use the usual padded value
        min_padded
    };

    [min_clamped, max_padded]
}

// Helper to generate labels for standard numeric axes
fn generate_axis_labels<F>(bounds: [f64; 2], formatter: F) -> Vec<Span<'static>>
where
    F: Fn(f64) -> String,
{
    let [min, max] = bounds;

    // Generate 3 tick marks: Min, Mid, Max
    let ticks = (0..=2).map(|i| min + (i as f64) * (max - min) / 2.0);
    ticks.map(|v| Span::from(formatter(v))).collect()
}

fn format_float(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    if v >= 100.0 {
        format!("{:.0}", v)
    } else if v >= 10.0 {
        format!("{:.1}", v)
    } else {
        format!("{:.2}", v)
    }
}

fn format_label(value: f64) -> String {
    let (divisor, unit) = if value >= ONE_GB {
        (ONE_GB, "GiB")
    } else if value >= ONE_MB {
        (ONE_MB, "MiB")
    } else {
        (ONE_KB, "KiB")
    };
    format!("{} {}", format_float(value / divisor), unit)
}

fn y_axis_for(kind: &MetricKind, bounds: [f64; 2]) -> Axis<'_> {
    let axis = Axis::default()
        .style(Style::default().fg(Color::Gray))
        .bounds(bounds);

    let labels = match kind {
        MetricKind::Bytes => generate_axis_labels(bounds, format_label),
        MetricKind::Count => generate_axis_labels(bounds, format_float),
        MetricKind::Duration => generate_axis_labels(bounds, |v| format!("{} s", format_float(v))),
        MetricKind::Percentage => vec![
            Span::from("0 %"),
            Span::from(format!("{} %", format_float(bounds[1]))),
        ],
    };

    axis.labels(labels)
}

/// Renders a chart with one or more datasets.
pub fn render_chart(
    frame: &mut Frame,
    area: Rect,
    datasets_config: &[ChartDatasetConfig],
    title: &str,
) {
    if datasets_config.is_empty() {
        return;
    }

    let kind = &datasets_config[0].metric.get_kind();
    if datasets_config.iter().any(|c| c.metric.get_kind() != *kind) {
        error!(
            "render_chart: Cannot render mixed MetricKinds in the same chart (Title: '{}')",
            title
        );
        return;
    }

    // Calculate the union of the bounds
    let (x_union, y_union) = datasets_config
        .iter()
        .filter_map(|c| c.data.sma_data.get_bounds())
        .fold(
            ([f64::MAX, f64::MIN], [f64::MAX, f64::MIN]),
            |(acc_x, acc_y), (curr_x, curr_y)| {
                (
                    [acc_x[0].min(curr_x[0]), acc_x[1].max(curr_x[1])],
                    [acc_y[0].min(curr_y[0]), acc_y[1].max(curr_y[1])],
                )
            },
        );

    // Clean the union
    let (x_union, y_union) = if x_union[0] > x_union[1] {
        ([0.0, 100.0], [0.0, 1.0])
    } else {
        (x_union, y_union)
    };

    let padded_y_bounds = get_padded_y_bounds(y_union);

    let data_cows: Vec<_> = datasets_config
        .iter()
        .map(|c| c.data.sma_data.data())
        .collect();

    let datasets: Vec<_> = datasets_config
        .iter()
        .zip(data_cows.iter())
        .map(|(config, data)| {
            Dataset::default()
                .name(config.label)
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(config.color))
                .data(data)
        })
        .collect();

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(title)
                .title_alignment(Alignment::Right)
                .borders(Borders::ALL),
        )
        .x_axis(Axis::default().bounds(x_union))
        .y_axis(y_axis_for(kind, padded_y_bounds));

    frame.render_widget(chart, area);
}

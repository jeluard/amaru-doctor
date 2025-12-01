use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout},
    model::time_series::TimeSeries,
    prometheus::model::{NodeMetrics, Timestamp},
    states::ComponentId,
};
use ratatui::{
    Frame,
    prelude::*,
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};
use std::any::Any;
use tokio::sync::mpsc::Receiver;

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

fn create_point_f64(metric: &(f64, Timestamp)) -> (f64, f64) {
    let x_time = metric.1.timestamp_micros() as f64 / 1_000_000.0;
    let y_value = metric.0;
    (x_time, y_value)
}

fn create_point_u64(metric: &(u64, Timestamp)) -> (f64, f64) {
    let x_time = metric.1.timestamp_micros() as f64 / 1_000_000.0;
    let y_value = metric.0;
    (x_time, y_value as f64)
}

pub struct PromMetricsComponent {
    id: ComponentId,
    receiver: Option<Receiver<NodeMetrics>>,
    last_metrics: Option<NodeMetrics>,
    memory: TimeSeries,
    cpu_util: TimeSeries,
    disk_live_read: TimeSeries,
    disk_total_read: TimeSeries,
    disk_live_write: TimeSeries,
    disk_total_write: TimeSeries,
}

impl PromMetricsComponent {
    pub fn new(id: ComponentId, receiver: Receiver<NodeMetrics>) -> Self {
        Self {
            id,
            receiver: Some(receiver),
            last_metrics: None,
            memory: TimeSeries::new(500, 20),
            cpu_util: TimeSeries::new(500, 20),
            disk_live_read: TimeSeries::new(500, 20),
            disk_total_read: TimeSeries::new(500, 20),
            disk_live_write: TimeSeries::new(500, 20),
            disk_total_write: TimeSeries::new(500, 20),
        }
    }

    pub fn sync(&mut self) {
        if let Some(receiver) = self.receiver.as_mut()
            && let Ok(new_metrics) = receiver.try_recv()
        {
            self.memory
                .add_point(create_point_u64(&new_metrics.mem_live_resident_bytes));
            self.cpu_util
                .add_point(create_point_f64(&new_metrics.cpu_percent_util));
            self.disk_live_read
                .add_point(create_point_u64(&new_metrics.disk_live_read_bytes));
            self.disk_total_read
                .add_point(create_point_u64(&new_metrics.disk_total_read_bytes));
            self.disk_live_write
                .add_point(create_point_u64(&new_metrics.disk_live_write_bytes));
            self.disk_total_write
                .add_point(create_point_u64(&new_metrics.disk_total_write_bytes));

            self.last_metrics = Some(new_metrics);
        }
    }
}

impl Component for PromMetricsComponent {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn calculate_layout(&self, area: Rect, _s: &AppState) -> ComponentLayout {
        let mut layout = ComponentLayout::new();
        layout.insert(self.id, area);
        layout
    }

    fn render(&self, f: &mut Frame, _s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };

        let block = Block::default();
        f.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        render_chart(
            f,
            top_chunks[0],
            "Memory",
            Color::Red,
            &self.memory,
            MetricKind::Bytes,
        );
        render_chart(
            f,
            top_chunks[1],
            "CPU Utilization",
            Color::Blue,
            &self.cpu_util,
            MetricKind::Percentage,
        );
        render_chart(
            f,
            bottom_chunks[0],
            "Disk Read",
            Color::Green,
            &self.disk_live_read,
            MetricKind::Bytes,
        );
        render_chart(
            f,
            bottom_chunks[1],
            "Disk Write",
            Color::Yellow,
            &self.disk_live_write,
            MetricKind::Bytes,
        );
    }
}

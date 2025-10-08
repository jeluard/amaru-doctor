use crate::{
    model::time_series::TimeSeries,
    prometheus::model::{NodeMetrics, Timestamp},
};
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct PromMetricsViewState {
    receiver: Receiver<NodeMetrics>,
    last_metrics: Option<NodeMetrics>,
    density: TimeSeries,
    cpu_util: TimeSeries,
    disk_live_read: TimeSeries,
    disk_total_read: TimeSeries,
    disk_live_write: TimeSeries,
    disk_total_write: TimeSeries,
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

impl PromMetricsViewState {
    pub fn new(receiver: Receiver<NodeMetrics>) -> Self {
        Self {
            receiver,
            last_metrics: None,
            density: TimeSeries::new(500),
            cpu_util: TimeSeries::new(500),
            disk_live_read: TimeSeries::new(500),
            disk_total_read: TimeSeries::new(500),
            disk_live_write: TimeSeries::new(500),
            disk_total_write: TimeSeries::new(500),
        }
    }

    pub fn sync(&mut self) {
        if let Ok(new_metrics) = self.receiver.try_recv() {
            self.density
                .add_point(create_point_f64(&new_metrics.density));
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

    pub fn metrics(&self) -> &Option<NodeMetrics> {
        &self.last_metrics
    }

    pub fn density(&self) -> &TimeSeries {
        &self.density
    }

    pub fn cpu_util(&self) -> &TimeSeries {
        &self.cpu_util
    }

    pub fn disk_live_read(&self) -> &TimeSeries {
        &self.disk_live_read
    }

    pub fn disk_total_read(&self) -> &TimeSeries {
        &self.disk_total_read
    }

    pub fn disk_live_write(&self) -> &TimeSeries {
        &self.disk_live_write
    }

    pub fn disk_total_write(&self) -> &TimeSeries {
        &self.disk_total_write
    }
}

use crate::prometheus::model::NodeMetrics;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct PromMetricsViewState {
    receiver: Receiver<NodeMetrics>,
    latest_metrics: Option<NodeMetrics>,
}

impl PromMetricsViewState {
    pub fn new(receiver: Receiver<NodeMetrics>) -> Self {
        Self {
            receiver,
            latest_metrics: None,
        }
    }

    pub fn sync(&mut self) {
        if let Ok(new_metrics) = self.receiver.try_recv() {
            self.latest_metrics = Some(new_metrics);
        }
    }

    pub fn latest_metrics(&self) -> &Option<NodeMetrics> {
        &self.latest_metrics
    }
}

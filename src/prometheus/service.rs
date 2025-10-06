use crate::prometheus::{client::MetricsClient, model::NodeMetrics};
use std::time::Duration;
use tokio::{
    sync::mpsc,
    task::{self, JoinHandle},
    time,
};
use tracing::error;

pub struct MetricsPoller {
    endpoint: &'static str,
    interval: Duration,
}

pub struct MetricsPollerHandle {
    pub receiver: mpsc::Receiver<NodeMetrics>,
    pub task_handle: JoinHandle<()>,
}

impl MetricsPoller {
    pub fn new(endpoint: &'static str, interval: Duration) -> Self {
        Self { endpoint, interval }
    }

    pub fn start(self) -> MetricsPollerHandle {
        let (sender, receiver) = mpsc::channel(1);

        let task_handle = task::spawn(async move {
            let client = MetricsClient::new(self.endpoint);
            let mut tick_timer = time::interval(self.interval);

            loop {
                tick_timer.tick().await;
                match client.get_metrics().await {
                    Ok(metrics) => {
                        if sender.send(metrics).await.is_err() {
                            error!("Metrics receiver dropped. Stopping polling task.");
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Failed to fetch metrics: {:?}", e);
                    }
                }
            }
        });

        MetricsPollerHandle {
            receiver,
            task_handle,
        }
    }
}

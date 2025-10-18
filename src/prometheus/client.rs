use crate::prometheus::model::{MetricParserError, NodeMetrics};
use prometheus_parse::Scrape;
use reqwest::Client;

#[derive(Debug, thiserror::Error)]
pub enum MetricsClientError {
    #[error("Failed to connect to metrics endpoint: {0}")]
    Connection(#[from] reqwest::Error),

    #[error("Non-success status from metrics endpoint: {0}")]
    NonSuccessStatus(reqwest::StatusCode),

    #[error("Failed to parse scrape: {0}")]
    Parse(#[from] std::io::Error),

    #[error("Faile to translate scrape to node metrics: {0}")]
    Translate(#[from] MetricParserError),

    #[error("An unexpected error occurred: {0}")]
    Other(anyhow::Error),
}

pub struct MetricsClient {
    endpoint: &'static str,
    client: Client,
}

impl MetricsClient {
    pub fn new(endpoint: &'static str) -> Self {
        Self {
            endpoint,
            client: Client::new(),
        }
    }

    pub async fn get_metrics(&self) -> Result<NodeMetrics, MetricsClientError> {
        let response = self.client.get(self.endpoint).send().await?;

        if !response.status().is_success() {
            return Err(MetricsClientError::NonSuccessStatus(response.status()));
        }

        let response_text = response.text().await?;
        let lines = response_text.lines().map(ToString::to_string).map(Ok);
        let scrape = Scrape::parse(lines)?;
        let metrics: NodeMetrics = scrape.try_into()?;

        Ok(metrics)
    }
}

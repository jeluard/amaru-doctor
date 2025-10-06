use crate::prometheus::model::NodeMetrics;
use anyhow::Result;
use prometheus_parse::Scrape;
use reqwest::Client;

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

    pub async fn get_metrics(&self) -> Result<NodeMetrics> {
        let response_text = self.client.get(self.endpoint).send().await?.text().await?;
        let lines = response_text.lines().map(ToString::to_string).map(Ok);
        let scrape = Scrape::parse(lines)?;
        scrape.try_into()
    }
}

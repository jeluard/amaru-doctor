use anyhow::{Result, anyhow};
use prometheus_parse::{Sample, Scrape, Value};
use std::collections::HashMap;

/// Represents the metrics scraped from the node's Prometheus endpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeMetrics {
    // Block metrics
    pub block_number: u64,
    pub density: f64,
    pub epoch: u64,
    pub slot_in_epoch: u64,
    pub slot_num: u64,
    pub transactions_processed: u64,

    // General process metrics
    pub cpu_percent_util: f64,
    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub disk_total_read_bytes: u64,
    pub disk_total_write_bytes: u64,
    pub mem_available_virtual_bytes: u64,
    pub mem_live_resident_bytes: u64,
    pub open_files: u64,
    pub runtime_seconds: u64,
}

/// A helper struct to simplify accessing metrics from a scrape.
/// It builds a HashMap for quick lookups by metric name.
struct MetricParser<'a> {
    map: HashMap<&'a str, &'a Sample>,
}

impl<'a> MetricParser<'a> {
    /// Creates a new MetricParser from a prometheus_parse::Scrape.
    fn new(scrape: &'a Scrape) -> Self {
        let map = scrape
            .samples
            .iter()
            .map(|s| (s.metric.as_str(), s))
            .collect();
        Self { map }
    }

    /// Gets a raw Sample by metric name.
    fn get_sample(&self, name: &str) -> Result<&'a Sample> {
        self.map
            .get(name)
            .copied()
            .ok_or_else(|| anyhow!("Metric '{}' not found", name))
    }

    /// Gets a metric value and casts it to u64.
    fn get_u64(&self, name: &str) -> Result<u64> {
        let sample = self.get_sample(name)?;
        match sample.value {
            Value::Counter(v) | Value::Gauge(v) | Value::Untyped(v) => Ok(v as u64),
            _ => Err(anyhow!("Metric '{}' has unexpected type for u64", name)),
        }
    }

    /// Gets a metric value and casts it to f64.
    fn get_f64(&self, name: &str) -> Result<f64> {
        let sample = self.get_sample(name)?;
        match sample.value {
            Value::Counter(v) | Value::Gauge(v) | Value::Untyped(v) => Ok(v),
            _ => Err(anyhow!("Metric '{}' has unexpected type for f64", name)),
        }
    }
}

/// Implementation to convert a Scrape into our NodeMetrics struct.
impl TryFrom<Scrape> for NodeMetrics {
    type Error = anyhow::Error;

    fn try_from(scrape: Scrape) -> Result<Self, Self::Error> {
        let parser = MetricParser::new(&scrape);
        Ok(Self {
            // Block
            block_number: parser.get_u64("cardano_node_metrics_blockNum_int")?,
            density: parser.get_f64("cardano_node_metrics_density_real")?,
            epoch: parser.get_u64("cardano_node_metrics_epoch_int")?,
            slot_in_epoch: parser.get_u64("cardano_node_metrics_slotInEpoch_int")?,
            slot_num: parser.get_u64("cardano_node_metrics_slotNum_int")?,
            transactions_processed: parser.get_u64("cardano_node_metrics_txsProcessedNum_int")?,

            // Process
            cpu_percent_util: parser.get_f64("process_cpu_live")?,
            disk_read_bytes: parser.get_u64("process_disk_live_read")?,
            disk_write_bytes: parser.get_u64("process_disk_live_write")?,
            disk_total_read_bytes: parser.get_u64("process_disk_total_read")?,
            disk_total_write_bytes: parser.get_u64("process_disk_total_write")?,
            mem_available_virtual_bytes: parser.get_u64("process_memory_available_virtual")?,
            mem_live_resident_bytes: parser.get_u64("process_memory_live_resident")?,
            open_files: parser.get_u64("process_open_files")?,
            runtime_seconds: parser.get_u64("process_runtime")?,
        })
    }
}

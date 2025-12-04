use super::metric_data::MetricData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricKind {
    Bytes,
    Percentage,
    Count,
    Duration,
}

impl TryFrom<&str> for MetricKind {
    type Error = String;

    fn try_from(unit: &str) -> Result<Self, Self::Error> {
        match unit {
            "bytes" => Ok(MetricKind::Bytes),
            "%" => Ok(MetricKind::Percentage),
            "seconds" => Ok(MetricKind::Duration),
            "int" | "real" | "1" | "" => Ok(MetricKind::Count),
            other => Err(other.to_owned()),
        }
    }
}

#[derive(Debug, strum::Display, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmaruMetric {
    // Process
    ProcessCpuLive,
    ProcessMemoryLiveResident,
    ProcessMemoryAvailableVirtual,
    ProcessDiskLiveRead,
    ProcessDiskLiveWrite,
    ProcessDiskTotalRead,
    ProcessDiskTotalWrite,
    ProcessOpenFiles,
    ProcessRuntime,

    // Cardano
    CardanoBlockNum,
    CardanoEpoch,
    CardanoSlotInEpoch,
    CardanoSlotNum,
    CardanoDensity,
    CardanoTxsProcessed,
}

impl AmaruMetric {
    pub fn get_kind(&self) -> MetricKind {
        match self {
            Self::ProcessCpuLive | Self::CardanoDensity => MetricKind::Percentage,

            Self::ProcessMemoryLiveResident
            | Self::ProcessMemoryAvailableVirtual
            | Self::ProcessDiskLiveRead
            | Self::ProcessDiskLiveWrite
            | Self::ProcessDiskTotalRead
            | Self::ProcessDiskTotalWrite => MetricKind::Bytes,

            Self::ProcessRuntime => MetricKind::Duration,

            Self::ProcessOpenFiles
            | Self::CardanoBlockNum
            | Self::CardanoEpoch
            | Self::CardanoSlotInEpoch
            | Self::CardanoSlotNum
            | Self::CardanoTxsProcessed => MetricKind::Count,
        }
    }
}

impl TryFrom<(&str, &str)> for AmaruMetric {
    type Error = String;

    fn try_from(pair: (&str, &str)) -> Result<Self, Self::Error> {
        match pair {
            ("process_cpu_live", "%") => Ok(Self::ProcessCpuLive),
            ("process_memory_live_resident", "bytes") => Ok(Self::ProcessMemoryLiveResident),
            ("process_memory_available_virtual", "bytes") => {
                Ok(Self::ProcessMemoryAvailableVirtual)
            }
            ("process_disk_live_read", "bytes") => Ok(Self::ProcessDiskLiveRead),
            ("process_disk_live_write", "bytes") => Ok(Self::ProcessDiskLiveWrite),
            ("process_disk_total_read", "bytes") => Ok(Self::ProcessDiskTotalRead),
            ("process_disk_total_write", "bytes") => Ok(Self::ProcessDiskTotalWrite),
            ("process_open_files", "") => Ok(Self::ProcessOpenFiles), // Empty unit for count
            ("process_runtime", "seconds") => Ok(Self::ProcessRuntime),

            // Cardano Metrics
            ("cardano_node_metrics_blockNum_int", "int") => Ok(Self::CardanoBlockNum),
            ("cardano_node_metrics_epoch_int", "int") => Ok(Self::CardanoEpoch),
            ("cardano_node_metrics_slotInEpoch_int", "int") => Ok(Self::CardanoSlotInEpoch),
            ("cardano_node_metrics_slotNum_int", "int") => Ok(Self::CardanoSlotNum),
            ("cardano_node_metrics_txsProcessedNum_int", "int") => Ok(Self::CardanoTxsProcessed),
            ("cardano_node_metrics_density_real", "real") => Ok(Self::CardanoDensity),

            (name, unit) => Err(format!(
                "Unknown or invalid metric pair: ('{}', '{}')",
                name, unit
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricUpdate {
    pub metric: AmaruMetric,
    pub value: f64,
}

#[derive(Debug, Default)]
pub struct NodeMetrics {
    // Process
    pub process_cpu_live: MetricData,
    pub process_memory_live_resident: MetricData,
    pub process_memory_available_virtual: MetricData,
    pub process_disk_live_read: MetricData,
    pub process_disk_live_write: MetricData,
    pub process_disk_total_read: MetricData,
    pub process_disk_total_write: MetricData,
    pub process_open_files: MetricData,
    pub process_runtime: MetricData,

    // Cardano
    pub cardano_block_num: MetricData,
    pub cardano_epoch: MetricData,
    pub cardano_slot_in_epoch: MetricData,
    pub cardano_slot_num: MetricData,
    pub cardano_density: MetricData,
    pub cardano_txs_processed: MetricData,
}

impl NodeMetrics {
    pub fn handle_update(&mut self, update: MetricUpdate) {
        let field = match update.metric {
            AmaruMetric::ProcessCpuLive => &mut self.process_cpu_live,
            AmaruMetric::ProcessMemoryLiveResident => &mut self.process_memory_live_resident,
            AmaruMetric::ProcessMemoryAvailableVirtual => {
                &mut self.process_memory_available_virtual
            }
            AmaruMetric::ProcessDiskLiveRead => &mut self.process_disk_live_read,
            AmaruMetric::ProcessDiskLiveWrite => &mut self.process_disk_live_write,
            AmaruMetric::ProcessDiskTotalRead => &mut self.process_disk_total_read,
            AmaruMetric::ProcessDiskTotalWrite => &mut self.process_disk_total_write,
            AmaruMetric::ProcessOpenFiles => &mut self.process_open_files,
            AmaruMetric::ProcessRuntime => &mut self.process_runtime,

            AmaruMetric::CardanoBlockNum => &mut self.cardano_block_num,
            AmaruMetric::CardanoEpoch => &mut self.cardano_epoch,
            AmaruMetric::CardanoSlotInEpoch => &mut self.cardano_slot_in_epoch,
            AmaruMetric::CardanoSlotNum => &mut self.cardano_slot_num,
            AmaruMetric::CardanoDensity => &mut self.cardano_density,
            AmaruMetric::CardanoTxsProcessed => &mut self.cardano_txs_processed,
        };

        field.add_value(update.value);
    }
}

use crate::model::stats::{SmaProcessor, StatProcessor};
use crate::model::time_series::TimeSeries;

const MAX_DATA_POINTS: usize = 500;
const SMA_WINDOW: usize = 50;

#[derive(Debug)]
pub struct MetricData {
    pub x_counter: u64,
    pub raw_data: TimeSeries,
    sma_processor: SmaProcessor,
    pub sma_data: TimeSeries,
}

impl Default for MetricData {
    fn default() -> Self {
        Self {
            x_counter: 0,
            raw_data: TimeSeries::new(MAX_DATA_POINTS),
            sma_processor: SmaProcessor::default(),
            sma_data: TimeSeries::new(MAX_DATA_POINTS),
        }
    }
}

impl MetricData {
    pub fn add_value(&mut self, value: f64) {
        self.x_counter += 1;
        let x = self.x_counter as f64;
        self.raw_data.add_point((x, value));
        let val_to_drop = self.raw_data.get_recent_y(SMA_WINDOW);
        let sma_val = self.sma_processor.process(value, val_to_drop);
        self.sma_data.add_point((x, sma_val));
    }
}

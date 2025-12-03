/// A trait for processing a stream of data points.
pub trait StatProcessor: Send + Sync + std::fmt::Debug {
    /// Process a new value.
    /// `dropped_value` is the value leaving the sliding window, if the window is full.
    fn process(&mut self, new_value: f64, dropped_value: Option<f64>) -> f64;
}

#[derive(Debug, Default)]
pub struct NoOpProcessor;

impl StatProcessor for NoOpProcessor {
    fn process(&mut self, value: f64, _dropped: Option<f64>) -> f64 {
        value
    }
}

#[derive(Debug, Default, Clone)]
pub struct SmaProcessor {
    /// The current sum of the window
    sum: f64,
    /// How many items we've processed
    count: usize,
}

impl StatProcessor for SmaProcessor {
    fn process(&mut self, new_value: f64, dropped_value: Option<f64>) -> f64 {
        self.sum += new_value;

        if let Some(dropped) = dropped_value {
            self.sum -= dropped;
        } else {
            self.count += 1;
        }

        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }
}

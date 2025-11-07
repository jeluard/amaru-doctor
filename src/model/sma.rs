use std::collections::VecDeque;

/// Calculates a Simple Moving Average (SMA)
#[derive(Debug)]
pub struct SmaCalculator {
    window_size: usize,
    values: VecDeque<f64>,
    sum: f64,
}

impl SmaCalculator {
    pub fn new(window_size: usize) -> Self {
        assert!(window_size > 0, "window_size must be greater than 0");
        Self {
            window_size,
            values: VecDeque::with_capacity(window_size),
            sum: 0.0,
        }
    }

    /// Adds a new value and returns the updated SMA.
    pub fn add(&mut self, y_value: f64) -> f64 {
        self.sum += y_value;
        self.values.push_back(y_value);

        // If the window is full, remove the oldest value from the sum
        if self.values.len() > self.window_size {
            let y_old = self.values.pop_front().unwrap();
            self.sum -= y_old;
        }

        let len = self.values.len() as f64;
        if len == 0.0 { 0.0 } else { self.sum / len }
    }
}

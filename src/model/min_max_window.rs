use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

/// A helper struct that efficiently tracks the minimum and maximum values
/// of a sliding window using a BTreeMap to sort values.
#[derive(Clone, Debug, Default)]
pub struct MinMaxWindow {
    /// Maps a value to the count of times it appears in the current window.
    /// Used to handle duplicate values.
    counts: BTreeMap<OrderedFloat<f64>, usize>,
}

impl MinMaxWindow {
    /// Adds a value to the window.
    pub fn add(&mut self, value: f64) {
        *self.counts.entry(OrderedFloat(value)).or_insert(0) += 1;
    }

    /// Removes a value from the window.
    ///
    /// If the value's count drops to zero, it is removed from the map,
    /// potentially updating the min/max bounds.
    pub fn remove(&mut self, value: f64) {
        let Some(count) = self.counts.get_mut(&OrderedFloat(value)) else {
            return;
        };
        *count -= 1;
        if *count == 0 {
            self.counts.remove(&OrderedFloat(value));
        }
    }

    /// Returns the [min, max] of the current values.
    /// Returns `None` if the window is empty.
    pub fn bounds(&self) -> Option<[f64; 2]> {
        if let Some((min_key, _)) = self.counts.first_key_value() {
            // Safe to unwrap last if first exists
            let max_key = self.counts.last_key_value().unwrap().0;
            Some([min_key.into_inner(), max_key.into_inner()])
        } else {
            None
        }
    }
}

use ordered_float::OrderedFloat;
use std::{
    borrow::Cow,
    collections::{BTreeMap, VecDeque},
};

#[derive(Debug)]
pub struct TimeSeries {
    max_size: usize,
    data: VecDeque<(f64, f64)>,
    // These track the counts of X and Y values, used for efficiently calc'ing the X and
    // Y bounds (min and max)
    x_values: BTreeMap<OrderedFloat<f64>, usize>,
    y_values: BTreeMap<OrderedFloat<f64>, usize>,
}

fn add_to_map(map: &mut BTreeMap<OrderedFloat<f64>, usize>, value: f64) {
    *map.entry(OrderedFloat(value)).or_insert(0) += 1;
}

fn remove_from_map(map: &mut BTreeMap<OrderedFloat<f64>, usize>, value: f64) {
    let Some(count) = map.get_mut(&OrderedFloat(value)) else {
        return;
    };
    *count -= 1;

    if *count != 0 {
        return;
    }

    map.remove(&OrderedFloat(value));
}

impl TimeSeries {
    pub fn new(max_size: usize) -> Self {
        assert!(max_size > 0, "max_size must be greater than 0");
        Self {
            max_size,
            data: VecDeque::with_capacity(max_size),
            x_values: BTreeMap::new(),
            y_values: BTreeMap::new(),
        }
    }

    pub fn add_point(&mut self, point: (f64, f64)) {
        // If we're at capacity, remove the oldest point's values from the maps
        if self.data.len() == self.max_size {
            let old_point = self.data.pop_front().unwrap();
            remove_from_map(&mut self.x_values, old_point.0);
            remove_from_map(&mut self.y_values, old_point.1);
        }

        // Add the new point to the list and update the maps
        self.data.push_back(point);
        add_to_map(&mut self.x_values, point.0);
        add_to_map(&mut self.y_values, point.1);
    }

    pub fn get_bounds(&self) -> ([f64; 2], [f64; 2]) {
        let x_bounds = self.get_map_bounds(&self.x_values);
        let y_bounds = self.get_map_bounds(&self.y_values);

        (x_bounds, y_bounds)
    }

    fn get_map_bounds(&self, map: &BTreeMap<OrderedFloat<f64>, usize>) -> [f64; 2] {
        if let Some((min_key, _)) = map.first_key_value() {
            let max_key = map.last_key_value().unwrap().0;
            [min_key.into_inner(), max_key.into_inner()]
        } else {
            [0.0, 1.0]
        }
    }

    pub fn data(&self) -> Cow<'_, [(f64, f64)]> {
        // A `VecDeque` is a ring buffer--data may not be stored in a single contiguous
        // block of memory if it has "wrapped around" the internal buffer's boundary
        let (slice1, slice2) = self.data.as_slices();

        if slice2.is_empty() {
            // If the data is contiguous (has not wrapped), return a `Cow::Borrowed`
            // slice, avoiding memory allocation
            Cow::Borrowed(slice1)
        } else {
            // If the data has wrapped, combine the parts into a single slice and return
            // a `Cow::Owned` vector, requiring memory allocation
            Cow::Owned([slice1, slice2].concat())
        }
    }
}

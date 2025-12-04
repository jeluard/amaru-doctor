use crate::model::min_max_window::MinMaxWindow;
use std::{borrow::Cow, collections::VecDeque};

#[derive(Clone, Debug)]
pub struct TimeSeries {
    max_size: usize,
    data: VecDeque<(f64, f64)>,
    y_bounds: MinMaxWindow,
}

impl TimeSeries {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            data: VecDeque::with_capacity(max_size),
            y_bounds: MinMaxWindow::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Retrieves the Y-value at a specific offset from the end.
    /// offset 0 = the most recently added point.
    /// offset 1 = the point before that.
    pub fn get_recent_y(&self, offset: usize) -> Option<f64> {
        let len = self.data.len();
        if offset >= len {
            return None;
        }
        self.data.get(len - 1 - offset).map(|(_, y)| *y)
    }

    pub fn add_point(&mut self, point: (f64, f64)) {
        let (_, y) = point;

        if self.data.len() == self.max_size {
            let old_point = self.data.pop_front().unwrap();
            self.y_bounds.remove(old_point.1);
        }

        self.data.push_back(point);
        self.y_bounds.add(y);
    }

    pub fn get_bounds(&self) -> Option<([f64; 2], [f64; 2])> {
        let x_min = self.data.front()?.0;
        let x_max = self.data.back()?.0;

        let y_bounds = self.y_bounds.bounds()?;

        Some(([x_min, x_max], y_bounds))
    }

    pub fn data(&self) -> Cow<'_, [(f64, f64)]> {
        let (s1, s2) = self.data.as_slices();
        if s2.is_empty() {
            Cow::Borrowed(s1)
        } else {
            Cow::Owned([s1, s2].concat())
        }
    }
}

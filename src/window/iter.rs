pub struct WindowIter<T, I>
where
    I: Iterator<Item = T>,
{
    iter: I,
    buffer: Vec<T>,
    window_start: usize,
    window_size: usize,
}

impl<T, I> WindowIter<T, I>
where
    I: Iterator<Item = T>,
{
    pub fn new(iter: I, window_size: usize) -> Self {
        Self {
            iter,
            buffer: Vec::new(),
            window_start: 0,
            window_size,
        }
    }

    pub fn set_window_size(&mut self, new_size: usize) {
        self.window_size = new_size;
        self.fill_buffer();
    }

    pub fn view(&mut self) -> &[T] {
        self.fill_buffer();
        let end = (self.window_start + self.window_size).min(self.buffer.len());
        &self.buffer[self.window_start..end]
    }

    pub fn shift_up(&mut self) {
        if self.window_start > 0 {
            self.window_start -= 1;
        }
    }

    pub fn shift_down(&mut self) {
        self.fill_buffer();
        if self.window_start + self.window_size < self.buffer.len() {
            self.window_start += 1;
        } else if let Some(next_item) = self.iter.next() {
            self.buffer.push(next_item);
            self.window_start += 1;
        }
    }

    fn fill_buffer(&mut self) {
        while self.buffer.len() < self.window_start + self.window_size {
            if let Some(item) = self.iter.next() {
                self.buffer.push(item);
            } else {
                break;
            }
        }
    }
}

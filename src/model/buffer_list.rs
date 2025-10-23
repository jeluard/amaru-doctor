/// A trait representing a view into a potentially large or lazy-loaded dataset.
pub trait BufferList<T> {
    /// Ensures that the data source has loaded all items up to a given index.
    fn load_up_to(&mut self, index: usize);

    /// Returns a slice of the currently loaded data items.
    fn buffer(&self) -> &[T];

    /// Returns the total number of items if the data source is finite and fully known.
    fn total_len(&self) -> Option<usize>;
}

use crate::update::scroll::{ScrollDirection, ScrollableList};

/// A "dynamic list" struct that helps the UI when the underlying data is allowed to
/// change. To that end this
/// 1. Efficiently scrolls up and down and
/// 2. Retains the currently selected item
#[derive(Debug, Default, Clone)]
pub struct DynamicList<T: Clone + PartialEq> {
    items: Vec<T>,
    /// The index of the selected item.
    selected_index: Option<usize>,
}

impl<T: Clone + PartialEq> DynamicList<T> {
    /// Updates the list of items and re-syncs the selection. The selection is maintained
    /// (by value) if the item still exists in the new list.
    pub fn set_items(&mut self, items: Vec<T>) {
        // Get the value of the current selection before replacing the items.
        let old_selection = self.selection().cloned();
        self.items = items;

        // After updating items, find the old selection in the new list.
        self.selected_index = old_selection
            .as_ref()
            .and_then(|selected| self.items.iter().position(|item| item == selected));
    }

    /// Returns a reference to the currently selected item.
    pub fn selection(&self) -> Option<&T> {
        self.selected_index.and_then(|i| self.items.get(i))
    }

    /// Returns a reference to the items as a slice.
    pub fn _items(&self) -> &[T] {
        &self.items
    }

    /// Scrolls down using the cached index
    fn scroll_down(&mut self) {
        if self.items.is_empty() {
            self.selected_index = None;
            return;
        }

        // Allow wrapping
        let next_index = self
            .selected_index
            .map_or(0, |i| (i + 1) % self.items.len());
        self.selected_index = Some(next_index);
    }

    /// Scrolls up using the cached index
    fn scroll_up(&mut self) {
        if self.items.is_empty() {
            self.selected_index = None;
            return;
        }

        let len = self.items.len();
        let next_index = self.selected_index.map_or(len - 1, |i| (i + len - 1) % len);
        self.selected_index = Some(next_index);
    }
}

impl<T: Clone + PartialEq> ScrollableList for DynamicList<T> {
    fn scroll(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Up => self.scroll_up(),
            ScrollDirection::Down => self.scroll_down(),
        }
    }
}

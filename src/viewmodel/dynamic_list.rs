use ratatui::{Frame, layout::Rect};

use crate::{ui::to_list_item::ToListItem, view::list::ListViewState};

/// A stateful component for a list whose entire dataset can be replaced
/// dynamically. It combines the data model (`Vec<T>`) with the view state
/// (`ListViewState`).
#[derive(Debug)]
pub struct DynamicListViewModel<T: Clone + PartialEq> {
    items: Vec<T>,
    view: ListViewState,
}

impl<T: Clone + PartialEq + ToListItem> DynamicListViewModel<T> {
    pub fn new(title: &'static str) -> Self {
        Self {
            items: Vec::new(),
            view: ListViewState::new(title),
        }
    }

    /// Replaces the list of items and intelligently re-syncs the selection.
    pub fn set_items(&mut self, items: Vec<T>) {
        let old_selection = self.selected_item().cloned();
        self.items = items;
        let len = self.items.len();

        // Find the old item in the new list and update the view's selection.
        let new_selected_index = old_selection
            .as_ref()
            .and_then(|selected| self.items.iter().position(|item| item == selected))
            .unwrap_or(0); // Default to the first item if not found or if nothing was selected.

        self.view.select(new_selected_index, len);
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.items.get(self.view.selected_index())
    }

    pub fn select_index_by_row(&mut self, relative_row: usize) {
        self.view
            .select_index_by_row(relative_row, self.items.len());
    }

    pub fn cursor_back(&mut self) {
        self.view.cursor_back();
    }

    pub fn cursor_next(&mut self) {
        self.view.cursor_next(Some(self.items.len()));
    }

    /// Natural scroll up (see later content)
    pub fn advance_window(&mut self) {
        self.view.advance_window(Some(self.items.len()));
    }

    /// Natural scroll down (see earlier content)
    pub fn retreat_window(&mut self) {
        self.view.retreat_window();
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        self.view.draw(f, area, &self.items, is_focused);
    }
}

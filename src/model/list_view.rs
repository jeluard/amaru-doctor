use crate::{
    model::{buffer_list::BufferList, streaming_iter::StreamingIter},
    ui::to_list_item::ToListItem,
    view::list::ListViewState,
};
use ratatui::{prelude::Frame, prelude::Rect};

/// A stateful component that encapsulates both the data model (`StreamingIter`)
/// and the view state (`ListViewState`) for a list, providing an ergonomic API.
pub struct ListModelView<T> {
    iter: StreamingIter<T>,
    view: ListViewState,
}

impl<T: ToListItem> ListModelView<T> {
    pub fn new(
        title: &'static str,
        iter: impl Iterator<Item = T> + Send + Sync + 'static,
        initial_buffer_size: usize,
    ) -> Self {
        Self {
            iter: StreamingIter::new(iter, initial_buffer_size),
            view: ListViewState::new(title),
        }
    }

    /// Updates the component's height and ensures the streaming iter is loaded
    /// for the new view.
    pub fn set_height(&mut self, new_height: usize) {
        self.view.set_height(new_height);
        let required_index = self.view.max_visible_index();
        self.iter.load_up_to(required_index);
    }

    /// Returns a reference to the currently selected item.
    pub fn selected_item(&self) -> Option<&T> {
        let index = self.view.selected_index();
        self.iter.buffer().get(index)
    }

    /// Sets the selected index based on a row clicked within the visible
    /// window.
    pub fn select_index_by_row(&mut self, relative_row: usize) {
        self.view
            .select_index_by_row(relative_row, self.iter.buffer().len());
    }

    /// Moves the selection cursor up by one.
    pub fn cursor_back(&mut self) {
        self.view.cursor_back();
        let required_index = self.view.max_visible_index();
        self.iter.load_up_to(required_index);
    }

    /// Moves the selection cursor down by one.
    pub fn cursor_next(&mut self) {
        self.view.cursor_next(self.iter.total_len());
        let required_index = self.view.max_visible_index();
        self.iter.load_up_to(required_index);
    }

    /// Retreats the window. Visually the content moves down the screen.
    pub fn retreat_window(&mut self) {
        self.view.retreat_window();
        let required_index = self.view.max_visible_index();
        self.iter.load_up_to(required_index);
    }

    /// Advances the window. Visually the content moves up the screen.
    pub fn advance_window(&mut self) {
        self.view.advance_window(self.iter.total_len());
        let required_index = self.view.max_visible_index();
        self.iter.load_up_to(required_index);
    }

    /// Draws the list component.
    pub fn draw(&self, f: &mut Frame, area: Rect, is_focsued: bool) {
        self.view.draw(f, area, &self.iter, is_focsued);
    }
}

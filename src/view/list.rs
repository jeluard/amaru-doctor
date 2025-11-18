use crate::{model::buffer_list::BufferList, ui::to_list_item::ToListItem};
use ratatui::{
    prelude::{Frame, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use tracing::debug;

/// A component that manages the UI state (offset and selection) for a
/// scrollable list. It operates on a data source provided to it externally.
#[derive(Debug)]
pub struct ListViewState {
    title: &'static str,
    offset: usize,
    selected: usize,
    height: usize,
}

impl ListViewState {
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            offset: 0,
            selected: 0,
            height: 0,
        }
    }

    pub fn title(&self) -> &'static str {
        self.title
    }

    pub fn max_visible_index(&self) -> usize {
        self.offset + self.height
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn set_height(&mut self, new_height: usize) {
        self.height = new_height;
    }

    /// Sets the selected index directly, clamping it to valid bounds and
    /// adjusting the view offset to ensure the selection is visible.
    pub fn select(&mut self, index: usize, total_len: usize) {
        if total_len == 0 {
            self.selected = 0;
            return;
        }
        self.selected = index.min(total_len - 1);

        // Adjust the view offset to bring the selection into view if it's off-screen.
        if self.selected < self.offset {
            self.offset = self.selected;
        } else if self.height > 0 && self.selected >= self.offset + self.height {
            self.offset = self.selected - self.height + 1;
        }
    }

    /// Sets the selected index based on a row clicked within the visible
    /// viewport.
    pub fn select_index_by_row(&mut self, relative_row: usize, buffer_len: usize) {
        let absolute_index = self.offset + relative_row;

        if absolute_index >= buffer_len {
            debug!(
                "Clicked on empty row (index {} is >= buffer len {}). Ignoring selection.",
                absolute_index, buffer_len
            );
            return;
        }

        debug!(
            "Will set selected index to {} from relative row {}: {:?}",
            absolute_index, relative_row, self
        );
        self.selected = absolute_index;
        debug!("Did set selected index: {:?}", self);
    }

    pub fn cursor_back(&mut self) {
        debug!("ListViewState: cursor_back - BEFORE: {:?}", self);
        if self.selected > 0 {
            self.selected -= 1;
            debug!(
                "ListViewState: cursor_back - selected decremented to {}",
                self.selected
            );
            // If the cursor moves above the visible window, move the window up.
            if self.selected < self.offset {
                debug!(
                    "ListViewState: cursor_back - selected ({}) < offset ({}), adjusting offset",
                    self.selected, self.offset
                );
                self.offset = self.selected;
            }
        } else {
            debug!("ListViewState: cursor_back - selected is already 0, no change");
        }
        debug!("ListViewState: cursor_back - AFTER: {:?}", self);
    }

    pub fn cursor_next(&mut self, total_len: Option<usize>) {
        debug!(
            "ListViewState: cursor_next - BEFORE: {:?}, total_len: {:?}",
            self, total_len
        );
        let next_i = self.selected + 1;
        debug!("ListViewState: cursor_next - next_i: {}", next_i);

        if let Some(len) = total_len {
            debug!("ListViewState: cursor_next - total_len is Some({})", len);
            if next_i >= len {
                debug!(
                    "ListViewState: cursor_next - next_i ({}) >= total_len ({}), returning",
                    next_i, len
                );
                return;
            }
        } else {
            debug!("ListViewState: cursor_next - total_len is None, not checking upper bound yet");
        }

        self.selected = next_i;
        debug!(
            "ListViewState: cursor_next - selected updated to {}",
            self.selected
        );

        // If the cursor moves below the visible window, move the window down.
        if self.height > 0 && self.selected >= self.offset + self.height {
            debug!(
                "ListViewState: cursor_next - selected ({}) >= offset ({}) + height ({}), adjusting offset",
                self.selected, self.offset, self.height
            );
            self.offset = self.selected + 1 - self.height;
        } else {
            debug!("ListViewState: cursor_next - no offset adjustment needed");
        }
        debug!("ListViewState: cursor_next - AFTER: {:?}", self);
    }

    /// Moves the visible window up (content moves down).
    pub fn retreat_window(&mut self) {
        debug!("ListViewState: retreat_window - BEFORE: {:?}", self);
        let new_offset = self.offset.saturating_sub(1);
        debug!("ListViewState: retreat_window - new_offset: {}", new_offset);

        if self.offset != new_offset {
            debug!(
                "ListViewState: retreat_window - offset changed from {} to {}",
                self.offset, new_offset
            );
            self.offset = new_offset;

            // Check if the selection is now off-screen (below the window).
            let last_visible_index = self.offset + self.height - 1;
            debug!(
                "ListViewState: retreat_window - last_visible_index: {}, selected: {}",
                last_visible_index, self.selected
            );
            if self.selected > last_visible_index {
                debug!(
                    "ListViewState: retreat_window - selected ({}) > last_visible_index ({}), adjusting selected",
                    self.selected, last_visible_index
                );
                // If so, move it to the new last visible item.
                self.selected = last_visible_index;
            } else {
                debug!("ListViewState: retreat_window - selected is within bounds");
            }
        } else {
            debug!("ListViewState: retreat_window - offset did not change");
        }
        debug!("ListViewState: retreat_window - AFTER: {:?}", self);
    }

    /// Moves the visible window down (content moves up).
    pub fn advance_window(&mut self, total_len: Option<usize>) {
        debug!(
            "ListViewState: advance_window - BEFORE: {:?}, total_len: {:?}",
            self, total_len
        );
        if self.height == 0 {
            debug!("ListViewState: advance_window - height is 0, returning");
            return;
        }
        let new_offset = self.offset + 1;
        let final_offset;

        if let Some(len) = total_len {
            debug!("ListViewState: advance_window - total_len is Some({})", len);
            let max_offset = len.saturating_sub(self.height);
            debug!(
                "ListViewState: advance_window - max_offset: {}, new_offset: {}",
                max_offset, new_offset
            );
            final_offset = new_offset.min(max_offset);
        } else {
            debug!("ListViewState: advance_window - total_len is None, no max_offset calculation");
            final_offset = new_offset;
        }
        debug!(
            "ListViewState: advance_window - final_offset: {}",
            final_offset
        );

        if self.offset != final_offset {
            debug!(
                "ListViewState: advance_window - offset changed from {} to {}",
                self.offset, final_offset
            );
            self.offset = final_offset;

            // Check if the selection is now off-screen (above the window).
            if self.selected < self.offset {
                debug!(
                    "ListViewState: advance_window - selected ({}) < offset ({}), adjusting selected",
                    self.selected, self.offset
                );
                // If so, move it to the new first visible item.
                self.selected = self.offset;
            } else {
                debug!("ListViewState: advance_window - selected is within bounds");
            }
        } else {
            debug!("ListViewState: advance_window - offset did not change");
        }
        debug!("ListViewState: advance_window - AFTER: {:?}", self);
    }

    pub fn draw<T, B>(&self, f: &mut Frame, area: Rect, data: &B, is_focused: bool)
    where
        T: ToListItem,
        B: BufferList<T>,
    {
        let mut block = Block::default().borders(Borders::ALL).title(self.title);
        if is_focused {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let items: Vec<ListItem> = data.buffer().iter().map(ToListItem::to_list_item).collect();

        let list_widget = List::new(items).block(block).highlight_symbol(">> ");

        let mut list_state = ListState::default()
            .with_offset(self.offset)
            .with_selected(Some(self.selected));

        f.render_stateful_widget(list_widget, area, &mut list_state);
    }
}

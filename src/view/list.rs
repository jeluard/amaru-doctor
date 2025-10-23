use crate::{
    model::{buffer_list::BufferList, streaming_iter::StreamingIter},
    ui::to_list_item::ToListItem,
};
use ratatui::{
    prelude::{Frame, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use tracing::debug;

/// A component that manages the UI state (offset and selection) for a scrollable list.
/// It operates on a data source provided to it externally.
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

    pub fn max_visible_index(&self) -> usize {
        self.offset + self.height
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn set_height(&mut self, new_height: usize) {
        self.height = new_height;
    }

    /// Sets the selected index based on a row clicked within the visible window.
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
        debug!("Will scroll up cursor: {:?}", self);
        if self.selected > 0 {
            self.selected -= 1;
            // If the cursor moves above the visible window, move the window up.
            if self.selected < self.offset {
                self.offset = self.selected;
            }
        }
        debug!("Did scroll up cursor: {:?}", self);
    }

    pub fn cursor_next(&mut self, total_len: Option<usize>) {
        debug!("Will scroll down cursor: {:?}", self);
        let next_i = self.selected + 1;

        if let Some(len) = total_len
            && next_i >= len
        {
            return;
        }

        self.selected = next_i;

        // If the cursor moves below the visible window, move the window down.
        if self.height > 0 && self.selected >= self.offset + self.height {
            self.offset = self.selected + 1 - self.height;
        }
        debug!("Did scroll down cursor: {:?}", self);
    }

    /// Moves the visible window up (content moves down).
    pub fn retreat_window(&mut self) {
        debug!("Will retreat window: {:?}", self);
        let new_offset = self.offset.saturating_sub(1);

        if self.offset != new_offset {
            self.offset = new_offset;

            // Check if the selection is now off-screen (below the window).
            let last_visible_index = self.offset + self.height - 1;
            if self.selected > last_visible_index {
                // If so, move it to the new last visible item.
                self.selected = last_visible_index;
            }
        }
        debug!("Did retreat window: {:?}", self);
    }

    /// Moves the visible window down (content moves up).
    pub fn advance_window(&mut self, total_len: Option<usize>) {
        debug!("Will advance window: {:?}", self);
        if self.height == 0 {
            return;
        }
        let new_offset = self.offset + 1;
        let final_offset;

        if let Some(len) = total_len {
            let max_offset = len.saturating_sub(self.height);
            final_offset = new_offset.min(max_offset);
        } else {
            final_offset = new_offset;
        }

        if self.offset != final_offset {
            self.offset = final_offset;

            // Check if the selection is now off-screen (above the window).
            if self.selected < self.offset {
                // If so, move it to the new first visible item.
                self.selected = self.offset;
            }
        }
        debug!("Did advance window: {:?}", self);
    }

    pub fn draw<T>(&self, f: &mut Frame, area: Rect, iter: &StreamingIter<T>, is_focused: bool)
    where
        T: ToListItem,
    {
        let mut block = Block::default().borders(Borders::ALL).title(self.title);
        if is_focused {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let items: Vec<ListItem> = iter.buffer().iter().map(ToListItem::to_list_item).collect();

        let list_widget = List::new(items).block(block).highlight_symbol(">> ");

        let mut list_state = ListState::default()
            .with_offset(self.offset)
            .with_selected(Some(self.selected));

        f.render_stateful_widget(list_widget, area, &mut list_state);
    }
}

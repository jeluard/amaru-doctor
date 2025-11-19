use crate::{
    components::list::ListModel, model::async_provider::AsyncProvider,
    ui::to_list_item::ToListItem, view::list::ListViewState,
};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::marker::PhantomData;
use tokio::sync::mpsc::error::TryRecvError;

/// A data model for a list whose data is loaded asynchronously.
/// It implements `ListModel` so it can be wrapped by a `ListComponent` or used
/// inside a `SearchListComponent`.
pub struct AsyncListModel<T>
where
    T: ToListItem + Send + Sync + 'static,
{
    provider: AsyncProvider<T>,
    pub buffer: Vec<T>,
    pub view: ListViewState,
    is_loading: bool,
    _phantom: PhantomData<T>,
}

impl<T> AsyncListModel<T>
where
    T: ToListItem + Send + Sync + 'static,
{
    pub fn new(title: &'static str, provider: AsyncProvider<T>) -> Self {
        Self {
            provider,
            buffer: Vec::new(),
            view: ListViewState::new(title),
            is_loading: true,
            _phantom: PhantomData,
        }
    }

    /// Polls the provider for new data without blocking.
    /// This should be called on every application tick via the parent component.
    pub fn poll_data(&mut self) {
        // If we're no longer loading, do nothing.
        if !self.is_loading {
            return;
        }

        // Poll up to 100 items per tick to avoid blocking
        // the UI for too long on a fast data influx.
        for _ in 0..100 {
            match self.provider.rx.try_recv() {
                Ok(item) => {
                    self.buffer.push(item);
                }
                Err(TryRecvError::Empty) => {
                    // No more data right now
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    // The provider finished its query
                    self.is_loading = false;
                    break;
                }
            }
        }
    }
}

impl<T> ListModel for AsyncListModel<T>
where
    T: ToListItem + Send + Sync + 'static,
{
    type Item = T;

    fn draw(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        if self.is_loading && self.buffer.is_empty() {
            // Show a Loading message
            let mut block = Block::default()
                .title(self.view.title())
                .borders(Borders::ALL);

            if is_focused {
                block = block
                    .border_style(Style::default().fg(Color::Blue))
                    .title_style(Style::default().fg(Color::White));
            }

            let loading_text = vec![
                Line::from(""),
                Line::from(Span::styled("Loading...", Style::default().fg(Color::Gray))),
            ];

            let widget = Paragraph::new(loading_text)
                .alignment(Alignment::Center)
                .block(block);

            f.render_widget(widget, area);
        } else {
            // Render the list as normal
            self.view.draw(f, area, &self.buffer, is_focused);
        }
    }

    fn selected_item(&self) -> Option<&T> {
        self.buffer.get(self.view.selected_index())
    }

    fn select_index_by_row(&mut self, relative_row: usize) {
        self.view
            .select_index_by_row(relative_row, self.buffer.len());
    }

    fn cursor_back(&mut self) {
        self.view.cursor_back();
    }

    fn cursor_next(&mut self) {
        self.view.cursor_next(Some(self.buffer.len()));
    }

    fn retreat_window(&mut self) {
        self.view.retreat_window();
    }

    fn advance_window(&mut self) {
        self.view.advance_window(Some(self.buffer.len()));
    }

    fn set_height(&mut self, new_height: usize) {
        self.view.set_height(new_height);
    }
}

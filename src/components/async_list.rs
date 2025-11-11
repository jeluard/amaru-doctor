use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, MouseScrollDirection},
    model::async_provider::AsyncProvider,
    states::{Action, ComponentId, WidgetSlot},
    ui::to_list_item::ToListItem,
    update::scroll::ScrollDirection,
    view::list::ListViewState,
};
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::marker::PhantomData;
use tokio::sync::mpsc::error::TryRecvError;
use tracing::info;

/// A stateful component for a list whose data is loaded asynchronously.
pub struct AsyncListModel<T>
where
    T: ToListItem + Send + Sync + 'static,
{
    id: ComponentId,
    slot: WidgetSlot,
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
    pub fn new(
        id: ComponentId,
        slot: WidgetSlot,
        title: &'static str,
        provider: AsyncProvider<T>,
    ) -> Self {
        Self {
            id,
            slot,
            provider,
            buffer: Vec::new(),
            view: ListViewState::new(title),
            is_loading: true,
            _phantom: PhantomData,
        }
    }

    /// Polls the provider for new data without blocking.
    /// This should be called on every application tick.
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

    pub fn set_height(&mut self, new_height: usize) {
        self.view.set_height(new_height);
    }

    pub fn selected_item(&self) -> Option<&T> {
        self.buffer.get(self.view.selected_index())
    }

    pub fn select_index_by_row(&mut self, relative_row: usize) {
        self.view
            .select_index_by_row(relative_row, self.buffer.len());
    }
}

impl<T> Component for AsyncListModel<T>
where
    T: ToListItem + Send + Sync + 'static,
{
    fn id(&self) -> ComponentId {
        self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn calculate_layout(&self, area: Rect, _s: &AppState) -> ComponentLayout {
        let mut layout = ComponentLayout::new();
        layout.insert(self.id, area);
        layout
    }

    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };

        let is_focused = s.layout_model.is_focused(self.slot);

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

    fn handle_scroll(&mut self, direction: ScrollDirection) -> Vec<Action> {
        match direction {
            ScrollDirection::Up => self.view.cursor_back(),
            ScrollDirection::Down => self.view.cursor_next(Some(self.buffer.len())),
        }
        Vec::new()
    }

    fn handle_click(&mut self, area: Rect, row: u16, _col: u16) -> Vec<Action> {
        let relative_row = row.saturating_sub(area.y + 1) as usize;
        self.select_index_by_row(relative_row);
        Vec::new()
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> Vec<Action> {
        info!("No key handling for AsyncListModel");
        Vec::new()
    }

    fn handle_mouse_scroll(&mut self, direction: MouseScrollDirection) -> Vec<Action> {
        match direction {
            MouseScrollDirection::Up => self.view.cursor_back(),
            MouseScrollDirection::Down => self.view.cursor_next(Some(self.buffer.len())),
        }
        Vec::new()
    }

    fn handle_mouse_drag(&mut self, direction: ScrollDirection) -> Vec<Action> {
        match direction {
            ScrollDirection::Up => self.view.advance_window(Some(self.buffer.len())),
            ScrollDirection::Down => self.view.retreat_window(),
        }
        Vec::new()
    }
}

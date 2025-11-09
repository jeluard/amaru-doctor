use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout},
    model::cursor::Cursor,
    states::{Action, ComponentId, WidgetSlot},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, ToLine},
    widgets::{Block, Borders, Tabs},
};
use std::{any::Any, marker::PhantomData};
use strum::IntoEnumIterator;
use tracing::info;

/// A stateful component that renders a horizontal, selectable tab bar.
pub struct TabsComponent<T>
where
    T: IntoEnumIterator + ToLine + Copy + PartialEq + Eq,
{
    id: ComponentId,
    slot: WidgetSlot,
    pub cursor: Cursor<T>,
    _phantom: PhantomData<T>,
}

impl<T> TabsComponent<T>
where
    T: IntoEnumIterator + ToLine + Copy + PartialEq + Eq,
{
    pub fn new(id: ComponentId, slot: WidgetSlot) -> Self {
        Self {
            id,
            slot,
            cursor: Cursor::new(T::iter().collect()).expect("TabsComponent must have options"),
            _phantom: PhantomData,
        }
    }

    /// Determines which tab was clicked based on the column and updates the
    /// cursor. Returns true if a tab was selected, false otherwise.
    pub fn select_by_column(&mut self, area: Rect, column: u16) -> bool {
        const DIVIDER: &str = " | ";
        let divider_width = DIVIDER.len() as u16;
        let mut current_col = area.x.saturating_add(1); // +1 for border
        let max_col = area.x.saturating_add(area.width).saturating_sub(1); // -1 for border

        for (index, item) in self.cursor.iter().enumerate() {
            if current_col >= max_col {
                break;
            }

            let tab_line = item.to_line();
            let tab_width = tab_line.width() as u16;

            let next_tab_start_col = current_col
                .saturating_add(tab_width)
                .saturating_add(divider_width);

            let clickable_end_col = next_tab_start_col.min(max_col);

            if column >= current_col && column < clickable_end_col {
                self.cursor.select_index(index);
                return true;
            }

            // Move to the start of the next tab for the next iteration.
            current_col = next_tab_start_col;
        }
        false
    }

    pub fn selected(&self) -> T {
        *self.cursor.current()
    }
}

impl<T> Component for TabsComponent<T>
where
    T: IntoEnumIterator + ToLine + Copy + PartialEq + Eq + Send + Sync + 'static,
{
    fn id(&self) -> ComponentId {
        self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
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
        let mut block = Block::default().borders(Borders::ALL);
        if is_focused {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let tab_lines: Vec<Line> = self.cursor.iter().map(ToLine::to_line).collect();
        let tabs_widget = Tabs::new(tab_lines)
            .select(self.cursor.index())
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(tabs_widget, area);
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Vec<Action> {
        match key.code {
            KeyCode::Left => {
                self.cursor.next_back();
            }
            KeyCode::Right => {
                self.cursor.non_empty_next();
            }
            _ => {}
        }
        vec![Action::UpdateLayout(Rect::default())]
    }

    fn handle_click(&mut self, area: Rect, row: u16, col: u16) -> Vec<Action> {
        if row >= area.y && row < area.y + area.height && self.select_by_column(area, col) {
            // Click succeeded, trigger a layout update
            return vec![Action::UpdateLayout(Rect::default())];
        }
        Vec::new()
    }

    fn handle_scroll(&mut self, _direction: super::ScrollDirection) -> Vec<Action> {
        info!("Nothing to scroll");
        Vec::new()
    }
}

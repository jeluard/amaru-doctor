use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout},
    model::cursor::Cursor,
    states::{Action, ComponentId},
    tui::Event,
};
use crossterm::event::{KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, ToLine},
    widgets::{Block, Borders, Tabs},
};
use std::{any::Any, marker::PhantomData};
use strum::IntoEnumIterator;

/// A stateful component that renders a horizontal, selectable tab bar.
pub struct TabsComponent<T>
where
    T: IntoEnumIterator + ToLine + Copy + PartialEq + Eq,
{
    id: ComponentId,
    border: bool,
    pub cursor: Cursor<T>,
    _phantom: PhantomData<T>,
}

impl<T> TabsComponent<T>
where
    T: IntoEnumIterator + ToLine + Copy + PartialEq + Eq,
{
    pub fn new(id: ComponentId, border: bool) -> Self {
        Self {
            id,
            border,
            cursor: Cursor::new(T::iter().collect()).expect("TabsComponent must have options"),
            _phantom: PhantomData,
        }
    }

    pub fn selected(&self) -> T {
        *self.cursor.current()
    }

    /// Determines which tab was clicked based on the column and updates the
    /// cursor. Returns true if a tab was selected, false otherwise.
    pub fn select_by_column(&mut self, area: Rect, column: u16) -> bool {
        const DIVIDER: &str = " | ";
        let divider_width = DIVIDER.len() as u16;
        let mut current_col = area.x.saturating_add(1);
        let max_col = area.x.saturating_add(area.width).saturating_sub(1);

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

    pub fn handle_click(&mut self, area: Rect, row: u16, col: u16) -> Vec<Action> {
        if row >= area.y && row < area.y + area.height {
            self.select_by_column(area, col);
        }
        Vec::new()
    }

    pub fn render_focused(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        let mut block = Block::default();
        if self.border {
            block = block.borders(Borders::ALL);
        }

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

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Left => {
                    self.cursor.next_back();
                }
                KeyCode::Right => {
                    self.cursor.non_empty_next();
                }
                _ => {}
            },
            Event::Mouse(mouse) => {
                if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                    return self.handle_click(area, mouse.row, mouse.column);
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, _f: &mut Frame, _s: &AppState, _l: &ComponentLayout) {}
}

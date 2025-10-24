// src/model/tabs_state.rs (or a similar new file)

use crate::model::cursor::Cursor;
use anyhow::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, ToLine},
    widgets::{Block, Borders, Tabs},
};
use strum::IntoEnumIterator;

pub struct TabsState<T>
where
    T: IntoEnumIterator + ToLine,
{
    pub cursor: Cursor<T>,
}

impl<T> TabsState<T>
where
    T: IntoEnumIterator + ToLine,
{
    pub fn new() -> Result<Self> {
        Ok(Self {
            cursor: Cursor::new(T::iter().collect())?,
        })
    }

    pub fn select_index(&mut self, index: usize) {
        self.cursor.select_index(index);
    }

    /// Determines which tab was clicked based on the column and updates the cursor.
    /// Returns true if a tab was selected, false otherwise.
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
                self.select_index(index);
                return true;
            }

            // Move to the start of the next tab for the next iteration.
            current_col = next_tab_start_col;
        }
        false
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
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

        frame.render_widget(tabs_widget, area);
    }
}

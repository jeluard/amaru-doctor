use crate::model::cursor::Cursor;
use anyhow::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, ToLine},
    widgets::{Block, Borders, Tabs},
};

pub fn render_tabs<T: ToLine>(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    cursor: &Cursor<T>,
    is_focused: bool,
) -> Result<()> {
    let mut block = Block::default().borders(Borders::ALL).title(title);

    if is_focused {
        block = block
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::White));
    }

    let tab_lines: Vec<Line> = cursor.iter().map(ToLine::to_line).collect();
    let tabs_widget = Tabs::new(tab_lines)
        .select(cursor.index())
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(tabs_widget, area);
    Ok(())
}

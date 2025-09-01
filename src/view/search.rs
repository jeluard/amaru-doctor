use anyhow::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_search_query(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    search_query: &str,
    is_focused: bool,
) -> Result<()> {
    let mut block = Block::default().title(title).borders(Borders::ALL);

    if is_focused {
        block = block
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::White));
    }

    let paragraph = Paragraph::new(Line::from(Span::raw(search_query))).block(block);
    frame.render_widget(paragraph, area);

    Ok(())
}

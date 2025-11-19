use crate::ui::RichText;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn _draw_empty_list(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    text: &str,
    is_focused: bool,
) {
    let mut block = Block::default().title(title).borders(Borders::ALL);
    if is_focused {
        block = block
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::White));
    }

    let lines = RichText::Single(Span::raw(text.to_owned()));
    let widget = Paragraph::new(lines.unwrap_lines())
        .wrap(Wrap { trim: true })
        .block(block);
    frame.render_widget(widget, area);
}

use crate::ui::{RichText, ToRichText};
use amaru_consensus::BlockHeader;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_header(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    header_opt_opt: Option<Option<&BlockHeader>>,
    is_focused: bool,
) {
    let mut block = Block::default().title(title).borders(Borders::ALL);
    if is_focused {
        block = block
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::White));
    }

    let lines = match header_opt_opt {
        Some(header_opt) => match header_opt {
            Some(header) => header.to_rich_text(),
            None => RichText::Single(Span::raw("No header found")),
        },
        None => RichText::Single(Span::raw(
            "To search a header, enter its hash in the search bar and press enter",
        )),
    };
    let widget = Paragraph::new(lines.unwrap_lines())
        .wrap(Wrap { trim: true })
        .block(block);
    frame.render_widget(widget, area);
}

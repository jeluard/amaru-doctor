use crate::ui::{RichText, ToRichText};
use amaru_kernel::RawBlock;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_block(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    block_opt_opt: Option<Option<&RawBlock>>,
    is_focused: bool,
) {
    let mut block = Block::default().title(title).borders(Borders::ALL);
    if is_focused {
        block = block
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::White));
    }

    let lines = match block_opt_opt {
        Some(block_res) => match block_res {
            Some(block) => block.to_rich_text(),
            None => RichText::Single(Span::raw("No block found")),
        },
        None => RichText::Single(Span::raw(
            "To search a block, enter its hash in the search bar and press enter",
        )),
    };
    let widget = Paragraph::new(lines.unwrap_lines())
        .wrap(Wrap { trim: true })
        .block(block);
    frame.render_widget(widget, area);
}

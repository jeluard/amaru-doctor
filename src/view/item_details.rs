use crate::ui::{RichText, ToRichText};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
};

// TODO: Give this its own state obj like List
pub fn draw_details<T: ToRichText>(
    frame: &mut Frame,
    area: Rect,
    title: String,
    item_opt: Option<&T>,
    is_focused: bool,
) {
    let mut block = Block::default().title(title).borders(Borders::ALL);
    if is_focused {
        block = block
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::White));
    }

    let lines = item_opt
        .map(|i| i.to_rich_text())
        .unwrap_or(RichText::Single(Span::raw("Nothing selected")))
        .unwrap_lines();

    let widget = Paragraph::new(lines).wrap(Wrap { trim: true }).block(block);
    frame.render_widget(widget, area);
}

use crate::{model::otel_view::OtelViewState, ui::ToRichText};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_span(frame: &mut Frame, area: Rect, state: &OtelViewState, is_focused: bool) {
    let mut block = Block::default().title("Span Details").borders(Borders::ALL);
    if is_focused {
        block = block.border_style(Style::default().fg(Color::Blue));
    }

    let lines = if let Some(focused_span) = &state.focused_span {
        focused_span.to_rich_text().unwrap_lines()
    } else {
        vec![Line::from("No span focused.")]
    };

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

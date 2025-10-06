use crate::{model::prom_metrics::PromMetricsViewState, ui::ToRichText};
use anyhow::Result;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_prom_metrics(
    frame: &mut Frame,
    area: Rect,
    state: &PromMetricsViewState,
    is_focused: bool,
) -> Result<()> {
    let mut block = Block::default().title("Span Details").borders(Borders::ALL);
    if is_focused {
        block = block.border_style(Style::default().fg(Color::Blue));
    }

    let lines = match state.latest_metrics() {
        Some(metrics) => metrics.to_rich_text().unwrap_lines(),
        None => vec![Line::from("No metrics.")],
    };

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);

    Ok(())
}

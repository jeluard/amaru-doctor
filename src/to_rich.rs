use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub mod account;
pub mod utxo;

const LABEL_STYLE: Style = Style::new().fg(Color::Gray).add_modifier(Modifier::BOLD);

pub enum RichText {
    Lines(Vec<Line<'static>>),
    Single(Span<'static>),
}

pub trait ToRichText {
    fn into_rich_text(self) -> RichText;
}

impl RichText {
    pub fn unwrap_lines(self) -> Vec<Line<'static>> {
        match self {
            RichText::Lines(v) => v,
            RichText::Single(span) => vec![Line::from(vec![span])],
        }
    }
}

fn labeled(label: String, value: RichText, value_style: Style) -> Vec<Line<'static>> {
    match value {
        RichText::Single(span) => vec![Line::from(vec![
            Span::styled(format!("{label}: "), LABEL_STYLE),
            span.style(value_style),
        ])],
        RichText::Lines(lines) => lines
            .into_iter()
            .enumerate()
            .map(|(i, line)| {
                if i == 0 {
                    let mut spans = vec![Span::styled(format!("{label}: "), LABEL_STYLE)];
                    spans.extend(line.spans);
                    Line::from(spans)
                } else {
                    line
                }
            })
            .collect(),
    }
}

use amaru_kernel::{Bytes, CertificatePointer, Hash, KeyValuePairs, Nullable, RationalNumber, Set};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use std::fmt;

pub mod account;
pub mod block_issuer;
pub mod drep;
pub mod header;
pub mod pool;
pub mod proposal;
pub mod utxo;

const LABEL_STYLE: Style = Style::new().fg(Color::Gray).add_modifier(Modifier::BOLD);

pub enum RichText {
    Lines(Vec<Line<'static>>),
    Single(Span<'static>),
}

pub trait ToRichText {
    fn to_rich_text(&self) -> RichText;
}

impl RichText {
    pub fn unwrap_lines(self) -> Vec<Line<'static>> {
        match self {
            RichText::Lines(v) => v,
            RichText::Single(span) => vec![Line::from(vec![span])],
        }
    }
}

impl FromIterator<Line<'static>> for RichText {
    fn from_iter<I: IntoIterator<Item = Line<'static>>>(iter: I) -> Self {
        RichText::Lines(iter.into_iter().collect())
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

fn labeled_default<T>(label: &str, value: &T) -> Vec<Line<'static>>
where
    T: ToRichText,
{
    labeled(label.to_owned(), value.to_rich_text(), Style::default())
}

fn labeled_default_single<T>(label: &str, value: T) -> Vec<Line<'static>>
where
    T: fmt::Display,
{
    labeled(
        label.to_owned(),
        RichText::Single(Span::raw(value.to_string())),
        Style::default(),
    )
}

fn labeled_default_opt<T>(label: &str, value: Option<&T>) -> Vec<Line<'static>>
where
    T: ToRichText,
{
    labeled(
        label.to_owned(),
        value
            .map(|v| v.to_rich_text())
            .unwrap_or(RichText::Single(Span::raw("None"))),
        Style::default(),
    )
}

fn labeled_default_opt_single<T>(label: &str, value: Option<T>) -> Vec<Line<'static>>
where
    T: fmt::Display,
{
    labeled(
        label.to_owned(),
        RichText::Single(Span::raw(
            value.map_or("None".to_string(), |v| v.to_string()),
        )),
        Style::default(),
    )
}

pub struct RationalNumberDisplay<'a>(pub &'a RationalNumber);

impl<'a> fmt::Display for RationalNumberDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0.numerator, self.0.denominator)
    }
}

impl ToRichText for RationalNumber {
    fn to_rich_text(&self) -> RichText {
        RichText::Single(Span::raw(RationalNumberDisplay(self).to_string()))
    }
}

impl ToRichText for u8 {
    fn to_rich_text(&self) -> RichText {
        RichText::Single(Span::raw(self.to_string()))
    }
}

impl ToRichText for u64 {
    fn to_rich_text(&self) -> RichText {
        RichText::Single(Span::raw(self.to_string()))
    }
}

impl ToRichText for Bytes {
    fn to_rich_text(&self) -> RichText {
        RichText::Single(Span::raw(self.to_string()))
    }
}

impl<const BYTES: usize> ToRichText for Hash<BYTES> {
    fn to_rich_text(&self) -> RichText {
        RichText::Single(Span::raw(self.to_string()))
    }
}

impl ToRichText for String {
    fn to_rich_text(&self) -> RichText {
        RichText::Single(Span::raw(self.to_owned()))
    }
}

impl<T> ToRichText for (T, T)
where
    T: fmt::Display,
{
    fn to_rich_text(&self) -> RichText {
        RichText::Single(Span::raw(format!("({}, {})", self.0, self.1)))
    }
}

impl<T> ToRichText for Nullable<T>
where
    T: Clone + ToRichText,
{
    fn to_rich_text(&self) -> RichText {
        match &self {
            Nullable::Some(v) => v.to_rich_text(),
            Nullable::Null => RichText::Single(Span::raw("None")),
            Nullable::Undefined => RichText::Single(Span::raw("Undefined")),
        }
    }
}

impl<K> ToRichText for Set<K>
where
    K: ToRichText,
{
    fn to_rich_text(&self) -> RichText {
        self.iter()
            .flat_map(|k| k.to_rich_text().unwrap_lines())
            .collect()
    }
}

impl<K, V> ToRichText for KeyValuePairs<K, V>
where
    K: Clone + ToRichText,
    V: Clone + ToRichText,
{
    fn to_rich_text(&self) -> RichText {
        self.iter()
            .flat_map(|(k, v)| {
                let mut lines = Vec::new();
                lines.extend(labeled_default("Key", k));
                lines.extend(labeled_default("Value", v));
                lines
            })
            .collect()
    }
}

impl ToRichText for CertificatePointer {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled(
            "Slot".to_string(),
            RichText::Single(Span::raw(self.transaction.slot.to_string())),
            Style::default(),
        ));
        lines.extend(labeled(
            "Transaction Index".to_string(),
            RichText::Single(Span::raw(self.transaction.transaction_index.to_string())),
            Style::default(),
        ));
        lines.extend(labeled(
            "Certificate Index".to_string(),
            RichText::Single(Span::raw(self.certificate_index.to_string())),
            Style::default(),
        ));
        RichText::Lines(lines)
    }
}

use amaru_kernel::Address;
use amaru_kernel::{
    PostAlonzoTransactionOutput, PseudoTransactionOutput, TransactionInput, Value,
    alonzo::PlutusData, alonzo::TransactionOutput,
};
use pallas_codec::utils::CborWrap;
use pallas_primitives::babbage::PseudoDatumOption;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

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

impl ToRichText
    for (
        TransactionInput,
        PseudoTransactionOutput<PostAlonzoTransactionOutput>,
    )
{
    fn into_rich_text(self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled(
            "UTXO".to_string(),
            RichText::Single(Span::raw(self.0.transaction_id.to_string())),
            Style::default(),
        ));
        lines.extend(self.1.into_rich_text().unwrap_lines());
        RichText::Lines(lines)
    }
}

impl ToRichText for PseudoTransactionOutput<PostAlonzoTransactionOutput> {
    fn into_rich_text(self) -> RichText {
        match self {
            PseudoTransactionOutput::Legacy(inner) => inner.into_rich_text(),
            PseudoTransactionOutput::PostAlonzo(inner) => inner.into_rich_text(),
        }
    }
}

impl ToRichText for PostAlonzoTransactionOutput {
    fn into_rich_text(self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled(
            "Type".to_string(),
            RichText::Single(Span::raw("Post-Alonzo")),
            Style::new().fg(Color::Cyan),
        ));
        let bech32 = Address::from_bytes(&self.address)
            .ok()
            .and_then(|a| a.to_bech32().ok())
            .unwrap_or_else(|| self.address.to_string());
        lines.extend(labeled(
            "Address".to_string(),
            RichText::Single(Span::raw(bech32)),
            Style::default(),
        ));
        lines.extend(labeled(
            "Value".to_string(),
            GenericValueRichText(self.value).into_rich_text(),
            Style::default(),
        ));

        lines.extend(labeled(
            "Datum".to_string(),
            self.datum_option
                .map(|d| d.into_rich_text())
                .unwrap_or_else(|| RichText::Single(Span::raw("None"))),
            Style::default(),
        ));

        lines.extend(labeled(
            "Script".to_string(),
            self.script_ref
                .map(|s| CborWrapRichText(s).into_rich_text())
                .unwrap_or_else(|| RichText::Single(Span::raw("None"))),
            Style::default(),
        ));

        RichText::Lines(lines)
    }
}

impl ToRichText for TransactionOutput {
    fn into_rich_text(self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled(
            "Output Type".to_string(),
            RichText::Single(Span::raw("Legacy")),
            Style::new().fg(Color::Yellow),
        ));
        let bech32 = Address::from_bytes(&self.address)
            .ok()
            .and_then(|a| a.to_bech32().ok())
            .unwrap_or_else(|| self.address.to_string());
        lines.extend(labeled(
            "Address".to_string(),
            RichText::Single(Span::raw(bech32)),
            Style::default(),
        ));
        lines.extend(labeled(
            "DatumHash".to_string(),
            RichText::Single(Span::raw(
                self.datum_hash
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "None".into()),
            )),
            Style::default(),
        ));
        lines.extend(labeled(
            "Value".to_string(),
            AlonzoValueRichText(self.amount).into_rich_text(),
            Style::default(),
        ));
        RichText::Lines(lines)
    }
}

pub struct GenericValueRichText(pub Value);

impl ToRichText for GenericValueRichText {
    fn into_rich_text(self) -> RichText {
        match self.0 {
            Value::Coin(c) => RichText::Single(Span::raw(format!("{} lovelace", c))),
            Value::Multiasset(coin, assets) => {
                let mut lines = vec![Line::from(vec![Span::raw(format!("{} lovelace +", coin))])];
                for (pid, aset) in assets.iter() {
                    let pid_str = pid.to_string();
                    for (aname, amount) in aset.iter() {
                        let name_str = aname.to_string();
                        lines.push(Line::from(vec![Span::raw(format!(
                            "{} {}@{}",
                            u64::from(*amount),
                            name_str,
                            pid_str
                        ))]));
                    }
                }
                RichText::Lines(lines)
            }
        }
    }
}

pub struct AlonzoValueRichText(pub amaru_kernel::alonzo::Value);

impl ToRichText for AlonzoValueRichText {
    fn into_rich_text(self) -> RichText {
        match self.0 {
            amaru_kernel::alonzo::Value::Coin(c) => {
                RichText::Single(Span::raw(format!("{} lovelace", c)))
            }
            amaru_kernel::alonzo::Value::Multiasset(coin, assets) => {
                let mut lines = vec![Line::from(vec![Span::raw(format!("{} lovelace +", coin))])];
                for (pid, aset) in assets.iter() {
                    let pid_str = pid.to_string();
                    for (aname, amount) in aset.iter() {
                        let name_str = aname.to_string();
                        lines.push(Line::from(vec![Span::raw(format!(
                            "{} {}@{}",
                            amount, name_str, pid_str
                        ))]));
                    }
                }
                RichText::Lines(lines)
            }
        }
    }
}

impl ToRichText for PseudoDatumOption<PlutusData> {
    fn into_rich_text(self) -> RichText {
        match self {
            PseudoDatumOption::Hash(h) => RichText::Single(Span::raw(format!("DatumHash({})", h))),
            PseudoDatumOption::Data(cbor) => CborWrapRichText(cbor).into_rich_text(),
        }
    }
}

pub struct CborWrapRichText<T>(pub CborWrap<T>);

impl<'a, T: minicbor::Encode<()>> ToRichText for CborWrapRichText<T> {
    fn into_rich_text(self) -> RichText {
        match minicbor::to_vec(&self.0) {
            Ok(inner_bytes) => match cbor_diag::parse_bytes(&inner_bytes) {
                Ok(diag) => RichText::Single(Span::styled(
                    format!("Datum({})", diag.to_diag()),
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::ITALIC),
                )),
                Err(e) => RichText::Single(Span::styled(
                    format!("CBOR parse error: {}", e),
                    Style::default().fg(Color::Red),
                )),
            },
            Err(e) => RichText::Single(Span::styled(
                format!("CBOR encode error: {}", e),
                Style::default().fg(Color::Red),
            )),
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

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

pub enum RichText<'a> {
    Lines(Vec<Line<'a>>),
    Single(Span<'a>),
}

pub trait ToRichText<'a> {
    fn into_rich_text(self) -> RichText<'a>;
}

impl<'a> RichText<'a> {
    pub fn unwrap_lines(self) -> Vec<Line<'a>> {
        match self {
            RichText::Lines(v) => v,
            RichText::Single(span) => vec![Line::from(vec![span])],
        }
    }
}

impl<'a> ToRichText<'a>
    for (
        TransactionInput,
        PseudoTransactionOutput<PostAlonzoTransactionOutput>,
    )
{
    fn into_rich_text(self) -> RichText<'a> {
        let mut lines = Vec::new();
        lines.extend(labeled(
            "UTXO",
            RichText::Single(Span::raw(self.0.transaction_id.to_string())),
            Style::default(),
        ));
        lines.extend(self.1.into_rich_text().unwrap_lines());
        RichText::Lines(lines)
    }
}

impl<'a> ToRichText<'a> for PseudoTransactionOutput<PostAlonzoTransactionOutput> {
    fn into_rich_text(self) -> RichText<'a> {
        match self {
            PseudoTransactionOutput::Legacy(inner) => inner.into_rich_text(),
            PseudoTransactionOutput::PostAlonzo(inner) => inner.into_rich_text(),
        }
    }
}

impl<'a> ToRichText<'a> for PostAlonzoTransactionOutput {
    fn into_rich_text(self) -> RichText<'a> {
        let mut lines = Vec::new();
        lines.extend(labeled(
            "Type",
            RichText::Single(Span::raw("Post-Alonzo")),
            Style::new().fg(Color::Cyan),
        ));
        let bech32 = Address::from_bytes(&self.address)
            .ok()
            .and_then(|a| a.to_bech32().ok())
            .unwrap_or_else(|| self.address.to_string());
        lines.extend(labeled(
            "Address",
            RichText::Single(Span::raw(bech32)),
            Style::default(),
        ));
        lines.extend(labeled(
            "Value",
            GenericValueRichText(self.value).into_rich_text(),
            Style::default(),
        ));

        lines.extend(labeled(
            "Datum",
            self.datum_option
                .map(|d| d.into_rich_text())
                .unwrap_or_else(|| RichText::Single(Span::raw("None"))),
            Style::default(),
        ));

        lines.extend(labeled(
            "Script",
            self.script_ref
                .map(|s| CborWrapRichText(s).into_rich_text())
                .unwrap_or_else(|| RichText::Single(Span::raw("None"))),
            Style::default(),
        ));

        RichText::Lines(lines)
    }
}

impl<'a> ToRichText<'a> for TransactionOutput {
    fn into_rich_text(self) -> RichText<'a> {
        let mut lines = Vec::new();
        lines.extend(labeled(
            "Output Type",
            RichText::Single(Span::raw("Legacy")),
            Style::new().fg(Color::Yellow),
        ));
        let bech32 = Address::from_bytes(&self.address)
            .ok()
            .and_then(|a| a.to_bech32().ok())
            .unwrap_or_else(|| self.address.to_string());
        lines.extend(labeled(
            "Address",
            RichText::Single(Span::raw(bech32)),
            Style::default(),
        ));
        lines.extend(labeled(
            "DatumHash",
            RichText::Single(Span::raw(
                self.datum_hash
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "None".into()),
            )),
            Style::default(),
        ));
        lines.extend(labeled(
            "Value",
            AlonzoValueRichText(self.amount).into_rich_text(),
            Style::default(),
        ));
        RichText::Lines(lines)
    }
}

pub struct GenericValueRichText(pub Value);

impl<'a> ToRichText<'a> for GenericValueRichText {
    fn into_rich_text(self) -> RichText<'a> {
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

impl<'a> ToRichText<'a> for AlonzoValueRichText {
    fn into_rich_text(self) -> RichText<'a> {
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

impl<'a> ToRichText<'a> for PseudoDatumOption<PlutusData> {
    fn into_rich_text(self) -> RichText<'a> {
        match self {
            PseudoDatumOption::Hash(h) => RichText::Single(Span::raw(format!("DatumHash({})", h))),
            PseudoDatumOption::Data(cbor) => CborWrapRichText(cbor).into_rich_text(),
        }
    }
}

pub struct CborWrapRichText<T>(pub CborWrap<T>);

impl<'a, T: minicbor::Encode<()>> ToRichText<'a> for CborWrapRichText<T> {
    fn into_rich_text(self) -> RichText<'a> {
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

fn labeled<'a>(label: &'a str, value: RichText<'a>, value_style: Style) -> Vec<Line<'a>> {
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

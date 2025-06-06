use super::{RichText, ToRichText, labeled};
use amaru_kernel::Address;
use amaru_kernel::{
    PostAlonzoTransactionOutput, PseudoTransactionOutput, TransactionInput, TransactionOutput,
    Value, alonzo, alonzo::PlutusData,
};
use pallas_codec::utils::CborWrap;
use pallas_primitives::babbage::PseudoDatumOption;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use std::fmt;

pub struct TransactionInputDisplay<'a>(pub &'a TransactionInput);

impl<'a> fmt::Display for TransactionInputDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0.transaction_id, self.0.index)
    }
}

impl ToRichText for (TransactionInput, TransactionOutput) {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled(
            "UTXO".to_string(),
            RichText::Single(Span::raw(self.0.transaction_id.to_string())),
            Style::default(),
        ));
        lines.extend(self.1.to_rich_text().unwrap_lines());
        RichText::Lines(lines)
    }
}

impl ToRichText for TransactionOutput {
    fn to_rich_text(&self) -> RichText {
        match self {
            PseudoTransactionOutput::Legacy(inner) => inner.to_rich_text(),
            PseudoTransactionOutput::PostAlonzo(inner) => inner.to_rich_text(),
        }
    }
}

impl ToRichText for PostAlonzoTransactionOutput {
    fn to_rich_text(&self) -> RichText {
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
            GenericValueRichText(&self.value).to_rich_text(),
            Style::default(),
        ));

        lines.extend(labeled(
            "Datum".to_string(),
            self.datum_option
                .as_ref()
                .map(|d| d.to_rich_text())
                .unwrap_or_else(|| RichText::Single(Span::raw("None"))),
            Style::default(),
        ));

        lines.extend(labeled(
            "Script".to_string(),
            self.script_ref
                .as_ref()
                .map(|s| CborWrapRichText(s).to_rich_text())
                .unwrap_or_else(|| RichText::Single(Span::raw("None"))),
            Style::default(),
        ));

        RichText::Lines(lines)
    }
}

impl ToRichText for alonzo::TransactionOutput {
    fn to_rich_text(&self) -> RichText {
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
            AlonzoValueRichText(&self.amount).to_rich_text(),
            Style::default(),
        ));
        RichText::Lines(lines)
    }
}

pub struct GenericValueRichText<'a>(pub &'a Value);

impl<'a> ToRichText for GenericValueRichText<'a> {
    fn to_rich_text(&self) -> RichText {
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

pub struct AlonzoValueRichText<'a>(pub &'a amaru_kernel::alonzo::Value);

impl<'a> ToRichText for AlonzoValueRichText<'a> {
    fn to_rich_text(&self) -> RichText {
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
    fn to_rich_text(&self) -> RichText {
        match self {
            PseudoDatumOption::Hash(h) => RichText::Single(Span::raw(format!("DatumHash({})", h))),
            PseudoDatumOption::Data(cbor) => CborWrapRichText(cbor).to_rich_text(),
        }
    }
}

pub struct CborWrapRichText<'a, T>(pub &'a CborWrap<T>);

impl<'a, T: minicbor::Encode<()>> ToRichText for CborWrapRichText<'a, T> {
    fn to_rich_text(&self) -> RichText {
        match minicbor::to_vec(self.0) {
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

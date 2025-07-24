use crate::ui::{
    RichText, ToRichText, labeled, labeled_default, labeled_default_opt, labeled_default_single,
};
use amaru_kernel::{
    Address, MemoizedDatum, MemoizedScript, MemoizedTransactionOutput, PostAlonzoTransactionOutput,
    PseudoScript, TransactionInput, Value,
};
use pallas_codec::utils::CborWrap;
use pallas_primitives::{PlutusData, alonzo, babbage::PseudoDatumOption};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use std::fmt;

pub struct TransactionInputDisplay<'a>(pub &'a TransactionInput);

impl<'a> fmt::Display for TransactionInputDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0.transaction_id, self.0.index)
    }
}

impl ToRichText for (TransactionInput, MemoizedTransactionOutput) {
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

impl ToRichText for MemoizedTransactionOutput {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single("Address", &self.address));
        lines.extend(labeled_default("Value", &GenericValueRichText(&self.value)));
        lines.extend(labeled_default("Datum", &self.datum));
        lines.extend(labeled_default_opt("Script", self.script.as_ref()));
        RichText::Lines(lines)
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

pub struct AlonzoValueRichText<'a>(pub &'a alonzo::Value);

impl<'a> ToRichText for AlonzoValueRichText<'a> {
    fn to_rich_text(&self) -> RichText {
        match self.0 {
            alonzo::Value::Coin(c) => RichText::Single(Span::raw(format!("{} lovelace", c))),
            alonzo::Value::Multiasset(coin, assets) => {
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
            PseudoDatumOption::Hash(h) => {
                RichText::Single(Span::raw(format!("DatumHash({})", hex::encode(h))))
            }
            PseudoDatumOption::Data(cbor) => CborWrapRichText(cbor).to_rich_text(),
        }
    }
}

impl ToRichText for MemoizedDatum {
    fn to_rich_text(&self) -> RichText {
        match self {
            MemoizedDatum::None => RichText::Single(Span::from("None")),
            MemoizedDatum::Hash(d) => {
                RichText::Single(Span::raw(format!("MemoizedDatumHash({})", hex::encode(d))))
            }
            MemoizedDatum::Inline(d) => RichText::Single(Span::raw(format!(
                "MemoizedDatumInline({})",
                hex::encode(d.original_bytes())
            ))),
        }
    }
}

impl ToRichText for MemoizedScript {
    fn to_rich_text(&self) -> RichText {
        match self {
            PseudoScript::NativeScript(s) => RichText::Single(Span::raw(format!(
                "NativeScript({})",
                hex::encode(s.original_bytes())
            ))),
            PseudoScript::PlutusV1Script(s) => {
                RichText::Single(Span::raw(format!("PlutusV1Script({})", hex::encode(&*s.0))))
            }
            PseudoScript::PlutusV2Script(s) => {
                RichText::Single(Span::raw(format!("PlutusV2Script({})", hex::encode(&*s.0))))
            }
            PseudoScript::PlutusV3Script(s) => {
                RichText::Single(Span::raw(format!("PlutusV3Script({})", hex::encode(&*s.0))))
            }
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

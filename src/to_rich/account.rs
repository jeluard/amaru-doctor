use super::{RichText, ToRichText, labeled};
use crate::{to_list_item::AccountItem, to_rich::labeled_default_single};
use amaru_kernel::{DRep, StakeCredential};
use amaru_ledger::store::columns::accounts::Row;
use ratatui::{
    style::{Color, Style},
    text::Span,
};
use std::fmt;

pub struct StakeCredentialDisplay<'a>(pub &'a StakeCredential);

impl<'a> fmt::Display for StakeCredentialDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.0 {
            StakeCredential::ScriptHash(hash) => hash.to_string(),
            StakeCredential::AddrKeyhash(hash) => hash.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl ToRichText for StakeCredential {
    fn to_rich_text(&self) -> RichText {
        RichText::Single(Span::raw(StakeCredentialDisplay(self).to_string()))
    }
}

impl ToRichText for AccountItem {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single(
            "Account",
            StakeCredentialDisplay(&self.0),
        ));
        lines.extend(self.1.to_rich_text().unwrap_lines());
        RichText::Lines(lines)
    }
}

impl ToRichText for Row {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();

        lines.extend(labeled(
            "Delegatee".to_string(),
            self.delegatee
                .map(|pool_id| RichText::Single(Span::raw(pool_id.to_string())))
                .unwrap_or_else(|| RichText::Single(Span::raw("None"))),
            Style::default().fg(Color::Yellow),
        ));

        lines.extend(labeled(
            "Deposit".to_string(),
            RichText::Single(Span::raw(format!("{} lovelace", self.deposit))),
            Style::default(),
        ));

        lines.extend(labeled(
            "DRep".to_string(),
            self.drep
                .as_ref()
                .map(|(drep, ptr)| {
                    let mut drep_lines = drep.to_rich_text().unwrap_lines();
                    drep_lines.extend(ptr.to_rich_text().unwrap_lines());
                    RichText::Lines(drep_lines)
                })
                .unwrap_or_else(|| RichText::Single(Span::raw("None"))),
            Style::default(),
        ));

        lines.extend(labeled(
            "Rewards".to_string(),
            RichText::Single(Span::raw(format!("{} lovelace", self.rewards))),
            Style::default(),
        ));

        RichText::Lines(lines)
    }
}

impl ToRichText for DRep {
    fn to_rich_text(&self) -> RichText {
        let (label, value, color) = match self {
            DRep::Key(h) => ("DRep", format!("Key({})", h), Color::Green),
            DRep::Script(h) => ("DRep", format!("Script({})", h), Color::Magenta),
            DRep::Abstain => ("DRep", "Abstain".to_string(), Color::Yellow),
            DRep::NoConfidence => ("DRep", "NoConfidence".to_string(), Color::Red),
        };

        RichText::Lines(labeled(
            label.to_string(),
            RichText::Single(Span::raw(value)),
            Style::default().fg(color),
        ))
    }
}

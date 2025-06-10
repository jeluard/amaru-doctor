use crate::ui::to_rich::{RichText, ToRichText, labeled, labeled_default, labeled_default_single};
use amaru_kernel::{Epoch, Nullable, PoolId, PoolMetadata, PoolParams, Relay};
use amaru_ledger::store::columns::pools::Row;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use std::fmt;

pub struct PoolIdDisplay(pub PoolId);

impl fmt::Display for PoolIdDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToRichText for (PoolId, Row) {
    fn to_rich_text(&self) -> RichText {
        let (id, row) = self;
        let mut lines = Vec::new();

        lines.extend(labeled(
            "Pool ID".into(),
            RichText::Single(Span::raw(id.to_string())),
            Style::default(),
        ));
        lines.extend(labeled(
            "Current Params".into(),
            row.current_params.to_rich_text(),
            Style::default(),
        ));

        let future_lines = if row.future_params.is_empty() {
            RichText::Single(Span::raw("None"))
        } else {
            RichText::Lines(
                row.future_params
                    .iter()
                    .flat_map(|entry| entry.to_rich_text().unwrap_lines())
                    .collect(),
            )
        };

        lines.extend(labeled(
            "Future Params".into(),
            future_lines,
            Style::default(),
        ));

        RichText::Lines(lines)
    }
}

impl ToRichText for PoolParams {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();

        lines.extend(labeled(
            "VRF Keyhash".into(),
            RichText::Single(Span::raw(self.vrf.to_string())),
            Style::default(),
        ));
        lines.extend(labeled(
            "Pledge".into(),
            RichText::Single(Span::raw(format!("{} lovelace", self.pledge))),
            Style::default(),
        ));
        lines.extend(labeled(
            "Cost".into(),
            RichText::Single(Span::raw(format!("{} lovelace", self.cost))),
            Style::default(),
        ));
        lines.extend(labeled(
            "Margin".into(),
            RichText::Single(Span::raw(format!(
                "{}/{}",
                self.margin.numerator, self.margin.denominator
            ))),
            Style::default(),
        ));
        lines.extend(labeled(
            "Reward Account".into(),
            RichText::Single(Span::raw(self.reward_account.to_string())),
            Style::default(),
        ));

        lines.extend(labeled(
            "Owners".into(),
            if self.owners.is_empty() {
                RichText::Single(Span::raw("None"))
            } else {
                RichText::Lines(
                    self.owners
                        .iter()
                        .map(|o| Line::from(vec![Span::raw(o.to_string())]))
                        .collect(),
                )
            },
            Style::default(),
        ));

        lines.extend(labeled(
            "Relays".into(),
            if self.relays.is_empty() {
                RichText::Single(Span::raw("None"))
            } else {
                RichText::Lines(
                    self.relays
                        .iter()
                        .map(|r| Line::from(vec![Span::raw(format!("{:?}", r))]))
                        .collect(),
                )
            },
            Style::default(),
        ));

        lines.extend(labeled_default("Metadata", &self.metadata));

        RichText::Lines(lines)
    }
}

impl ToRichText for PoolMetadata {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single("Url", &self.url));
        lines.extend(labeled_default_single("Hash", self.hash));
        RichText::Lines(lines)
    }
}

impl ToRichText for Relay {
    fn to_rich_text(&self) -> RichText {
        match self {
            Relay::SingleHostAddr(port, ipv4, ipv6) => {
                let mut lines = Vec::new();

                lines.extend(labeled(
                    "Relay Type".to_string(),
                    RichText::Single(Span::raw("SingleHostAddr")),
                    Style::default(),
                ));

                if let Nullable::Some(p) = port {
                    lines.extend(labeled(
                        "Port".to_string(),
                        RichText::Single(Span::raw(p.to_string())),
                        Style::default(),
                    ));
                }

                if let Nullable::Some(ip) = ipv4 {
                    let formatted = ip.iter().map(u8::to_string).collect::<Vec<_>>().join(".");
                    lines.extend(labeled(
                        "IPv4".to_string(),
                        RichText::Single(Span::raw(formatted)),
                        Style::default(),
                    ));
                }

                if let Nullable::Some(ip) = ipv6 {
                    lines.extend(labeled(
                        "IPv6".to_string(),
                        RichText::Single(Span::raw(format!("{:x?}", ip))),
                        Style::default(),
                    ));
                }

                RichText::Lines(lines)
            }

            Relay::SingleHostName(port, hostname) => {
                let mut lines = Vec::new();

                lines.extend(labeled(
                    "Relay Type".to_string(),
                    RichText::Single(Span::raw("SingleHostName")),
                    Style::default(),
                ));

                lines.extend(labeled(
                    "Hostname".to_string(),
                    RichText::Single(Span::raw(hostname.clone())),
                    Style::default(),
                ));

                if let Nullable::Some(p) = port {
                    lines.extend(labeled(
                        "Port".to_string(),
                        RichText::Single(Span::raw(p.to_string())),
                        Style::default(),
                    ));
                }

                RichText::Lines(lines)
            }

            Relay::MultiHostName(hostname) => {
                let mut lines = Vec::new();

                lines.extend(labeled(
                    "Relay Type".to_string(),
                    RichText::Single(Span::raw("MultiHostName")),
                    Style::default(),
                ));

                lines.extend(labeled(
                    "Hostname".to_string(),
                    RichText::Single(Span::raw(hostname.clone())),
                    Style::default(),
                ));

                RichText::Lines(lines)
            }
        }
    }
}

impl ToRichText for (Option<PoolParams>, Epoch) {
    fn to_rich_text(&self) -> RichText {
        match self {
            (Some(p), epoch) => {
                let mut lines = vec![Line::from(vec![Span::raw(format!(
                    "Epoch {}: Update",
                    epoch
                ))])];
                lines.extend(p.to_rich_text().unwrap_lines());
                RichText::Lines(lines)
            }
            (None, epoch) => RichText::Single(Span::raw(format!("Epoch {}: Retirement", epoch))),
        }
    }
}

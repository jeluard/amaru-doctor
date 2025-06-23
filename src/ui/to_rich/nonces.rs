use crate::ui::{RichText, ToRichText, labeled_default, labeled_default_single};
use amaru_consensus::Nonces;

impl ToRichText for Nonces {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default("Active", &self.active));
        lines.extend(labeled_default("Evolving", &self.evolving));
        lines.extend(labeled_default("Candidate", &self.candidate));
        lines.extend(labeled_default("Tail", &self.tail));
        lines.extend(labeled_default_single("Epoch", self.epoch));
        RichText::Lines(lines)
    }
}

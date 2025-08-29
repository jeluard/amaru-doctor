use crate::ui::{
    RichText, ToRichText, labeled_default, labeled_default_opt, labeled_default_single,
    to_list_item::DRepItem, to_rich::account::StakeCredentialDisplay,
};
use amaru_kernel::Anchor;
use amaru_ledger::store::columns::dreps;

impl ToRichText for DRepItem {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single(
            "DRep",
            StakeCredentialDisplay(&self.0),
        ));
        lines.extend(self.1.to_rich_text().unwrap_lines());
        RichText::Lines(lines)
    }
}

impl ToRichText for dreps::Row {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single("Deposit", self.deposit));
        lines.extend(labeled_default_opt("Anchor", self.anchor.as_ref()));
        lines.extend(labeled_default("Registered At", &self.registered_at));
        lines.extend(labeled_default_single("Valid Until", self.valid_until));
        lines.extend(labeled_default_opt(
            "Previous Deregistration",
            self.previous_deregistration.as_ref(),
        ));
        RichText::Lines(lines)
    }
}

impl ToRichText for Anchor {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single("Url", &self.url));
        lines.extend(labeled_default_single("Content Hash", self.content_hash));
        RichText::Lines(lines)
    }
}

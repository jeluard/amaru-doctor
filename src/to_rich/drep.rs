use crate::{
    components::list_and_details::drep::DRepItem,
    to_rich::{
        RichText, ToRichText, account::StakeCredentialDisplay, labeled_default,
        labeled_default_opt, labeled_default_opt_single, labeled_default_single,
    },
};
use amaru_kernel::Anchor;
use amaru_ledger::store::columns::dreps;

impl ToRichText for DRepItem {
    fn to_rich_text(&self) -> super::RichText {
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
        lines.extend(labeled_default_opt_single(
            "Last Interaction",
            self.last_interaction,
        ));
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

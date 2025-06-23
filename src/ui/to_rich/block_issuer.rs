use crate::ui::{
    RichText, ToRichText, labeled_default_single, to_list_item::BlockIssuerItem,
    to_rich::pool::PoolIdDisplay,
};

impl ToRichText for BlockIssuerItem {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single("Block Issuer", self.0.to_string()));
        lines.extend(labeled_default_single(
            "Slot Leader",
            PoolIdDisplay(self.1.slot_leader).to_string(),
        ));
        RichText::Lines(lines)
    }
}

use crate::ui::{
    RichText, ToRichText, labeled_default, labeled_default_opt, labeled_default_single,
};
use amaru_kernel::{Header, HeaderBody};
use pallas_primitives::{VrfCert, conway::OperationalCert};

impl ToRichText for Header {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default("Header body", &self.header_body));
        lines.extend(labeled_default("Body signature", &self.body_signature));
        RichText::Lines(lines)
    }
}

impl ToRichText for HeaderBody {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default("Block number", &self.block_number));
        lines.extend(labeled_default("Slot", &self.slot));
        lines.extend(labeled_default_opt("Prev hash", self.prev_hash.as_ref()));
        lines.extend(labeled_default("Issuer vkey", &self.issuer_vkey));
        lines.extend(labeled_default("VRF vkey", &self.vrf_vkey));
        lines.extend(labeled_default("VRF result", &self.vrf_result));
        lines.extend(labeled_default("Block body size", &self.block_body_size));
        lines.extend(labeled_default("Block body hash", &self.block_body_hash));
        lines.extend(labeled_default("Operational cert", &self.operational_cert));
        lines.extend(labeled_default("Protocol version", &self.protocol_version));
        RichText::Lines(lines)
    }
}

impl ToRichText for VrfCert {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default_single("0", &self.0));
        lines.extend(labeled_default_single("1", &self.1));
        RichText::Lines(lines)
    }
}

impl ToRichText for OperationalCert {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        lines.extend(labeled_default(
            "Operational cert hot vkey",
            &self.operational_cert_hot_vkey,
        ));
        lines.extend(labeled_default(
            "Operational cert sequence number",
            &self.operational_cert_sequence_number,
        ));
        lines.extend(labeled_default(
            "Operational cert kes period",
            &self.operational_cert_kes_period,
        ));
        lines.extend(labeled_default(
            "Operational cert sigma",
            &self.operational_cert_sigma,
        ));
        RichText::Lines(lines)
    }
}

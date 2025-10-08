use crate::{
    prometheus::model::NodeMetrics,
    ui::{RichText, ToRichText, labeled_default_single},
};

impl ToRichText for NodeMetrics {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();

        lines.extend(labeled_default_single("Block Number", self.block_number.0));
        lines.extend(labeled_default_single("Density", self.density.0));
        lines.extend(labeled_default_single("Epoch", self.epoch.0));
        lines.extend(labeled_default_single(
            "Slot in Epoch",
            self.slot_in_epoch.0,
        ));
        lines.extend(labeled_default_single("Slot Num", self.slot_num.0));
        lines.extend(labeled_default_single(
            "Transactions Processed",
            self.transactions_processed.0,
        ));

        lines.extend(labeled_default_single(
            "CPU Percent Util",
            self.cpu_percent_util.0,
        ));
        lines.extend(labeled_default_single(
            "Disk Live Read Bytes",
            self.disk_live_read_bytes.0,
        ));
        lines.extend(labeled_default_single(
            "Disk Live Write Bytes",
            self.disk_live_write_bytes.0,
        ));
        lines.extend(labeled_default_single(
            "Disk Total Read Bytes",
            self.disk_total_read_bytes.0,
        ));
        lines.extend(labeled_default_single(
            "Disk Total Write Bytes",
            self.disk_total_write_bytes.0,
        ));
        lines.extend(labeled_default_single(
            "Mem Available Virtual Bytes",
            self.mem_available_virtual_bytes.0,
        ));
        lines.extend(labeled_default_single(
            "Mem Live Resident Bytes",
            self.mem_live_resident_bytes.0,
        ));
        lines.extend(labeled_default_single("Open Files", self.open_files.0));
        lines.extend(labeled_default_single(
            "Runtime Seconds",
            self.runtime_seconds.0,
        ));

        RichText::Lines(lines)
    }
}

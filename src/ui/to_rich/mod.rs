use crate::ui::RichText;
use ratatui::text::Line;

pub mod account;
pub mod block_issuer;
pub mod drep;
pub mod header;
pub mod metrics;
pub mod nonces;
pub mod pool;
pub mod proposal;
pub mod span;
pub mod utxo;

impl From<Vec<Line<'static>>> for RichText {
    fn from(lines: Vec<Line<'static>>) -> Self {
        RichText::Lines(lines)
    }
}

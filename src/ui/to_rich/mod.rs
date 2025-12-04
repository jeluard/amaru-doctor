use crate::ui::{RichText, ToRichText};
use ratatui::text::{Line, Span};

pub mod account;
pub mod block_issuer;
pub mod drep;
pub mod header;
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

impl ToRichText for Vec<u8> {
    fn to_rich_text(&self) -> RichText {
        RichText::Single(Span::from(hex::encode(self)))
    }
}

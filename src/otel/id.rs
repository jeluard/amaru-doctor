use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Id<const N: usize>(pub [u8; N]);

pub type SpanId = Id<8>;
pub type TraceId = Id<16>;
pub type RootId = SpanId;

impl<const N: usize> fmt::Display for Id<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl<const N: usize> fmt::Debug for Id<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for Id<N> {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let actual_len = value.len();

        value.try_into().map(Id).map_err(|_| {
            anyhow::anyhow!(
                "Invalid length for id: expected {}, actual {}",
                N,
                actual_len
            )
        })
    }
}

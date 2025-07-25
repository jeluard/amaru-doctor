use crate::update::search::SearchState;
use amaru_consensus::Nonces;
use amaru_kernel::{Hash, Header, RawBlock};

pub struct ChainViewState {
    pub chain_search: SearchState<Hash<32>, Option<(Header, RawBlock, Nonces)>>,
}

impl ChainViewState {
    pub fn new() -> Self {
        Self {
            chain_search: SearchState::default(),
        }
    }
}

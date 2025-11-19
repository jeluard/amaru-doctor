use crate::model::search::SearchState;
use amaru_consensus::{BlockHeader, Nonces};
use amaru_kernel::{Hash, RawBlock};

#[derive(Default)]
pub struct ChainViewState {
    pub chain_search: SearchState<Hash<32>, Option<(BlockHeader, RawBlock, Nonces)>>,
}

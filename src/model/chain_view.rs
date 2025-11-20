use crate::model::search::SearchCache;
use amaru_consensus::{BlockHeader, Nonces};
use amaru_kernel::{Hash, RawBlock};

#[derive(Default)]
pub struct ChainViewState {
    pub chain_search: SearchCache<Hash<32>, Option<(BlockHeader, RawBlock, Nonces)>>,
}

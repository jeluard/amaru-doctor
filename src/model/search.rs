use std::{collections::HashMap, hash::Hash};

/// The state needed to manage cached search results.
/// Q: The parsed Query type (e.g. Address, Hash)
/// R: The Result type (e.g. AsyncListModel, (Header, Block, Nonces))
#[derive(Debug)]
pub struct SearchCache<Q, R> {
    /// The parsed query object of the currently active result.
    pub parsed: Option<Q>,
    /// A map of previous query results, cached to avoid re-fetching.
    pub results: HashMap<Q, R>,
}

impl<Q, R> SearchCache<Q, R>
where
    Q: Clone + Eq + Hash,
{
    /// Stores a result for a specific query and sets it as the active view.
    pub fn cache_result(&mut self, query: Q, result: R) {
        self.parsed = Some(query.clone());
        self.results.insert(query, result);
    }

    pub fn get_current_res(&self) -> Option<&R> {
        self.parsed.as_ref().and_then(|q| self.results.get(q))
    }

    pub fn get_current_res_mut(&mut self) -> Option<&mut R> {
        self.parsed.as_ref().and_then(|q| self.results.get_mut(q))
    }
}

impl<Q, R> Default for SearchCache<Q, R> {
    fn default() -> Self {
        Self {
            parsed: Default::default(),
            results: Default::default(),
        }
    }
}

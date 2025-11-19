use std::collections::HashMap;

#[derive(Debug)]
pub struct SearchState<Q, R> {
    pub builder: String,
    pub parsed: Option<Q>,
    pub results: HashMap<Q, R>,
}

impl<Q, R> SearchState<Q, R>
where
    Q: Clone + Eq + std::hash::Hash,
{
    pub fn push_char(&mut self, c: char) {
        self.builder.push(c);
    }
    pub fn pop_char(&mut self) {
        self.builder.pop();
    }
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

impl<Q, R> Default for SearchState<Q, R> {
    fn default() -> Self {
        Self {
            builder: Default::default(),
            parsed: Default::default(),
            results: Default::default(),
        }
    }
}

use crate::{
    app_state::AppState,
    states::{Action, ComponentId},
    update::{
        Update,
        search::handler::{ChainSearch, LedgerUtxosByAddr},
    },
};
use crossterm::event::KeyCode;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};
use strum::{EnumIter, IntoEnumIterator};
use tracing::trace;

pub mod handler;

/// The state needed to manage searching amaru db's
pub struct SearchState<Q, R> {
    /// The query builder
    pub builder: String,
    /// What's parsed after the user hits enter
    pub parsed: Option<Q>,
    /// A map of query results, cached should the user search for the same query
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
        self.results.entry(query).or_insert_with(|| result);
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

trait SearchHandler {
    type Query: Clone + Eq + FromStr + std::hash::Hash + Debug;
    type Result;

    fn debug_name(&self) -> &'static str;

    /// Which ComponentId this search handler is bound to
    fn id(&self) -> ComponentId;

    /// Whether this handler should be active given current AppState
    fn is_active(&self, app: &AppState) -> bool;

    /// Get access to the associated SearchState from AppState
    fn state<'a>(&self, app: &'a AppState) -> &'a SearchState<Self::Query, Self::Result>;

    /// Get access to the associated SearchState from AppState
    fn state_mut<'a>(
        &self,
        app: &'a mut AppState,
    ) -> &'a mut SearchState<Self::Query, Self::Result>;

    /// How to compute results given a query
    fn compute(&self, app: &AppState, query: &Self::Query) -> Self::Result;
}

/// The supported searches
#[derive(EnumIter)]
enum Search {
    LedgerUtxosByAddr(LedgerUtxosByAddr),
    ChainSearch(ChainSearch),
}

impl Update for Search {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        match self {
            Search::LedgerUtxosByAddr(h) => update_search(h, action, s),
            Search::ChainSearch(h) => update_search(h, action, s),
        }
    }
}

fn update_search<H>(handler: &H, a: &Action, s: &mut AppState) -> Vec<Action>
where
    H: SearchHandler,
    <H::Query as FromStr>::Err: Display,
{
    if !s.layout_model.is_component_focused(handler.id()) || !handler.is_active(s) {
        return Vec::new();
    }
    trace!(
        "{} is focused and active, handling search",
        handler.debug_name()
    );

    match a {
        Action::Key(KeyCode::Char(c)) => {
            handler.state_mut(s).push_char(*c);
        }
        Action::Key(KeyCode::Backspace) => {
            handler.state_mut(s).pop_char();
        }
        Action::Key(KeyCode::Enter) => {
            let input = handler.state(s).builder.clone();
            match H::Query::from_str(&input) {
                Ok(query) => {
                    let result = handler.compute(s, &query);
                    handler.state_mut(s).cache_result(query, result);
                }
                Err(e) => return vec![Action::Error(e.to_string())],
            }
        }
        _ => {}
    }

    Vec::new()
}

pub struct SearchUpdate;
impl Update for SearchUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action> {
        let Action::Key(_) = action else {
            return Vec::new();
        };
        trace!("Will handle key event in Search");
        let mut actions = Vec::new();
        for handler in Search::iter() {
            actions.extend(handler.update(action, app_state));
        }
        actions
    }
}

use crate::{
    action::Action,
    components::{Component, r#static::search_types::SearchType},
    focus::{FocusState, FocusableComponent},
    shared::SharedGetter,
};
use amaru_kernel::{Address, HasAddress, TransactionInput};
use amaru_ledger::store::ReadOnlyStore;
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use std::{collections::HashMap, str::FromStr, sync::Arc};

pub struct SearchResultComponent<R>
where
    R: ReadOnlyStore,
{
    db: Arc<R>,
    search_type: SharedGetter<SearchType>,
    query: SharedGetter<String>,
    index: HashMap<Address, Vec<TransactionInput>>,
    results: Vec<TransactionInput>,
    focus: FocusState,
}

impl<R> SearchResultComponent<R>
where
    R: ReadOnlyStore,
{
    pub fn new(
        db: Arc<R>,
        search_type: SharedGetter<SearchType>,
        query: SharedGetter<String>,
    ) -> Self {
        Self {
            db,
            search_type,
            query,
            index: HashMap::new(),
            results: Vec::new(),
            focus: FocusState::default(),
        }
    }

    fn ensure_index(&mut self) {
        if self.index.is_empty() {
            for (input, output) in self.db.iter_utxos().unwrap() {
                self.index
                    .entry(output.address().unwrap())
                    .or_default()
                    .push(input);
            }
        }
    }

    fn update_results(&mut self) {
        let search_type = self.search_type.borrow().get().as_deref().copied();
        let query = self.query.borrow().get().as_deref().map(|s| s.to_owned());

        match (search_type, query) {
            (Some(SearchType::UtxoByAddress), Some(address_str)) => {
                self.ensure_index();
                if let Ok(address) = Address::from_str(&address_str) {
                    self.results = self.index.get(&address).cloned().unwrap_or_default();
                } else {
                    self.results.clear();
                }
            }
            _ => {
                self.results.clear();
            }
        }
    }
}

impl<R> FocusableComponent for SearchResultComponent<R>
where
    R: ReadOnlyStore,
{
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl<R> Component for SearchResultComponent<R>
where
    R: ReadOnlyStore,
{
    fn debug_name(&self) -> String {
        "SearchResultComponent".into()
    }

    fn handle_key_event(&mut self, _key: crossterm::event::KeyEvent) -> Result<Vec<Action>> {
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.update_results();

        let items = self
            .results
            .iter()
            .enumerate()
            .map(|(i, tx)| ListItem::new(format!("{i}: {}", tx.transaction_id)))
            .collect::<Vec<_>>();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Search Results"),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(list, area);
        Ok(())
    }
}

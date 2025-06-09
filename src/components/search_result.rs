use crate::{
    action::Action,
    components::{Component, r#static::search_types::SearchType},
    focus::{FocusState, FocusableComponent},
    shared::{Getter, Shared},
    to_rich::utxo::TransactionInputDisplay,
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
    search_type: Shared<dyn Getter<SearchType>>,
    query: Shared<dyn Getter<Option<String>>>,
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
        search_type: Shared<dyn Getter<SearchType>>,
        query: Shared<dyn Getter<Option<String>>>,
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
        let search = *self.search_type.borrow().get();
        let query = self.query.borrow().get().clone();
        if let (SearchType::UtxoByAddress, Some(query)) = (search, query) {
            self.ensure_index();
            if let Ok(address) = Address::from_str(&query) {
                self.results = self.index.get(&address).cloned().unwrap_or_default();
            } else {
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
            .map(|txi| ListItem::new(TransactionInputDisplay(txi).to_string()))
            .collect::<Vec<_>>();

        let mut block = Block::default()
            .title("Search Results")
            .borders(Borders::ALL);
        if self.has_focus() {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let list = List::new(items)
            .highlight_symbol(">> ")
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .block(block);

        frame.render_widget(list, area);
        Ok(())
    }
}

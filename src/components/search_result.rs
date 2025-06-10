use crate::{
    states::Action,
    components::{Component, list::ListComponent, r#static::search_types::SearchType},
    focus::{FocusState, FocusableComponent},
    shared::{GetterOpt, SharedGetterOpt},
    store::owned_iter::OwnedUtxoIter,
    store::rocks_db_switch::RocksDBSwitch,
    ui::to_list_item::UtxoItem,
};
use amaru_kernel::{Address, HasAddress};
use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use std::{
    iter::{self},
    str::FromStr,
    sync::Arc,
};

pub struct SearchResultComponent {
    db: Arc<RocksDBSwitch>,
    search_type: SharedGetterOpt<SearchType>,
    query: SharedGetterOpt<String>,
    list: ListComponent<UtxoItem>,
    focus: FocusState,
}

impl SearchResultComponent {
    pub fn new(
        db: Arc<RocksDBSwitch>,
        search_type: SharedGetterOpt<SearchType>,
        query: SharedGetterOpt<String>,
    ) -> Self {
        Self {
            db,
            search_type,
            query,
            list: ListComponent::from_iter("Search Results".to_string(), Box::new(iter::empty())),
            focus: FocusState::default(),
        }
    }

    fn search_result_iter(&self) -> Box<dyn Iterator<Item = UtxoItem> + 'static> {
        let st_opt = self.search_type.borrow().get().cloned();
        let q_opt = self.query.borrow().get().cloned();

        if let (Some(SearchType::UtxosByAddress), Some(query)) = (st_opt, q_opt) {
            if let Ok(address) = Address::from_str(&query) {
                let filter = Box::new(move |(_, out): &UtxoItem| out.address().unwrap() == address);
                let iter = OwnedUtxoIter::new(self.db.clone()).filter(filter);
                return Box::new(iter);
            }
        }
        Box::new(iter::empty())
    }
}

impl FocusableComponent for SearchResultComponent {
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }

    fn has_focus(&self) -> bool {
        self.list.has_focus()
    }

    fn set_focus(&mut self, b: bool) {
        self.list.set_focus(b);
    }
}

impl GetterOpt<UtxoItem> for SearchResultComponent {
    fn get(&self) -> Option<&UtxoItem> {
        self.list.get()
    }
}

impl Component for SearchResultComponent {
    fn debug_name(&self) -> String {
        "SearchResultComponent".into()
    }

    fn update(&mut self, action: Action) -> Result<Vec<Action>> {
        if action == Action::SearchRequest {
            self.list =
                ListComponent::from_iter("Search Results".to_string(), self.search_result_iter());
        }
        Ok(vec![])
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if self.has_focus() {
            return self.list.handle_key_event(key);
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.list.draw(frame, area)
    }
}

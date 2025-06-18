use crate::{
    app_state::AppState,
    model::window::WindowState,
    states::{Action, SearchOption, WidgetSlot},
    store::owned_iter::OwnedUtxoIter,
    ui::to_list_item::UtxoItem,
    update::Update,
};
use amaru_kernel::{Address, HasAddress};
use crossterm::event::KeyCode;
use std::str::FromStr;

pub struct SearchQuery {}

impl Update for SearchQuery {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Option<Action> {
        let WidgetSlot::SearchBar = app_state.slot_focus.current() else {
            return None;
        };

        match action {
            Action::Key(KeyCode::Char(ch)) => app_state.search_query_bldr.push(*ch),
            Action::Key(KeyCode::Backspace) => {
                app_state.search_query_bldr.pop();
            }
            Action::Key(KeyCode::Enter) => match app_state.search_options.selected() {
                SearchOption::UtxosByAddress => {
                    let query = app_state.search_query_bldr.clone();
                    let Ok(addr) = Address::from_str(&query) else {
                        return Some(Action::Error(format!(
                            "Couldn't parse query into address, {}",
                            query
                        )));
                    };
                    app_state.search_query_addr = Some(addr);
                    return Some(Action::SearchUtxosByAddr);
                }
            },
            _ => {}
        }

        None
    }
}

pub struct SearchRequest {}

impl Update for SearchRequest {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Option<Action> {
        // The only currently supported search
        let Action::SearchUtxosByAddr = action else {
            return None;
        };

        let Some(ref addr) = app_state.search_query_addr else {
            return Some(Action::Error(
                "No search query address despite SearchUtxosByAddr action".to_owned(),
            ));
        };

        app_state
            .utxos_by_addr_search_res
            .entry(addr.clone())
            .or_insert_with(|| {
                let owned_addr = addr.clone();
                let iter = OwnedUtxoIter::new(app_state.ledger_db.clone())
                    .filter(move |(_, out): &UtxoItem| out.address().unwrap() == owned_addr);
                let mut window = WindowState::from_box(Box::new(iter));
                window.set_window_size(app_state.list_window_size);
                window
            });

        None
    }
}

use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    model::window::WindowState,
    states::{Action, StoreOption, WidgetSlot},
    store::owned_iter::OwnedUtxoIter,
    ui::to_list_item::UtxoItem,
    update::Update,
};
use amaru_consensus::consensus::store::ChainStore;
use amaru_kernel::{Address, HasAddress, Hash};
use color_eyre::Result;
use crossterm::event::KeyCode;
use std::{collections::HashMap, fmt::Debug, str::FromStr};
use tracing::trace;

pub trait KeyboardHandler: Sync {
    fn update(&self, action: &Action, s: &mut AppState) -> Option<Action>;
}
pub struct SearchHandler<Q, R> {
    /// Helpful for debugging
    pub debug_name: &'static str,
    /// WidgetSlot related to this handler
    pub slot: WidgetSlot,
    /// bool determining if this is the right handler
    pub is_placed: fn(&AppState) -> bool,
    /// &mut String where user-typed chars are stored (or removed on backspace)
    pub get_bldr_mut: fn(&mut AppState) -> &mut String,
    /// how to parse the builder text into Q
    pub parse: fn(&str) -> Result<Q>,
    /// &mut Option<Q> that stores the query obj, parsed after the user presses enter
    pub get_query_mut: fn(&mut AppState) -> &mut Option<Q>,
    /// makes results
    pub make_result: fn(&AppState, &Q) -> R,
    /// &mut HashMap<Q, R> that stores results for queries
    pub get_map_mut: fn(&mut AppState) -> &mut HashMap<Q, R>,
}

impl<Q, R> KeyboardHandler for SearchHandler<Q, R>
where
    Q: Clone + Debug + Eq + std::hash::Hash,
{
    fn update(&self, action: &Action, s: &mut AppState) -> Option<Action> {
        if !is_widget_focused(s, self.slot) {
            return None;
        }

        if !(self.is_placed)(s) {
            return None;
        }

        let bldr = (self.get_bldr_mut)(s);
        match action {
            Action::Key(KeyCode::Char(ch)) => {
                trace!("{}: Got character key", self.debug_name);
                bldr.push(*ch);
            }
            Action::Key(KeyCode::Backspace) => {
                bldr.pop();
            }
            Action::Key(KeyCode::Enter) => {
                trace!("{}: Got enter key", self.debug_name);
                let val = match (self.parse)(bldr) {
                    Ok(v) => {
                        trace!(
                            "{}: Parsed query builder to value, {:?}",
                            self.debug_name, v
                        );
                        v
                    }
                    Err(e) => {
                        trace!(
                            "{}: Error parsing query builder to value, {}",
                            self.debug_name, bldr
                        );
                        return Some(Action::Error(e.to_string()));
                    }
                };
                trace!("{}: Set parsed value", self.debug_name);
                *(self.get_query_mut)(s) = Some(val.clone());
                let res = (self.make_result)(s, &val);
                trace!("{}: Made res", self.debug_name);
                (self.get_map_mut)(s)
                    .entry(val.clone())
                    .or_insert_with(|| res);
                trace!("{}: Set res in res map", self.debug_name);
            }
            _ => {}
        }
        None
    }
}

static SEARCH_HANDLERS: &[&dyn KeyboardHandler] = &[
    &SearchHandler {
        debug_name: "LedgerSearch",
        slot: WidgetSlot::SearchBar,
        is_placed: |s| *s.store_option.current() == StoreOption::Ledger,
        get_bldr_mut: |s| &mut s.ledger_search_query_bldr,
        get_query_mut: |s| &mut s.ledger_search_query_addr,
        parse: |q| Address::from_str(q).map_err(Into::into),
        get_map_mut: |s| &mut s.utxos_by_addr_search_res,
        make_result: |s, q| {
            let owned_addr = q.clone();
            let iter = OwnedUtxoIter::new(s.ledger_db.clone())
                .filter(move |(_, out): &UtxoItem| out.address().unwrap() == owned_addr);
            let mut window = WindowState::from_box(Box::new(iter));
            window.set_window_size(s.list_window_size);
            window
        },
    },
    &SearchHandler {
        debug_name: "ChainSearch",
        slot: WidgetSlot::SearchBar,
        is_placed: |s| *s.store_option.current() == StoreOption::Chain,
        get_bldr_mut: |s| &mut s.chain_search_query_bldr,
        get_query_mut: |s| &mut s.chain_search_query_hash,
        parse: |q| Hash::<32>::from_str(q).map_err(Into::into),
        get_map_mut: |s| &mut s.headers_by_hash_search_res,
        make_result: |s, q| {
            let res = s.chain_db.load_header(q);
            trace!("Got load header res {:?}", res);
            res
        },
    },
];

pub struct SearchQuery {}

impl Update for SearchQuery {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Option<Action> {
        let Action::Key(_) = action else {
            return None;
        };
        trace!("Got key event");
        for handler in SEARCH_HANDLERS {
            if let Some(act) = handler.update(action, app_state) {
                return Some(act);
            }
        }
        None
    }
}

// impl Update for SearchRequest {
//     fn update(&self, action: &Action, app_state: &mut AppState) -> Option<Action> {
//         // The only currently supported search
//         let Action::SearchUtxosByAddr = action else {
//             return None;
//         };

//         let Some(ref addr) = app_state.ledger_search_query_addr else {
//             return Some(Action::Error(
//                 "No search query address despite SearchUtxosByAddr action".to_owned(),
//             ));
//         };

//         app_state
//             .utxos_by_addr_search_res
//             .entry(addr.clone())
//             .or_insert_with(|| {
//                 let owned_addr = addr.clone();
//                 let iter = OwnedUtxoIter::new(app_state.ledger_db.clone())
//                     .filter(move |(_, out): &UtxoItem| out.address().unwrap() == owned_addr);
//                 let mut window = WindowState::from_box(Box::new(iter));
//                 window.set_window_size(app_state.list_window_size);
//                 window
//             });

//         None
//     }
// }

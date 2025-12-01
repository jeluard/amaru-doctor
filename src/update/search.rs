use crate::{
    app_state::AppState,
    components::{Component, root::RootComponent},
    states::{Action, InspectOption},
    update::Update,
};

pub struct SearchUpdate;

impl Update for SearchUpdate {
    fn update(&self, action: &Action, _s: &mut AppState, root: &mut RootComponent) -> Vec<Action> {
        let Action::SubmitSearch(query) = action else {
            return Vec::new();
        };

        match root.tabs.selected() {
            InspectOption::Ledger => root.ledger_page.handle_search(query),
            InspectOption::Chain => root.chain_page.handle_search(query),
            _ => {}
        }

        vec![Action::Render]
    }
}

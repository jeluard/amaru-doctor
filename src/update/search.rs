use crate::{
    app_state::AppState,
    states::{Action, ComponentId, InspectOption},
    update::Update,
};

pub struct SearchUpdate;

impl Update for SearchUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::SubmitSearch(query) = action else {
            return Vec::new();
        };

        match s.get_inspect_tabs().selected() {
            InspectOption::Ledger => {
                if let Some(page) = s.component_registry.get_mut(&ComponentId::LedgerPage) {
                    page.handle_search(query);
                }
            }
            InspectOption::Chain => {
                if let Some(comp) = s.component_registry.get_mut(&ComponentId::ChainSearch) {
                    comp.handle_search(query);
                }
            }
            _ => {}
        }

        Vec::new()
    }
}

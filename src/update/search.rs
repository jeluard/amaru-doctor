use crate::{
    app_state::AppState,
    components::{chain_page::ChainPageComponent, root::RootComponent},
    states::{Action, ComponentId, InspectOption},
    update::Update,
};

pub struct SearchUpdate;

impl Update for SearchUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::SubmitSearch(query) = action else {
            return Vec::new();
        };

        let selected_tab = s
            .component_registry
            .get(&ComponentId::Root)
            .and_then(|comp| comp.as_any().downcast_ref::<RootComponent>())
            .map(|root| root.tabs.selected())
            .unwrap_or(InspectOption::Ledger);

        match selected_tab {
            InspectOption::Ledger => {
                if let Some(page) = s.component_registry.get_mut(&ComponentId::LedgerPage) {
                    page.handle_search(query);
                }
            }
            InspectOption::Chain => {
                if let Some(page) = s.component_registry.get_mut(&ComponentId::ChainPage)
                    && let Some(chain_page) = page.as_any_mut().downcast_mut::<ChainPageComponent>()
                {
                    chain_page.handle_search(query);
                }
            }
            _ => {}
        }

        Vec::new()
    }
}

use crate::{
    app_state::AppState,
    components::{Component, search_list::SearchListComponent},
    states::{Action, ComponentId, InspectOption},
    ui::to_list_item::UtxoItem,
    update::Update,
};
use amaru_kernel::Address;

pub struct SearchUpdate;

impl Update for SearchUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::SubmitSearch(query) = action else {
            return Vec::new();
        };

        match s.get_inspect_tabs().selected() {
            InspectOption::Ledger => {
                if let Some(comp) = s
                    .component_registry
                    .get_mut(&ComponentId::LedgerUtxosByAddrList)
                    && let Some(list) = comp
                        .as_any_mut()
                        .downcast_mut::<SearchListComponent<Address, UtxoItem>>()
                {
                    list.handle_search(query);
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

use crate::{
    app_state::AppState,
    components::{Component, ledger_page::LedgerPageComponent},
    states::{Action, ComponentId},
    update::Update,
};

pub struct PollSearchUpdate;
impl Update for PollSearchUpdate {
    fn update(&self, _action: &Action, s: &mut AppState) -> Vec<Action> {
        if let Some(page) = s.component_registry.get_mut(&ComponentId::LedgerPage)
            && let Some(ledger_page) = page.as_any_mut().downcast_mut::<LedgerPageComponent>()
        {
            return ledger_page.tick();
        }

        Vec::new()
    }
}

use crate::{
    app_state::AppState,
    components::{Component, root::RootComponent},
    states::Action,
    update::Update,
};

pub struct PollSearchUpdate;
impl Update for PollSearchUpdate {
    fn update(&self, _action: &Action, _s: &mut AppState, root: &mut RootComponent) -> Vec<Action> {
        root.ledger_page.tick()
    }
}

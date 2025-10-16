use crate::{
    app_state::AppState,
    model::button::{ButtonId, InputEvent},
    states::Action,
    update::Update,
};
use tracing::debug;

pub struct GetButtonEventsUpdate;
impl Update for GetButtonEventsUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        let mut actions = Vec::new();
        if *a == Action::GetButtonEvents {
            for input_event in s.button_events.try_iter() {
                actions.push(translate_input(input_event));
            }
        }
        actions
    }
}

fn translate_input(input_event: InputEvent) -> Action {
    match input_event {
        InputEvent {
            id: ButtonId::A, ..
        } => {
            debug!("Translating Button A press to Up Action");
            Action::Up
        }
        InputEvent {
            id: ButtonId::B, ..
        } => {
            debug!("Translating Button B press to Down Action");
            Action::Down
        }
        InputEvent {
            id: ButtonId::X, ..
        } => {
            debug!("Translating Button X press to Forward Action");
            Action::Forward
        }
        InputEvent {
            id: ButtonId::Y, ..
        } => {
            debug!("Translating Button Y press to Back Action");
            Action::Back
        }
    }
}

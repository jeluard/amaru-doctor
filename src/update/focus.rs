use crate::{app_state::AppState, model::layout::MoveFocus, states::Action, update::Update};

pub struct FocusUpdate;
impl Update for FocusUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action> {
        let dir = match action {
            Action::FocusUp => MoveFocus::Up,
            Action::FocusDown => MoveFocus::Down,
            Action::FocusLeft => MoveFocus::Left,
            Action::FocusRight => MoveFocus::Right,
            _ => return vec![],
        };
        app_state.layout_model.set_focus_by_move(dir);
        Vec::new()
    }
}

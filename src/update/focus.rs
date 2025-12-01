use crate::{
    app_state::AppState, components::root::RootComponent, model::layout::MoveFocus, states::Action,
    update::Update,
};

pub struct FocusUpdate;
impl Update for FocusUpdate {
    fn update(
        &self,
        action: &Action,
        app_state: &mut AppState,
        _root: &mut RootComponent,
    ) -> Vec<Action> {
        match action {
            Action::FocusUp => app_state.layout_model.set_focus_by_move(MoveFocus::Up),
            Action::FocusDown => app_state.layout_model.set_focus_by_move(MoveFocus::Down),
            Action::FocusLeft => app_state.layout_model.set_focus_by_move(MoveFocus::Left),
            Action::FocusRight => app_state.layout_model.set_focus_by_move(MoveFocus::Right),
            Action::SetFocus(id) => {
                app_state.layout_model.set_focus(*id);
                return vec![Action::Render];
            }
            _ => {
                return Vec::new();
            }
        };
        Vec::new()
    }
}

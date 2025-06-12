use crate::{
    app_state::AppState,
    model::cursor::Cursor,
    states::{Action, WidgetSlot},
    update::Update,
};

pub struct FocusUpdate {
    pub get_focus: fn(&mut AppState) -> &mut Cursor<WidgetSlot>,
}

impl Update for FocusUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) {
        match action {
            Action::FocusPrev => (self.get_focus)(app_state).next_back(),
            Action::FocusNext => (self.get_focus)(app_state).next(),
            _ => {}
        }
    }
}

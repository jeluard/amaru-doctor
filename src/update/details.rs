use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    model::window::WindowState,
    states::{Action, WidgetId},
    update::Update,
};

pub struct DetailsUpdate<T> {
    pub widget_id: WidgetId,
    pub get_details: fn(&mut AppState) -> &mut WindowState<T>,
}

impl<T> Update for DetailsUpdate<T> {
    fn update(&self, action: &Action, app_state: &mut AppState) {
        let is_focused = is_widget_focused(app_state, &self.widget_id);
        let _details = (self.get_details)(app_state);

        match action {
            Action::ScrollUp => {
                if is_focused {
                    // TODO: Add offset to AppState
                    // details.scroll_up();
                }
            }
            Action::ScrollDown => {
                if is_focused {
                    // details.scroll_down();
                }
            }
            _ => {}
        }
    }
}

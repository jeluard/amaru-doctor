use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    model::window::WindowState,
    states::{Action, WidgetId},
    update::Update,
};

pub struct ListUpdate<T> {
    pub widget_id: WidgetId,
    pub get_list: fn(&mut AppState) -> &mut WindowState<T>,
}

impl<T> Update for ListUpdate<T> {
    fn update(&self, action: &Action, app_state: &mut AppState) {
        let is_focused = is_widget_focused(app_state, &self.widget_id);
        let list = (self.get_list)(app_state);

        match action {
            Action::ScrollUp => {
                if is_focused {
                    list.scroll_up();
                }
            }
            Action::ScrollDown => {
                if is_focused {
                    list.scroll_down();
                }
            }
            _ => {}
        }
    }
}

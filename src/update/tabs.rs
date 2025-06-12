use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    model::cursor::Cursor,
    states::{Action, Tab, WidgetId},
    update::Update,
};

pub struct TabsUpdate {
    pub widget_id: WidgetId,
    pub get_tabs: fn(&mut AppState) -> &mut Cursor<Tab>,
}

impl Update for TabsUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) {
        let is_focused = is_widget_focused(app_state, &self.widget_id);
        let tabs = (self.get_tabs)(app_state);

        match action {
            Action::ScrollUp => {
                if is_focused {
                    tabs.next_back();
                }
            }
            Action::ScrollDown => {
                if is_focused {
                    tabs.next();
                }
            }
            _ => {}
        }
    }
}

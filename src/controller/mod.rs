use crate::{
    app_state::AppState,
    states::WidgetSlot::{self},
};

pub mod layout;

pub fn is_widget_focused(app_state: &AppState, widget_slot: WidgetSlot) -> bool {
    app_state.slot_focus.current() == &widget_slot
}

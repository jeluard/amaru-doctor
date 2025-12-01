use crate::{
    app_state::AppState,
    components::root::RootComponent,
    states::Action,
    update::{
        button::GetButtonEventsUpdate, focus::FocusUpdate, layout::LayoutUpdate,
        mouse::MouseEventUpdate, mouse_click::MouseClickUpdate, tabs::TabsUpdate, tick::TickUpdate,
    },
};

pub mod button;
pub mod focus;
pub mod layout;
pub mod mouse;
pub mod mouse_click;
pub mod tabs;
pub mod tick;

pub type UpdateList = Vec<&'static dyn Update>;

pub trait Update: Sync {
    fn update(
        &self,
        action: &Action,
        app_state: &mut AppState,
        root: &mut RootComponent,
    ) -> Vec<Action>;
}

pub static UPDATE_DEFS: &[&dyn Update] = &[
    &FocusUpdate,
    &LayoutUpdate,
    &TickUpdate,
    &GetButtonEventsUpdate,
    &MouseEventUpdate,
    &MouseClickUpdate,
    &TabsUpdate,
];

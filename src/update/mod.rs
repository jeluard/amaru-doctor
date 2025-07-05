use crate::{
    app_state::AppState,
    states::Action,
    update::{
        focus::FocusUpdate, layout::LayoutUpdate, scroll::ScrollUpdate, search::SearchUpdate,
        window::WindowSizeUpdate,
    },
};

pub mod focus;
pub mod layout;
pub mod scroll;
pub mod search;
pub mod window;

pub type UpdateList = Vec<&'static dyn Update>;
pub trait Update: Sync {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action>;
}

pub static UPDATE_DEFS: &[&dyn Update] = &[
    &FocusUpdate,
    &ScrollUpdate,
    &WindowSizeUpdate,
    &SearchUpdate,
    &LayoutUpdate,
];

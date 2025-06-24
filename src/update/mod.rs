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

pub type UpdateList = Vec<Box<dyn Update>>;
pub trait Update {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action>;
}

pub fn get_updates() -> UpdateList {
    vec![
        Box::new(FocusUpdate {}),
        Box::new(ScrollUpdate {}),
        Box::new(WindowSizeUpdate {}),
        Box::new(SearchUpdate {}),
        Box::new(LayoutUpdate {}),
    ]
}

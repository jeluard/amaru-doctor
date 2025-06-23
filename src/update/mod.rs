use crate::{
    app_state::AppState,
    states::Action,
    update::{
        focus::FocusUpdate, scroll::ScrollUpdate, search::SearchQuery, window::WindowSizeUpdate,
    },
};

pub mod focus;
pub mod scroll;
pub mod search;
pub mod window;

pub type UpdateList = Vec<Box<dyn Update>>;
pub trait Update {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Option<Action>;
}

pub fn get_updates() -> UpdateList {
    vec![
        Box::new(FocusUpdate {}),
        Box::new(ScrollUpdate {}),
        Box::new(WindowSizeUpdate {}),
        Box::new(SearchQuery {}),
    ]
}

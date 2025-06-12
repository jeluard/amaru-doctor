use crate::{
    app_state::AppState,
    states::Action,
    update::{focus::FocusUpdate, scroll::ScrollUpdate, window::WindowSizeUpdate},
};

pub mod focus;
pub mod scroll;
pub mod window;

pub type UpdateList = Vec<Box<dyn Update>>;
pub trait Update {
    fn update(&self, action: &Action, app_state: &mut AppState);
}

pub fn get_updates() -> UpdateList {
    let updates: UpdateList = vec![
        Box::new(FocusUpdate {
            get_focus: |s: &mut AppState| &mut s.slot_focus,
        }),
        Box::new(ScrollUpdate {}),
        Box::new(WindowSizeUpdate {}),
    ];

    updates
}

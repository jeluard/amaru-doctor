use crate::{
    ScreenMode,
    model::{button::InputEvent, layout::LayoutModel},
    states::{ComponentId, InspectOption, LedgerMode},
    update::mouse::MouseState,
};
use anyhow::Result;
use ratatui::layout::Rect;
use std::sync::mpsc;

/// Holds ALL the app's state. Does not self-mutate.
pub struct AppState {
    pub screen_mode: ScreenMode,
    pub frame_area: Rect,
    pub layout_model: LayoutModel,
    pub button_events: mpsc::Receiver<InputEvent>,
    pub mouse_state: MouseState,
    pub focused_component: ComponentId,
}

impl AppState {
    pub fn new(
        button_events: mpsc::Receiver<InputEvent>,
        frame_area: Rect,
        screen_mode: ScreenMode,
    ) -> Result<Self> {
        let layout_model = LayoutModel::new(
            screen_mode,
            InspectOption::default(),
            LedgerMode::default(),
            frame_area,
        );

        Ok(Self {
            screen_mode,
            frame_area: Rect::default(),
            layout_model,
            button_events,
            mouse_state: MouseState::default(),
            focused_component: ComponentId::InspectTabs,
        })
    }
}

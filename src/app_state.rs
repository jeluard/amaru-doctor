use crate::{ScreenMode, model::button::InputEvent};
use anyhow::Result;
use ratatui::layout::Rect;
use std::sync::mpsc;

/// Holds ALL the app's state. Does not self-mutate.
pub struct AppState {
    pub screen_mode: ScreenMode,
    pub frame_area: Rect,
    pub button_events: mpsc::Receiver<InputEvent>,
}

impl AppState {
    pub fn new(
        button_events: mpsc::Receiver<InputEvent>,
        frame_area: Rect,
        screen_mode: ScreenMode,
    ) -> Result<Self> {
        Ok(Self {
            screen_mode,
            frame_area,
            button_events,
        })
    }
}

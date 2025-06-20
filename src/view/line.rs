use crate::{app_state::AppState, view::View};
use color_eyre::Result;
use ratatui::{Frame, layout::Rect, widgets::Paragraph};

pub struct LineView {
    get_line: fn(&AppState) -> String,
}

impl LineView {
    pub fn new(get_line: fn(&AppState) -> String) -> Self {
        Self { get_line }
    }
}

impl View for LineView {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        let line = (self.get_line)(app_state);
        let tabs_widget = Paragraph::new(line);
        frame.render_widget(tabs_widget, area);
        Ok(())
    }
}

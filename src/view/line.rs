use crate::{app_state::AppState, states::WidgetId, view::View};
use color_eyre::Result;
use ratatui::{Frame, layout::Rect, widgets::Paragraph};

pub struct LineView {
    _widget_id: WidgetId,
    get_line: fn(&AppState) -> String,
}

impl LineView {
    pub fn new(_widget_id: WidgetId, get_line: fn(&AppState) -> String) -> Self {
        Self {
            _widget_id,
            get_line,
        }
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

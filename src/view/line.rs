use ratatui::{Frame, layout::Rect, widgets::Paragraph};

pub fn render_line(frame: &mut Frame, area: Rect, line: String) {
    let tabs_widget = Paragraph::new(line);
    frame.render_widget(tabs_widget, area);
}

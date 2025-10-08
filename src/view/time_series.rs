use crate::model::time_series::TimeSeries;
use ratatui::{
    prelude::*,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};

pub fn render_chart(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    color: Color,
    time_series: &TimeSeries,
) {
    let data = time_series.data();
    let (x_bounds, y_bounds) = time_series.get_bounds();

    let dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(color))
        .data(&data);

    let x_axis = Axis::default()
        .title("Time (s)")
        .style(Style::default().fg(Color::Gray))
        .bounds(x_bounds)
        .labels(vec![
            Span::from(format!("{:.1}", x_bounds[0])),
            Span::from(format!("{:.1}", (x_bounds[0] + x_bounds[1]) / 2.0)),
            Span::from(format!("{:.1}", x_bounds[1])),
        ]);

    let y_axis = Axis::default()
        .title(title)
        .style(Style::default().fg(Color::Gray))
        .bounds(y_bounds)
        .labels(vec![
            Span::from(format!("{:.2}", y_bounds[0])),
            Span::from(format!("{:.2}", y_bounds[1])),
        ]);

    let chart = Chart::new(vec![dataset])
        .block(Block::default().title(title).borders(Borders::ALL))
        .x_axis(x_axis)
        .y_axis(y_axis);

    frame.render_widget(chart, area);
}

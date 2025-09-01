use crate::{model::window::WindowState, ui::to_list_item::ToListItem};
use anyhow::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn render_list<T: ToListItem>(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    list_opt: Option<&WindowState<T>>,
    is_focused: bool,
) -> Result<()> {
    let mut block = Block::default().title(title).borders(Borders::ALL);
    if is_focused {
        block = block
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::White));
    }

    match list_opt {
        Some(list) => {
            let (view, selected) = list.window_view();
            let items: Vec<ListItem> = view.iter().map(|i| i.to_list_item()).collect();

            let widget = List::new(items)
                .highlight_symbol(">> ")
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .block(block);

            let mut state = ListState::default();
            state.select(selected);

            frame.render_stateful_widget(widget, area, &mut state);
        }
        None => {
            let msg = Paragraph::new(Line::from(Span::raw("Nothing to list"))).block(block);
            frame.render_widget(msg, area);
        }
    }

    Ok(())
}

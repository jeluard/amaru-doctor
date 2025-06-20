use crate::{
    app_state::AppState, controller::is_widget_focused, model::window::WindowState,
    states::WidgetSlot, ui::to_list_item::ToListItem, view::View,
};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub struct ListView<T> {
    title: &'static str,
    widget_slot: WidgetSlot,
    get_list: fn(&AppState) -> &WindowState<T>,
}

impl<T> ListView<T> {
    pub fn new(
        title: &'static str,
        widget_slot: WidgetSlot,
        get_list: fn(&AppState) -> &WindowState<T>,
    ) -> Self {
        Self {
            title,
            widget_slot,
            get_list,
        }
    }
}

impl<T: ToListItem> View for ListView<T> {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        render_list(
            frame,
            area,
            app_state,
            self.title,
            &self.widget_slot,
            Some((self.get_list)(app_state)),
        )
    }
}

pub struct OptListView<T> {
    title: &'static str,
    widget_slot: WidgetSlot,
    get_list: fn(&AppState) -> Option<&WindowState<T>>,
}

impl<T> OptListView<T> {
    pub fn new(
        title: &'static str,
        widget_slot: WidgetSlot,
        get_list: fn(&AppState) -> Option<&WindowState<T>>,
    ) -> Self {
        Self {
            title,
            widget_slot,
            get_list,
        }
    }
}

impl<T: ToListItem> View for OptListView<T> {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        render_list(
            frame,
            area,
            app_state,
            self.title,
            &self.widget_slot,
            (self.get_list)(app_state),
        )
    }
}

fn render_list<T: ToListItem>(
    frame: &mut Frame,
    area: Rect,
    app_state: &AppState,
    title: &str,
    widget_slot: &WidgetSlot,
    list_opt: Option<&WindowState<T>>,
) -> Result<()> {
    let mut block = Block::default().title(title).borders(Borders::ALL);
    if is_widget_focused(app_state, widget_slot) {
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

use crate::{
    app_state::AppState,
    controller::is_widget_focused,
    model::window::WindowState,
    states::WidgetSlot,
    ui::to_rich::{RichText, ToRichText},
    view::View,
};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct DetailsView<T> {
    title: &'static str,
    widget_slot: WidgetSlot,
    get_list: fn(&AppState) -> &WindowState<T>,
}

impl<T> DetailsView<T> {
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

impl<T: ToRichText> View for DetailsView<T> {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        render_details(
            frame,
            area,
            app_state,
            self.title,
            &self.widget_slot,
            Some((self.get_list)(app_state)),
        )
    }
}

pub struct OptDetailsView<T> {
    title: &'static str,
    widget_slot: WidgetSlot,
    get_list: fn(&AppState) -> Option<&WindowState<T>>,
}

impl<T> OptDetailsView<T> {
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

impl<T: ToRichText> View for OptDetailsView<T> {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()> {
        render_details(
            frame,
            area,
            app_state,
            self.title,
            &self.widget_slot,
            (self.get_list)(app_state),
        )
    }
}

fn render_details<T: ToRichText>(
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

    let widget = match list_opt {
        Some(list) => {
            let lines = list
                .selected()
                .map(ToRichText::to_rich_text)
                .unwrap_or(RichText::Single(Span::raw("Nothing selected")))
                .unwrap_lines();
            // TODO: Add offset state to AppState
            // .scroll((self.scroll_offset, 0));
            Paragraph::new(lines).wrap(Wrap { trim: true })
        }
        None => Paragraph::new(Line::from(Span::raw("Nothing to detail"))),
    }
    .block(block);
    frame.render_widget(widget, area);

    Ok(())
}

use crate::ui::RichText;
use crate::{model::window::WindowState, ui::ToRichText};
use color_eyre::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_details<T: ToRichText>(
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

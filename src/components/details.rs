use super::Component;
use crate::{
    app_state::AppState,
    focus,
    shared::{GetterOpt, Shared},
    states::{Action, WidgetId},
    ui::to_rich::{RichText, ToRichText},
    window::WindowState,
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tracing::trace;

pub struct DetailsComponent<K>
where
    K: Clone + ToRichText,
{
    comp_id: WidgetId,
    shared: Shared<WindowState<K>>,
    app_state: Shared<AppState>,
    scroll_offset: u16,
}

impl<K> DetailsComponent<K>
where
    K: Clone + ToRichText,
{
    pub fn new(
        comp_id: WidgetId,
        shared: Shared<WindowState<K>>,
        app_state: Shared<AppState>,
    ) -> Self {
        Self {
            comp_id,
            shared,
            app_state,
            scroll_offset: 0,
        }
    }

    fn has_focus(&self) -> bool {
        match focus::get_focused(self.app_state.clone()) {
            Some(id) => self.comp_id == id,
            _ => false,
        }
    }
}

impl<K> Component for DetailsComponent<K>
where
    K: Clone + ToRichText,
{
    fn debug_name(&self) -> String {
        format!("DetailsComponent:{}", self.comp_id)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            trace!("DetailsComponent::{}: no focus", self.comp_id);
            return Ok(vec![]);
        }
        trace!("DetailsComponent::{}: has focus", self.comp_id);

        match key.code {
            KeyCode::Up => self.scroll_offset = self.scroll_offset.saturating_sub(1),
            KeyCode::Down => self.scroll_offset = self.scroll_offset.saturating_add(1),
            _ => {}
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut block = Block::default()
            .title(self.comp_id.to_string())
            .borders(Borders::ALL);

        if self.has_focus() {
            block = block
                .title_style(Style::default().fg(Color::White))
                .border_style(Style::default().fg(Color::Blue));
        }

        let lines = self
            .shared
            .borrow()
            .get()
            .map_or(RichText::Single(Span::raw("None selected")), |t| {
                t.to_rich_text()
            })
            .unwrap_lines();
        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true })
            .scroll((self.scroll_offset, 0));
        frame.render_widget(paragraph, area);
        Ok(())
    }
}

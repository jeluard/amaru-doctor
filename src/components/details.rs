use super::Component;
use crate::{
    action::Action,
    focus::{FocusState, FocusableComponent},
    shared::SharedGetter,
    to_rich::ToRichText,
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tracing::trace;

pub struct DetailsComponent<'a, K>
where
    K: Clone + ToRichText,
{
    title: String,
    shared: SharedGetter<'a, K>,
    focus: FocusState,
    scroll_offset: u16,
}

impl<'a, K> DetailsComponent<'a, K>
where
    K: Clone + ToRichText,
{
    pub fn new(title: String, shared: SharedGetter<'a, K>) -> Self {
        Self {
            title,
            shared,
            focus: FocusState::default(),
            scroll_offset: 0,
        }
    }
}

impl<'a, K> FocusableComponent for DetailsComponent<'a, K>
where
    K: Clone + ToRichText,
{
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl<'a, K> Component for DetailsComponent<'a, K>
where
    K: Clone + ToRichText,
{
    fn debug_name(&self) -> String {
        format!("DetailsComponent:{}", self.title)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            trace!("DetailsComponent::{}: no focus", self.title);
            return Ok(vec![]);
        }
        trace!("DetailsComponent::{}: has focus", self.title);

        match key.code {
            KeyCode::Up => self.scroll_offset = self.scroll_offset.saturating_sub(1),
            KeyCode::Down => self.scroll_offset = self.scroll_offset.saturating_add(1),
            _ => {}
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut block = Block::default()
            .title(self.title.clone())
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);

        if self.has_focus() {
            block = block.border_style(Style::default().fg(Color::Blue));
        }

        let lines = match self.shared.borrow_mut().get_mut() {
            Some(val) => val.to_rich_text().unwrap_lines(),
            None => vec![Line::from("None selected")],
        };

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true })
            .scroll((self.scroll_offset, 0));
        frame.render_widget(paragraph, area);
        Ok(())
    }
}

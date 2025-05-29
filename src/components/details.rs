use super::Component;
use crate::{
    action::{Action, SelectedState, SelectsFrom},
    focus::{FocusState, Focusable},
    to_rich::RichText,
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

pub struct DetailsComponent<K, F>
where
    K: Clone + PartialEq,
    F: Fn(&K) -> Result<Option<RichText>>,
{
    title: String,
    selected: SelectedState<K>,
    focus: FocusState,
    scroll_offset: u16,
    render: F,
}

impl<K, F> DetailsComponent<K, F>
where
    K: Clone + PartialEq + SelectsFrom,
    F: Fn(&K) -> Result<Option<RichText>>,
{
    pub fn new(title: String, render: F) -> Self {
        Self {
            title,
            selected: SelectedState::new(),
            focus: FocusState::default(),
            scroll_offset: 0,
            render,
        }
    }
}

impl<K, F> Focusable for DetailsComponent<K, F>
where
    K: Clone + PartialEq + SelectsFrom,
    F: Fn(&K) -> Result<Option<RichText>>,
{
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl<K, F> Component for DetailsComponent<K, F>
where
    K: Clone + PartialEq + SelectsFrom,
    F: Fn(&K) -> Result<Option<RichText>>,
{
    fn update(&mut self, action: Action) -> Result<Vec<Action>> {
        if self.selected.update(&action) {
            self.scroll_offset = 0;
        }
        Ok(vec![])
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            return Ok(vec![]);
        }

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

        let lines = match self.selected.value.as_ref() {
            Some(key) => match (self.render)(key)? {
                Some(rich) => rich.unwrap_lines(),
                None => vec![Line::from("Not found")],
            },
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

use crate::{
    action::Action,
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::Getter,
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use std::cell::{Ref, RefCell};

pub struct SearchComponent {
    title: String,
    input: String,
    cursor_position: usize,
    query: RefCell<Option<String>>,
    focus: FocusState,
}

impl SearchComponent {
    pub fn new(title: String) -> Self {
        Self {
            title,
            input: String::new(),
            cursor_position: 0,
            query: RefCell::new(None),
            focus: FocusState::default(),
        }
    }
}

impl FocusableComponent for SearchComponent {
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl Getter<String> for SearchComponent {
    fn get(&self) -> Option<Ref<String>> {
        Ref::filter_map(self.query.borrow(), |q| q.as_ref()).ok()
    }
}

impl Component for SearchComponent {
    fn debug_name(&self) -> String {
        "SearchComponent".to_string()
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            return Ok(vec![]);
        }

        match key.code {
            KeyCode::Char(c) => {
                self.input.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.input.remove(self.cursor_position);
                }
            }
            KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_position < self.input.len() {
                    self.cursor_position += 1;
                }
            }
            KeyCode::Enter => {
                self.query.borrow_mut().replace(self.input.to_owned());
            }
            _ => {}
        }

        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let block = Block::default()
            .title(self.title.clone())
            .title_style(if self.has_focus() {
                Style::default().fg(Color::White)
            } else {
                Style::default()
            })
            .borders(Borders::ALL)
            .border_style(if self.has_focus() {
                Style::default().fg(Color::Blue)
            } else {
                Style::default()
            });

        let paragraph = Paragraph::new(self.input.clone()).block(block);
        frame.render_widget(paragraph, area);

        if self.has_focus() {
            let cursor_x = area.x + 1 + self.cursor_position.min(self.input.len()) as u16;
            let cursor_y = area.y + 1;
            frame.set_cursor_position(Position::new(cursor_x, cursor_y));
        }

        Ok(())
    }
}

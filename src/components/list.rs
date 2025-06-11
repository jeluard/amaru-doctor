use crate::{
    app_state::AppState,
    components::Component,
    shared::Shared,
    states::{Action, WidgetId},
    ui::to_list_item::ToListItem,
    window::WindowState,
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use tracing::trace;

pub struct ListComponent<T>
where
    T: Clone + ToListItem,
{
    comp_id: WidgetId,
    state: Shared<WindowState<T>>,
    app_state: Shared<AppState>,
}

impl<T> ListComponent<T>
where
    T: Clone + ToListItem,
{
    pub fn from_iter(
        comp_id: WidgetId,
        state: Shared<WindowState<T>>,
        app_state: Shared<AppState>,
    ) -> Self {
        Self {
            comp_id,
            state,
            app_state,
        }
    }

    fn has_focus(&self) -> bool {
        self.app_state
            .borrow()
            .is_widget_focused(self.comp_id.clone())
    }
}

impl<T> Component for ListComponent<T>
where
    T: Clone + ToListItem,
{
    fn debug_name(&self) -> String {
        format!("ListComponent:{}", self.comp_id)
    }

    fn handle_key_event(&mut self, evt: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            trace!("{}: No focus", self.debug_name());
            return Ok(vec![]);
        }
        trace!("{}: Have focus", self.debug_name());
        let mut actions = vec![];
        match evt.code {
            KeyCode::Up => actions.push(Action::ScrollUp(self.comp_id.clone())),
            KeyCode::Down => actions.push(Action::ScrollDown(self.comp_id.clone())),
            _ => {}
        }
        Ok(actions)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // TODO: Capture somewhere else
        self.state.borrow_mut().set_window_size(area.rows().count());

        let binding = self.state.borrow();
        let (view, selected) = binding.window_view();
        let items: Vec<ListItem> = view.iter().map(|i| i.to_list_item()).collect();

        let mut block = Block::default()
            .title(serde_plain::to_string(&self.comp_id)?)
            .borders(Borders::ALL);
        if self.has_focus() {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let list = List::new(items)
            .highlight_symbol(">> ")
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .block(block);

        let mut state = ListState::default();
        state.select(Some(selected));
        frame.render_stateful_widget(list, area, &mut state);

        Ok(())
    }
}

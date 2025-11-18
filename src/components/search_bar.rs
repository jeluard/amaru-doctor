use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, MouseScrollDirection},
    states::{Action, ComponentId, InspectOption, LedgerSearch},
    update::scroll::ScrollDirection,
    view::search::render_search_query,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};
use std::any::Any;

pub struct SearchBarComponent {
    id: ComponentId,
}

impl SearchBarComponent {
    pub fn new(id: ComponentId) -> Self {
        Self { id }
    }
}

impl Component for SearchBarComponent {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn calculate_layout(&self, area: Rect, _s: &AppState) -> ComponentLayout {
        let mut layout = ComponentLayout::new();
        layout.insert(self.id, area);
        layout
    }

    /// Renders the search bar, pulling its state from the old model
    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };
        let is_focused = s.layout_model.is_component_focused(self.id);

        // This is the old, complex logic from view/defs.rs
        let search_query = match s.get_inspect_tabs().selected() {
            InspectOption::Ledger => match s.get_ledger_search_options().model_view.selected_item()
            {
                Some(LedgerSearch::UtxosByAddress) => &s.ledger_mvs.utxos_by_addr_search.builder,
                None => "",
            },
            InspectOption::Chain => &s.chain_view.chain_search.builder,
            InspectOption::Otel => "",
            InspectOption::Prometheus => "",
        };

        render_search_query(f, area, "Search", search_query, is_focused);
    }
    fn handle_scroll(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_key_event(&mut self, key: KeyEvent) -> Vec<Action> {
        match key.code {
            KeyCode::Char(c) => {
                return vec![Action::Key(KeyCode::Char(c))];
            }
            KeyCode::Backspace => {
                return vec![Action::Key(KeyCode::Backspace)];
            }
            _ => {}
        }
        Vec::new()
    }
    fn handle_click(&mut self, _area: Rect, _row: u16, _col: u16) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_scroll(&mut self, _direction: MouseScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_drag(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
}

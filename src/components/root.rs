use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, InputRoute, MouseScrollDirection, ScrollDirection},
    states::{Action, ComponentId, InspectOption},
    tui::Event,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Frame, layout::Rect};
use std::{any::Any, collections::HashMap};

pub struct RootComponent {
    id: ComponentId,
}

impl Default for RootComponent {
    fn default() -> Self {
        Self {
            id: ComponentId::Root,
        }
    }
}

impl Component for RootComponent {
    fn id(&self) -> ComponentId {
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn calculate_layout(&self, area: Rect, s: &AppState) -> ComponentLayout {
        let active_page = match s.get_inspect_tabs().selected() {
            InspectOption::Ledger => ComponentId::LedgerPage,
            InspectOption::Chain => ComponentId::ChainPage,
            InspectOption::Otel => ComponentId::OtelPage,
            InspectOption::Prometheus => ComponentId::PrometheusPage,
        };

        let mut layout = HashMap::new();
        // The Root gives the full screen to the Active Page
        layout.insert(active_page, area);
        layout
    }

    fn route_event(&self, event: &Event, s: &AppState) -> InputRoute {
        // Check Global Keys FIRST
        if let Event::Key(key) = event {
            // Tab cycles focus
            if key.code == KeyCode::Tab {
                return InputRoute::Handle;
            }
            // Ctrl+C quits
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                return InputRoute::Handle;
            }
        }

        // Delegate everything else to the Active Page
        let active_page = match s.get_inspect_tabs().selected() {
            InspectOption::Ledger => ComponentId::LedgerPage,
            InspectOption::Chain => ComponentId::ChainPage,
            InspectOption::Otel => ComponentId::OtelPage,
            InspectOption::Prometheus => ComponentId::PrometheusPage,
        };
        InputRoute::Delegate(active_page, s.frame_area)
    }

    fn handle_event(&mut self, event: &Event, _area: Rect) -> Vec<Action> {
        // Handle the events we claimed above
        if let Event::Key(key) = event {
            if key.code == KeyCode::Tab {
                return vec![Action::FocusNext];
            }
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                return vec![Action::Quit];
            }
        }
        Vec::new()
    }

    fn render(&self, f: &mut Frame, s: &AppState, _layout: &ComponentLayout) {
        let area = f.area();
        let my_layout = self.calculate_layout(area, s);

        // Only render the active page found in the layout
        for (id, _) in my_layout.iter() {
            if let Some(page) = s.component_registry.get(id) {
                page.render(f, s, &my_layout);
            }
        }
    }

    fn handle_scroll(&mut self, _d: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_key_event(&mut self, _k: KeyEvent) -> Vec<Action> {
        Vec::new()
    }
    fn handle_click(&mut self, _a: Rect, _r: u16, _c: u16) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_scroll(&mut self, _d: MouseScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_drag(&mut self, _d: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
}

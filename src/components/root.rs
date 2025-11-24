use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, InputRoute, tabs::TabsComponent},
    controller::{LayoutSpec, walk_layout},
    states::{Action, ComponentId, InspectOption},
    tui::Event,
};
use crossterm::event::{KeyCode, KeyModifiers};
use either::Either::Left;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use std::{any::Any, collections::HashMap};

pub struct RootComponent {
    id: ComponentId,
    pub tabs: TabsComponent<InspectOption>,
}

impl Default for RootComponent {
    fn default() -> Self {
        Self {
            id: ComponentId::Root,
            tabs: TabsComponent::new(ComponentId::InspectTabs, false),
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
        let active_page = match self.tabs.selected() {
            InspectOption::Ledger => ComponentId::LedgerPage,
            InspectOption::Chain => ComponentId::ChainPage,
            InspectOption::Otel => ComponentId::OtelPage,
            InspectOption::Prometheus => ComponentId::PrometheusPage,
        };

        let spec = LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![
                (Constraint::Length(1), Left(ComponentId::InspectTabs)),
                (Constraint::Fill(1), Left(active_page)),
            ],
        };

        let mut layout = HashMap::new();
        walk_layout(&mut layout, &spec, area);
        if let Some(page_rect) = layout.get(&active_page)
            && let Some(page) = s.component_registry.get(&active_page)
        {
            let child_layout = page.calculate_layout(*page_rect, s);
            layout.extend(child_layout);
        }

        layout
    }

    fn render(&self, f: &mut Frame, s: &AppState, _ignored_layout: &ComponentLayout) {
        let area = f.area();
        let my_layout = self.calculate_layout(area, s);

        if let Some(tabs_area) = my_layout.get(&ComponentId::InspectTabs) {
            let mut tabs_layout = HashMap::new();
            tabs_layout.insert(ComponentId::InspectTabs, *tabs_area);
            self.tabs.render(f, s, &tabs_layout);
        }

        let active_page_id = match self.tabs.selected() {
            InspectOption::Ledger => ComponentId::LedgerPage,
            InspectOption::Chain => ComponentId::ChainPage,
            InspectOption::Otel => ComponentId::OtelPage,
            InspectOption::Prometheus => ComponentId::PrometheusPage,
        };

        if let Some(page) = s.component_registry.get(&active_page_id) {
            page.render(f, s, &my_layout);
        }
    }

    fn route_event(&self, event: &Event, s: &AppState) -> InputRoute {
        if let Event::Key(key) = event {
            if key.code == KeyCode::Tab {
                return InputRoute::Handle;
            }
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                return InputRoute::Handle;
            }
        }

        let area = s.frame_area;
        let my_layout = self.calculate_layout(area, s);

        if let Event::Mouse(mouse) = event
            && let Some(tabs_rect) = my_layout.get(&ComponentId::InspectTabs)
            && mouse.column >= tabs_rect.x
            && mouse.column < tabs_rect.x + tabs_rect.width
            && mouse.row >= tabs_rect.y
            && mouse.row < tabs_rect.y + tabs_rect.height
        {
            return InputRoute::Handle;
        }

        let active_page = match self.tabs.selected() {
            InspectOption::Ledger => ComponentId::LedgerPage,
            InspectOption::Chain => ComponentId::ChainPage,
            InspectOption::Otel => ComponentId::OtelPage,
            InspectOption::Prometheus => ComponentId::PrometheusPage,
        };

        if let Some(rect) = my_layout.get(&active_page) {
            return InputRoute::Delegate(active_page, *rect);
        }

        InputRoute::Ignore
    }

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        if let Event::Key(key) = event {
            if key.code == KeyCode::Tab {
                return vec![Action::FocusNext];
            }
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                return vec![Action::Quit];
            }
        }
        self.tabs.handle_event(event, area)
    }
}

use crate::{
    components::{
        Component, ComponentLayout, chain_page::ChainPageComponent,
        ledger_page::LedgerPageComponent, otel_page::OtelPageComponent, tabs::TabsComponent,
    },
    controller::{LayoutSpec, MoveFocus, walk_layout},
    metrics::page::MetricsPageComponent,
    otel::TraceGraphSnapshot,
    states::{Action, ComponentId, InspectOption},
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use either::Either::Left;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use std::{any::Any, collections::HashMap, sync::Arc};

pub struct RootComponent {
    id: ComponentId,
    pub tabs: TabsComponent<InspectOption>,
    pub ledger_page: LedgerPageComponent,
    pub chain_page: ChainPageComponent,
    pub otel_page: OtelPageComponent,
    pub metrics_page: MetricsPageComponent,
}

impl RootComponent {
    pub fn new(
        ledger_db: Arc<ReadOnlyRocksDB>,
        chain_db: Arc<ReadOnlyChainDB>,
        trace_graph: TraceGraphSnapshot,
    ) -> Self {
        Self {
            id: ComponentId::Root,
            tabs: TabsComponent::new(ComponentId::InspectTabs, false),
            ledger_page: LedgerPageComponent::new(ledger_db),
            chain_page: ChainPageComponent::new(chain_db),
            otel_page: OtelPageComponent::new(trace_graph),
            metrics_page: MetricsPageComponent::new_with_service(),
        }
    }

    fn calculate_layout(&self, area: Rect) -> ComponentLayout {
        let active_page_id = match self.tabs.selected() {
            InspectOption::Ledger => ComponentId::LedgerPage,
            InspectOption::Chain => ComponentId::ChainPage,
            InspectOption::Otel => ComponentId::OtelPage,
            InspectOption::Metrics => ComponentId::MetricsPage,
        };

        let spec = LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![
                (Constraint::Length(1), Left(ComponentId::InspectTabs)),
                (Constraint::Fill(1), Left(active_page_id)),
            ],
        };

        let mut layout = HashMap::new();
        walk_layout(&mut layout, &spec, area);

        if let Some(page_rect) = layout.get(&active_page_id) {
            let child_layout = match self.tabs.selected() {
                InspectOption::Ledger => self.ledger_page.calculate_layout(*page_rect),
                InspectOption::Chain => self.chain_page.calculate_layout(*page_rect),
                InspectOption::Otel => self.otel_page.calculate_layout(*page_rect),
                InspectOption::Metrics => self.metrics_page.calculate_layout(*page_rect),
            };
            layout.extend(child_layout);
        }

        layout
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let my_layout = self.calculate_layout(area);

        if let Some(tabs_area) = my_layout.get(&ComponentId::InspectTabs) {
            self.tabs.render_focused(frame, *tabs_area, false);
        }

        match self.tabs.selected() {
            InspectOption::Ledger => {
                if let Some(rect) = my_layout.get(&ComponentId::LedgerPage) {
                    self.ledger_page.render(frame, *rect);
                }
            }
            InspectOption::Chain => {
                if let Some(rect) = my_layout.get(&ComponentId::ChainPage) {
                    self.chain_page.render(frame, *rect);
                }
            }
            InspectOption::Otel => {
                if let Some(rect) = my_layout.get(&ComponentId::OtelPage) {
                    self.otel_page.render(frame, *rect);
                }
            }
            InspectOption::Metrics => {
                if let Some(rect) = my_layout.get(&ComponentId::MetricsPage) {
                    self.metrics_page.render(frame, *rect);
                }
            }
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

    fn tick(&mut self) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(self.tabs.tick());
        actions.extend(self.ledger_page.tick());
        actions.extend(self.chain_page.tick());
        actions.extend(self.otel_page.tick());
        actions.extend(self.metrics_page.tick());
        actions
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

        let tabs_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };

        let tab_actions = self.tabs.handle_event(event, tabs_area);
        if !tab_actions.is_empty() {
            return tab_actions;
        }

        let page_area = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: area.height.saturating_sub(3),
        };

        match self.tabs.selected() {
            InspectOption::Ledger => self.ledger_page.handle_event(event, page_area),
            InspectOption::Chain => self.chain_page.handle_event(event, page_area),
            InspectOption::Otel => self.otel_page.handle_event(event, page_area),
            InspectOption::Metrics => self.metrics_page.handle_event(event, page_area),
        }
    }

    fn handle_action(&mut self, action: Action) -> Vec<Action> {
        // Map Action::Focus* to MoveFocus enum
        let direction = match action {
            Action::FocusUp => MoveFocus::Up,
            Action::FocusDown => MoveFocus::Down,
            Action::FocusLeft => MoveFocus::Left,
            Action::FocusRight => MoveFocus::Right,
            _ => return Vec::new(),
        };

        // Delegate navigation to the active page
        match self.tabs.selected() {
            InspectOption::Ledger => self.ledger_page.handle_navigation(direction),
            InspectOption::Chain => self.chain_page.handle_navigation(direction),
            InspectOption::Otel => self.otel_page.handle_navigation(direction),
            InspectOption::Metrics => self.metrics_page.handle_navigation(direction),
        }
    }
}

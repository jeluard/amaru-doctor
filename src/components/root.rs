use crate::{
    components::{
        Component, ComponentLayout, chain_page::ChainPageComponent,
        ledger_page::LedgerPageComponent, otel_page::OtelPageComponent,
        prometheus_page::PrometheusPageComponent, tabs::TabsComponent,
    },
    controller::{LayoutSpec, MoveFocus, walk_layout},
    otel::TraceGraphSnapshot,
    prometheus::model::NodeMetrics,
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
use tokio::sync::mpsc::Receiver;

pub struct RootComponent {
    id: ComponentId,
    pub tabs: TabsComponent<InspectOption>,
    pub ledger_page: LedgerPageComponent,
    pub chain_page: ChainPageComponent,
    pub otel_page: OtelPageComponent,
    pub prometheus_page: PrometheusPageComponent,
}

impl RootComponent {
    pub fn new(
        ledger_db: Arc<ReadOnlyRocksDB>,
        chain_db: Arc<ReadOnlyChainDB>,
        trace_graph: TraceGraphSnapshot,
        prom_metrics: Receiver<NodeMetrics>,
    ) -> Self {
        Self {
            id: ComponentId::Root,
            tabs: TabsComponent::new(ComponentId::InspectTabs, false),
            ledger_page: LedgerPageComponent::new(ledger_db),
            chain_page: ChainPageComponent::new(chain_db),
            otel_page: OtelPageComponent::new(trace_graph),
            prometheus_page: PrometheusPageComponent::new(prom_metrics),
        }
    }

    fn calculate_layout(&self, area: Rect) -> ComponentLayout {
        let active_page_id = match self.tabs.selected() {
            InspectOption::Ledger => ComponentId::LedgerPage,
            InspectOption::Chain => ComponentId::ChainPage,
            InspectOption::Otel => ComponentId::OtelPage,
            InspectOption::Prometheus => ComponentId::PrometheusPage,
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
                InspectOption::Prometheus => self.prometheus_page.calculate_layout(*page_rect),
            };
            layout.extend(child_layout);
        }

        layout
    }

    pub fn render(&self, f: &mut Frame, _ignored_layout: &ComponentLayout) {
        let area = f.area();
        let my_layout = self.calculate_layout(area);

        // Render Tabs
        if let Some(tabs_area) = my_layout.get(&ComponentId::InspectTabs) {
            self.tabs.render_focused(f, *tabs_area, false);
        }

        // Render Active Page
        match self.tabs.selected() {
            InspectOption::Ledger => self.ledger_page.render(f, &my_layout),
            InspectOption::Chain => self.chain_page.render(f, &my_layout),
            InspectOption::Otel => self.otel_page.render(f, &my_layout),
            InspectOption::Prometheus => self.prometheus_page.render(f, &my_layout),
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
        actions.extend(self.prometheus_page.tick());
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
            InspectOption::Prometheus => self.prometheus_page.handle_event(event, page_area),
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
            InspectOption::Prometheus => self.prometheus_page.handle_navigation(direction),
        }
    }
}

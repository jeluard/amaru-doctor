use crate::{
    app_state::AppState,
    components::{
        Component, ComponentLayout, chain_page::ChainPageComponent,
        ledger_page::LedgerPageComponent, otel_page::OtelPageComponent,
        prometheus_page::PrometheusPageComponent, tabs::TabsComponent,
    },
    controller::{LayoutSpec, walk_layout},
    otel::TraceGraphSnapshot,
    prometheus::model::NodeMetrics,
    states::{Action, ComponentId, InspectOption},
    tui::Event,
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use crossterm::event::{KeyCode, KeyModifiers};
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
                InspectOption::Ledger => self.ledger_page.calculate_layout(*page_rect, s),
                InspectOption::Chain => self.chain_page.calculate_layout(*page_rect, s),
                InspectOption::Otel => self.otel_page.calculate_layout(*page_rect, s),
                InspectOption::Prometheus => self.prometheus_page.calculate_layout(*page_rect, s),
            };
            layout.extend(child_layout);
        }

        layout
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

    fn render(&self, f: &mut Frame, s: &AppState, _ignored_layout: &ComponentLayout) {
        let area = f.area();
        let my_layout = self.calculate_layout(area, s);

        // Render Tabs
        if let Some(tabs_area) = my_layout.get(&ComponentId::InspectTabs) {
            let mut tabs_layout = HashMap::new();
            tabs_layout.insert(ComponentId::InspectTabs, *tabs_area);
            self.tabs.render(f, s, &tabs_layout);
        }

        // Render Active Page
        match self.tabs.selected() {
            InspectOption::Ledger => self.ledger_page.render(f, s, &my_layout),
            InspectOption::Chain => self.chain_page.render(f, s, &my_layout),
            InspectOption::Otel => self.otel_page.render(f, s, &my_layout),
            InspectOption::Prometheus => self.prometheus_page.render(f, s, &my_layout),
        }
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
}

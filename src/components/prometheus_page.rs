use crate::{
    app_state::AppState,
    components::{
        Component, ComponentLayout, InputRoute, handle_container_event,
        prom_metrics::PromMetricsComponent, route_event_to_children,
    },
    controller::{LayoutSpec, walk_layout},
    prometheus::model::NodeMetrics,
    states::{Action, ComponentId},
    tui::Event,
};
use either::Either::Left;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use std::{any::Any, collections::HashMap, sync::RwLock};
use tokio::sync::mpsc::Receiver;

pub struct PrometheusPageComponent {
    id: ComponentId,
    pub metrics: PromMetricsComponent,
    last_layout: RwLock<ComponentLayout>,
    active_focus: RwLock<ComponentId>,
}

impl PrometheusPageComponent {
    pub fn new(prom_metrics: Receiver<NodeMetrics>) -> Self {
        Self {
            id: ComponentId::PrometheusPage,
            metrics: PromMetricsComponent::new(ComponentId::PrometheusMetrics, prom_metrics),
            last_layout: RwLock::new(HashMap::new()),
            active_focus: RwLock::new(ComponentId::PrometheusMetrics),
        }
    }
}

impl Component for PrometheusPageComponent {
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
        let spec = LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![(Constraint::Fill(1), Left(ComponentId::PrometheusMetrics))],
        };

        let mut layout = HashMap::new();
        walk_layout(&mut layout, &spec, area);
        layout
    }

    fn route_event(&self, event: &Event, s: &AppState) -> InputRoute {
        let my_area = s
            .layout_model
            .get_layout()
            .get(&self.id)
            .copied()
            .unwrap_or(s.frame_area);

        let my_layout = self.calculate_layout(my_area, s);
        let route = route_event_to_children(event, s, my_layout);
        match route {
            InputRoute::Delegate(ComponentId::PrometheusMetrics, _) => InputRoute::Handle,
            _ => route,
        }
    }

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        let layout = self.last_layout.read().unwrap().clone();
        let mut active_focus = *self.active_focus.read().unwrap();

        let actions = handle_container_event(
            &layout,
            &mut active_focus,
            event,
            area,
            |target_id, ev, child_area| {
                if target_id == ComponentId::PrometheusMetrics {
                    self.metrics.handle_event(ev, child_area)
                } else {
                    Vec::new()
                }
            },
        );

        *self.active_focus.write().unwrap() = active_focus;
        actions
    }

    fn render(&self, f: &mut Frame, s: &AppState, parent_layout: &ComponentLayout) {
        let my_area = parent_layout.get(&self.id).copied().unwrap_or(f.area());
        let my_layout = self.calculate_layout(my_area, s);
        {
            let mut layout_guard = self.last_layout.write().unwrap();
            *layout_guard = my_layout.clone();
        }
        if let Some(_rect) = my_layout.get(&ComponentId::PrometheusMetrics) {
            self.metrics.render(f, s, &my_layout);
        }
    }
}

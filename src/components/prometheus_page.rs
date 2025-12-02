use crate::{
    components::{
        Component, ComponentLayout, handle_container_event, prom_metrics::PromMetricsComponent,
    },
    controller::{LayoutSpec, walk_layout},
    model::layout::{MoveFocus, find_next_focus},
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

    pub fn handle_navigation(&mut self, direction: MoveFocus) -> Vec<Action> {
        let layout = self.last_layout.read().unwrap();
        let active_focus = *self.active_focus.read().unwrap();

        if let Some(next) = find_next_focus(&layout, active_focus, direction) {
            *self.active_focus.write().unwrap() = next;
            return vec![Action::SetFocus(next)];
        }

        Vec::new()
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

    fn calculate_layout(&self, area: Rect) -> ComponentLayout {
        let spec = LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![(Constraint::Fill(1), Left(ComponentId::PrometheusMetrics))],
        };

        let mut layout = HashMap::new();
        walk_layout(&mut layout, &spec, area);
        layout
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

    fn tick(&mut self) -> Vec<Action> {
        self.metrics.sync();
        Vec::new()
    }

    fn render(&self, f: &mut Frame, parent_layout: &ComponentLayout) {
        let my_area = parent_layout.get(&self.id).copied().unwrap_or(f.area());
        let my_layout = self.calculate_layout(my_area);
        {
            let mut layout_guard = self.last_layout.write().unwrap();
            *layout_guard = my_layout.clone();
        }
        if let Some(_rect) = my_layout.get(&ComponentId::PrometheusMetrics) {
            self.metrics.render(f, &my_layout);
        }
    }
}

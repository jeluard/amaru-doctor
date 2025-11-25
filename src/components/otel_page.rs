use crate::{
    app_state::AppState,
    components::{
        Component, ComponentLayout, InputRoute, details::DetailsComponent,
        flame_graph::FlameGraphComponent, route_event_to_children, trace_list::TraceListComponent,
    },
    controller::{LayoutSpec, walk_layout},
    states::{Action, ComponentId},
    tui::Event,
};
use either::Either::{Left, Right};
use opentelemetry_proto::tonic::trace::v1::Span;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use std::{any::Any, collections::HashMap, sync::RwLock};

pub struct OtelPageComponent {
    id: ComponentId,
    pub trace_list: TraceListComponent,
    pub flame_graph: FlameGraphComponent,
    pub span_details: DetailsComponent<Span>,

    last_layout: RwLock<ComponentLayout>,
    active_focus: RwLock<ComponentId>,
}

impl Default for OtelPageComponent {
    fn default() -> Self {
        Self {
            id: ComponentId::OtelPage,
            trace_list: TraceListComponent::new(ComponentId::OtelTraceList),
            flame_graph: FlameGraphComponent::new(ComponentId::OtelFlameGraph),
            span_details: DetailsComponent::new(
                ComponentId::OtelSpanDetails,
                "Span Details",
                Box::new(|s: &AppState| s.otel_view.focused_span.as_deref()),
            ),

            last_layout: RwLock::new(HashMap::new()),
            active_focus: RwLock::new(ComponentId::OtelTraceList),
        }
    }
}

impl Component for OtelPageComponent {
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
            constraints: vec![(
                Constraint::Fill(1),
                Right(LayoutSpec {
                    direction: Direction::Horizontal,
                    constraints: vec![
                        (Constraint::Percentage(10), Left(ComponentId::OtelTraceList)),
                        (
                            Constraint::Percentage(90),
                            Right(LayoutSpec {
                                direction: Direction::Horizontal,
                                constraints: vec![
                                    (
                                        Constraint::Percentage(70),
                                        Left(ComponentId::OtelFlameGraph),
                                    ),
                                    (
                                        Constraint::Percentage(30),
                                        Left(ComponentId::OtelSpanDetails),
                                    ),
                                ],
                            }),
                        ),
                    ],
                }),
            )],
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
            InputRoute::Delegate(
                ComponentId::OtelTraceList
                | ComponentId::OtelFlameGraph
                | ComponentId::OtelSpanDetails,
                _,
            ) => InputRoute::Handle,
            _ => route,
        }
    }

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        let target_id = match event {
            Event::Key(_) => *self.active_focus.read().unwrap(),
            Event::Mouse(mouse) => {
                let layout = self.last_layout.read().unwrap();
                layout
                    .iter()
                    .find(|(_, rect)| {
                        mouse.column >= rect.x
                            && mouse.column < rect.x + rect.width
                            && mouse.row >= rect.y
                            && mouse.row < rect.y + rect.height
                    })
                    .map(|(id, _)| *id)
                    .unwrap_or_else(|| *self.active_focus.read().unwrap())
            }
            _ => return Vec::new(),
        };

        let child_area = {
            let layout = self.last_layout.read().unwrap();
            layout.get(&target_id).copied().unwrap_or(area)
        };

        let mut actions = match target_id {
            ComponentId::OtelTraceList => self.trace_list.handle_event(event, child_area),
            ComponentId::OtelFlameGraph => self.flame_graph.handle_event(event, child_area),
            ComponentId::OtelSpanDetails => self.span_details.handle_event(event, child_area),
            _ => Vec::new(),
        };

        if let Event::Mouse(mouse) = event {
            actions.push(Action::MouseEvent(*mouse));
        }

        actions
    }

    fn render(&self, f: &mut Frame, s: &AppState, parent_layout: &ComponentLayout) {
        let my_area = parent_layout.get(&self.id).copied().unwrap_or(f.area());
        let my_layout = self.calculate_layout(my_area, s);

        {
            let mut layout_guard = self.last_layout.write().unwrap();
            *layout_guard = my_layout.clone();
        }
        {
            let mut focus_guard = self.active_focus.write().unwrap();
            *focus_guard = s.layout_model.get_focus();
        }

        if let Some(_rect) = my_layout.get(&ComponentId::OtelTraceList) {
            self.trace_list.render(f, s, &my_layout);
        }
        if let Some(_rect) = my_layout.get(&ComponentId::OtelFlameGraph) {
            self.flame_graph.render(f, s, &my_layout);
        }
        if let Some(_rect) = my_layout.get(&ComponentId::OtelSpanDetails) {
            self.span_details.render(f, s, &my_layout);
        }
    }
}

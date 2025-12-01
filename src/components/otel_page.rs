use crate::{
    app_state::AppState,
    components::{
        Component, ComponentLayout, InputRoute, details::DetailsComponent,
        flame_graph::FlameGraphComponent, route_event_to_children, trace_list::TraceListComponent,
    },
    controller::{LayoutSpec, walk_layout},
    model::otel_view::OtelViewState,
    otel::{TraceGraphSnapshot, span_ext::SpanExt},
    states::{Action, ComponentId},
    tui::Event,
};
use crossterm::event::MouseEventKind;
use either::Either::{Left, Right};
use opentelemetry_proto::tonic::trace::v1::Span;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use std::{any::Any, collections::HashMap, sync::RwLock};

pub struct OtelPageComponent {
    id: ComponentId,

    pub view_state: OtelViewState,
    pub trace_list: TraceListComponent,
    pub flame_graph: FlameGraphComponent,
    pub span_details: DetailsComponent<Span>,

    last_layout: RwLock<ComponentLayout>,
    active_focus: RwLock<ComponentId>,
}

impl OtelPageComponent {
    pub fn new(trace_graph: TraceGraphSnapshot) -> Self {
        Self {
            id: ComponentId::OtelPage,
            view_state: OtelViewState::new(trace_graph),
            trace_list: TraceListComponent::new(ComponentId::OtelTraceList),
            flame_graph: FlameGraphComponent::new(ComponentId::OtelFlameGraph),
            span_details: DetailsComponent::new(ComponentId::OtelSpanDetails, "Span Details"),

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
        let layout = self.last_layout.read().unwrap().clone();
        let mut active_focus = *self.active_focus.read().unwrap();

        let mut actions = crate::components::handle_container_event(
            &layout,
            &mut active_focus,
            event,
            area,
            |target_id, ev, child_area| {
                match target_id {
                    ComponentId::OtelTraceList => self.trace_list.handle_event(ev, child_area),

                    ComponentId::OtelFlameGraph => {
                        // Run standard handler
                        let mut acts = self.flame_graph.handle_event(ev, child_area);

                        // Calculate Hovered Span
                        // TODO: Move this logic into FlameGraphComponent.
                        if let Event::Mouse(mouse) = ev
                            && mouse.kind == MouseEventKind::Moved
                        {
                            // +1 for border
                            let relative_row = mouse.row.saturating_sub(child_area.y + 1) as usize;

                            let trace_graph = self.view_state.trace_graph.load();

                            let hovered_span_id = if let Some(selected_span) =
                                &self.view_state.selected_span
                            {
                                // Zoomed View
                                let selected_id = selected_span.span_id();
                                let ancestors = trace_graph
                                    .ancestor_iter(selected_id)
                                    .collect::<Vec<_>>()
                                    .into_iter()
                                    .rev();
                                let descendants = trace_graph.descendent_iter(selected_id);
                                ancestors.chain(descendants).nth(relative_row)
                            } else if let Some(selected_trace) = &self.view_state.selected_trace_id
                            {
                                // Full View
                                trace_graph.trace_iter(selected_trace).nth(relative_row)
                            } else {
                                None
                            };

                            // Update State
                            let new_focus = hovered_span_id
                                .and_then(|span_id| trace_graph.spans.get(&span_id).cloned());
                            if self.view_state.focused_span != new_focus {
                                self.view_state.focused_span = new_focus;
                                acts.push(Action::Render);
                            }
                        }
                        acts
                    }

                    ComponentId::OtelSpanDetails => self.span_details.handle_event(ev, child_area),
                    _ => Vec::new(),
                }
            },
        );

        *self.active_focus.write().unwrap() = active_focus;

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
        if let Some(rect) = my_layout.get(&ComponentId::OtelFlameGraph) {
            let is_focused = s.layout_model.is_focused(ComponentId::OtelFlameGraph);
            self.flame_graph
                .render_with_state(f, *rect, &self.view_state, is_focused);
        }
        if let Some(rect) = my_layout.get(&ComponentId::OtelSpanDetails) {
            let is_focused = s.layout_model.is_focused(ComponentId::OtelSpanDetails);
            self.span_details.render_with_data(
                f,
                *rect,
                is_focused,
                self.view_state.focused_span.as_deref(),
            );
        }
    }
}

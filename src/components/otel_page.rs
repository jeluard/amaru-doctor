use crate::{
    app_state::AppState,
    components::{
        Component, ComponentLayout, details::DetailsComponent, flame_graph::FlameGraphComponent,
        handle_container_event, trace_list::TraceListComponent,
    },
    controller::{LayoutSpec, walk_layout},
    model::otel_view::OtelViewState,
    otel::{TraceGraphSnapshot, graph::TraceGraph, id::SpanId, span_ext::SpanExt},
    states::{Action, ComponentId},
    tui::Event,
};
use crossterm::event::{KeyCode, MouseEventKind};
use either::Either::{Left, Right};
use opentelemetry_proto::tonic::trace::v1::Span;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use std::{any::Any, cmp::Reverse, collections::HashMap, sync::RwLock};

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

    fn get_visible_spans(&self, graph: &TraceGraph) -> Vec<SpanId> {
        if let Some(selected_span) = &self.view_state.selected_span {
            let selected_id = selected_span.span_id();
            let mut ancestors: Vec<SpanId> = graph.ancestor_iter(selected_id).collect();
            ancestors.reverse();
            let descendants = graph.descendent_iter(selected_id);
            ancestors.into_iter().chain(descendants).collect()
        } else if let Some(trace_id) = &self.view_state.selected_trace_id {
            graph.trace_iter(trace_id).collect()
        } else {
            Vec::new()
        }
    }

    fn scroll_trace_details(&mut self, direction: i32) {
        let data = self.view_state.trace_graph.load();
        let ordered_spans = self.get_visible_spans(&data);
        if ordered_spans.is_empty() {
            return;
        }

        let current_index = self
            .view_state
            .focused_span
            .as_ref()
            .and_then(|span| ordered_spans.iter().position(|id| *id == span.span_id()));

        let len = ordered_spans.len();
        let current = current_index.unwrap_or(0) as i32;

        // Wrap around math
        let new_index = (current + direction).rem_euclid(len as i32) as usize;

        if Some(new_index) != current_index {
            self.view_state.focused_span = ordered_spans
                .get(new_index)
                .and_then(|id| data.spans.get(id).cloned());
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

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        let layout = self.last_layout.read().unwrap().clone();
        let mut active_focus = *self.active_focus.read().unwrap();

        let mut actions = handle_container_event(
            &layout,
            &mut active_focus,
            event,
            area,
            |target_id, ev, child_area| {
                match target_id {
                    ComponentId::OtelTraceList => self.trace_list.handle_event(ev, child_area),

                    ComponentId::OtelFlameGraph => {
                        let mut acts = self.flame_graph.handle_event(ev, child_area);

                        // Hover Logic
                        if let Event::Mouse(mouse) = ev {
                            if mouse.kind == MouseEventKind::Moved {
                                let relative_row =
                                    mouse.row.saturating_sub(child_area.y + 1) as usize;
                                let trace_graph = self.view_state.trace_graph.load();
                                let visible_spans = self.get_visible_spans(&trace_graph);
                                let hovered_span_id = visible_spans.get(relative_row).copied();
                                let new_focus = hovered_span_id
                                    .and_then(|span_id| trace_graph.spans.get(&span_id).cloned());
                                if self.view_state.focused_span != new_focus {
                                    self.view_state.focused_span = new_focus;
                                    acts.push(Action::Render);
                                }
                            }

                            // Mouse Scroll Logic
                            if mouse.kind == MouseEventKind::ScrollDown {
                                self.scroll_trace_details(1);
                                acts.push(Action::Render);
                            } else if mouse.kind == MouseEventKind::ScrollUp {
                                self.scroll_trace_details(-1);
                                acts.push(Action::Render);
                            }
                        }

                        // Keyboard Scroll Logic (Up/Down)
                        if let Event::Key(key) = ev {
                            match key.code {
                                KeyCode::Down => {
                                    self.scroll_trace_details(1);
                                    acts.push(Action::Render);
                                }
                                KeyCode::Up => {
                                    self.scroll_trace_details(-1);
                                    acts.push(Action::Render);
                                }
                                _ => {}
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

    fn tick(&mut self) -> Vec<Action> {
        // Get the currently selected trace from the UI list
        let selected_trace = self.trace_list.selected_item().copied();

        // Sync the ViewState (Data) with the UI selection
        let changed = self.view_state.sync_state(selected_trace.as_ref());

        if !changed {
            return Vec::new();
        }

        // If data changed, update the UI List
        let data = self.view_state.trace_graph.load();
        let mut trace_ids: Vec<_> = data.traces.keys().copied().collect();
        trace_ids.sort_unstable_by_key(|id| Reverse(data.traces.get(id).unwrap().start_time()));

        self.trace_list.sync_state(trace_ids);

        Vec::new()
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

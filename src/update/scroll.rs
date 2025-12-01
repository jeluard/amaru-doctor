use crate::{
    app_state::AppState,
    components::{Component, root::RootComponent},
    model::otel_view::OtelViewState,
    otel::{graph::TraceGraph, id::SpanId, span_ext::SpanExt},
    states::{Action, ComponentId, InspectOption},
    update::Update,
};
use strum::Display;
use tracing::debug;

#[derive(Display, Debug, Clone, Copy)]
pub enum ScrollDirection {
    Up,
    Down,
}

pub struct ScrollUpdate;
impl Update for ScrollUpdate {
    fn update(&self, action: &Action, s: &mut AppState, root: &mut RootComponent) -> Vec<Action> {
        let Some(direction) = (match action {
            Action::ScrollUp => Some(ScrollDirection::Up),
            Action::ScrollDown => Some(ScrollDirection::Down),
            _ => None,
        }) else {
            return Vec::new();
        };

        let focus_id = s.layout_model.get_focus();
        debug!(
            "ScrollUpdate: Handling scroll direction: {:?}, current focus: {:?}",
            direction, focus_id
        );

        match focus_id {
            ComponentId::OtelTraceList => {
                debug!("ScrollUpdate: Dispatching scroll to OtelTraceList with side effects");
                root.otel_page.trace_list.handle_scroll(direction);
                let selected_trace = root.otel_page.trace_list.selected_item().copied();
                root.otel_page.view_state.select_trace(selected_trace);
            }

            ComponentId::OtelFlameGraph => {
                debug!("ScrollUpdate: Dispatching scroll to scroll_trace_details");
                scroll_trace_details(&mut root.otel_page.view_state, direction);
            }

            _ => {
                // TODO: Transitional: Manual dispatch to active page.
                let active_page = match root.tabs.selected() {
                    InspectOption::Ledger => &mut root.ledger_page as &mut dyn Component,
                    InspectOption::Chain => &mut root.chain_page as &mut dyn Component,
                    InspectOption::Otel => &mut root.otel_page as &mut dyn Component,
                    InspectOption::Prometheus => &mut root.prometheus_page as &mut dyn Component,
                };

                let mut actions = active_page.handle_scroll(direction);
                if matches!(
                    focus_id,
                    ComponentId::LedgerBrowseOptions | ComponentId::LedgerSearchOptions
                ) {
                    actions.push(Action::UpdateLayout(s.frame_area));
                }
                return actions;
            }
        }

        Vec::new()
    }
}

/// Scrolls to the next focused span within the OTEL trace details view.
fn scroll_trace_details(view_state: &mut OtelViewState, direction: ScrollDirection) {
    let data = view_state.trace_graph.load();
    let Some(ordered_spans) = get_ordered_spans_for_view(&data, view_state) else {
        return;
    };
    if ordered_spans.is_empty() {
        return;
    }

    // Find the index of the currently focused span in the span list
    let current_index = view_state
        .focused_span
        .as_ref()
        .and_then(|span| ordered_spans.iter().position(|id| *id == span.span_id()));

    let len = ordered_spans.len();

    let new_index = match direction {
        // Allow wrapping
        ScrollDirection::Down => current_index.map_or(0, |i| (i + 1) % len),
        ScrollDirection::Up => current_index.map_or(len - 1, |i| (i + len - 1) % len),
    };

    if Some(new_index) != current_index {
        // Update the focused span given the new index
        view_state.focused_span = ordered_spans
            .get(new_index)
            .and_then(|id| data.spans.get(id).cloned());
    }
}

/// Helper to get the list of spans for scrolling.
fn get_ordered_spans_for_view(
    data: &TraceGraph,
    view_state: &OtelViewState,
) -> Option<Vec<SpanId>> {
    // Determine if a span is selected
    if let Some(selected_span) = &view_state.selected_span {
        let selected_span_id = selected_span.span_id();
        // Get the span's ancestors. The iter starts at the span's parent and walks *up*
        // the tree--we reverse this so that the resulting list is in ascending order.
        let mut ancestors: Vec<SpanId> = data.ancestor_iter(selected_span_id).collect();
        ancestors.reverse();
        let self_and_descendants = data.descendent_iter(selected_span_id);
        Some(ancestors.into_iter().chain(self_and_descendants).collect())
    } else {
        // There's no selected span, render the selected trace's entire tree
        view_state
            .selected_trace_id
            .as_ref()
            .map(|trace_id| data.trace_iter(trace_id).collect())
    }
}

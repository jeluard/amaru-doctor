use crate::{
    app_state::AppState,
    components::{Component, root::RootComponent},
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

                // TODO: Transitional: Scroll via Root. Move this logic into OtelPageComponent::handle_event.
                root.otel_page.trace_list.handle_scroll(direction);

                // TODO: Transitional: Sync side-effects manually. Move to OtelPageComponent or a dedicated event handler.
                let graph = s.otel_view.trace_graph_source.load();
                let selected_item = root.otel_page.trace_list.selected_item();

                let new_focused_span = selected_item
                    .and_then(|trace_id| graph.traces.get(trace_id))
                    .and_then(|trace_meta| trace_meta.roots().first_key_value())
                    .and_then(|(_, root_ids)| root_ids.first())
                    .and_then(|root_id| graph.spans.get(root_id))
                    .cloned();

                s.otel_view.focused_span = new_focused_span;
                s.otel_view.selected_span = None;
                s.otel_view.selected_trace_id = selected_item.cloned();
            }

            ComponentId::OtelFlameGraph => {
                debug!("ScrollUpdate: Dispatching scroll to scroll_trace_details");
                // TODO: Transitional: Logic lives in helper fn. Move to FlameGraphComponent or OtelPageComponent.
                scroll_trace_details(s, direction);
            }

            _ => {
                // TODO: Transitional: Manual dispatch to active page.
                // Future: Remove ScrollUpdate entirely; Components should handle Action::ScrollUp/Down in handle_event.
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
fn scroll_trace_details(s: &mut AppState, direction: ScrollDirection) {
    let data = s.otel_view.trace_graph_source.load();
    let Some(ordered_spans) = get_ordered_spans_for_view(&data, s) else {
        return;
    };
    if ordered_spans.is_empty() {
        return;
    }

    // Find the index of the currently focused span in the span list
    let current_index = s
        .otel_view
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
        s.otel_view.focused_span = ordered_spans
            .get(new_index)
            .and_then(|id| data.spans.get(id).cloned());
    }
}

/// Helper to get the list of spans for scrolling. If a span is selected, we
/// only get that span's subtree. If a span isn't selected, we get all the spans
/// for the selected trace.
fn get_ordered_spans_for_view(data: &TraceGraph, s: &AppState) -> Option<Vec<SpanId>> {
    // Determine if a span is selected
    if let Some(selected_span) = &s.otel_view.selected_span {
        let selected_span_id = selected_span.span_id();
        // Get the span's ancestors. The iter starts at the span's parent and walks *up*
        // the tree--we reverse this so that the resulting list is in ascending order.
        let mut ancestors: Vec<SpanId> = data.ancestor_iter(selected_span_id).collect();
        ancestors.reverse();
        let self_and_descendants = data.descendent_iter(selected_span_id);
        Some(ancestors.into_iter().chain(self_and_descendants).collect())
    } else {
        // There's no selected span, render the selected trace's entire tree
        s.otel_view
            .selected_trace_id
            .as_ref()
            .map(|trace_id| data.trace_iter(trace_id).collect())
    }
}

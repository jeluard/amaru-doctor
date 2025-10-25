use crate::{
    model::dynamic_list::DynamicList,
    otel::{
        graph::TraceGraph,
        id::{SpanId, TraceId},
        span_ext::SpanExt,
    },
};
use arc_swap::ArcSwap;
use opentelemetry_proto::tonic::trace::v1::Span;
use std::{cmp::Reverse, sync::Arc};
use tracing::debug;

/// Manages the rendering state for the OTEL tab of the TUI.
///
/// This struct acts as a snapshot of the UI state, which is sync'd with a
/// shared, concurrently-updated data source (`trace_graph_source`).
///
/// The state includes the list of all traces, the currently selected trace, and
/// the states for hovered and selected spans within that trace's span tree.
#[derive(Debug)]
pub struct OtelViewState {
    /// A thread-safe, swappable reference to the authoritative `TraceGraph`.
    /// The view state will periodically sync its internal state from this
    /// source.
    pub trace_graph_source: Arc<ArcSwap<TraceGraph>>,
    /// A pointer to the last `TraceGraph` instance this state was synced
    /// against. This is used for efficient change detection via
    /// `Arc::ptr_eq`.
    pub last_synced_data: Option<Arc<TraceGraph>>,
    /// The stateful list of all trace IDs, sorted for display. This component
    /// manages selection and scrolling within the TUI's trace list view.
    pub trace_list: DynamicList<TraceId>,
    /// The span currently being hovered over in the TUI. This is used for
    /// showing span details.
    pub focused_span: Option<Arc<Span>>,
    /// The span that the user has actively selected. This is used to inspect a
    /// span's specific subtree.
    pub selected_span: Option<Arc<Span>>,
}

impl OtelViewState {
    pub fn new(trace_graph_source: Arc<ArcSwap<TraceGraph>>) -> Self {
        Self {
            trace_graph_source,
            last_synced_data: None,
            trace_list: DynamicList::default(),
            focused_span: None,
            selected_span: None,
        }
    }

    /// Syncs the view state with the latest data from the shared source.
    ///
    /// This method checks if the underlying `TraceGraph` has changed. If it
    /// has, it updates the trace list and validates the selected trace and
    /// spans.
    pub fn sync_state(&mut self) -> bool {
        // Atomically load the most recent `Arc<TraceGraph>` from the shared `ArcSwap`.
        // This is a lock-free operation. `latest_data` is now a snapshot of the data
        // that this sync operation will be based on.
        let latest_data = self.trace_graph_source.load_full();

        // Determine if the data has changed since the last sync
        let has_changed = match &self.last_synced_data {
            // Compare pointers. If the underlying data hasn't changed, the pointers
            // will be equal.
            Some(prev) => !Arc::ptr_eq(prev, &latest_data),
            None => true,
        };

        if !has_changed {
            return false;
        }
        debug!("TraceGraph changes found, updating");

        // Get the latest trace ids, sort them by start, and set them in the trace list
        // for display
        let mut trace_ids: Vec<_> = latest_data.traces.keys().copied().collect();
        trace_ids
            .sort_unstable_by_key(|id| Reverse(latest_data.traces.get(id).unwrap().start_time()));
        self.trace_list.set_items(trace_ids);

        if self.trace_list.selected_item().is_none() {
            // Trace selection was lost (because the trace was evicted), clear the
            // dependent span states
            self.focused_span = None;
            self.selected_span = None;
        } else {
            // Trace selection remains, validate the span states
            self.validate_span(&latest_data, |s| &mut s.focused_span);
            self.validate_span(&latest_data, |s| &mut s.selected_span);
        }

        self.last_synced_data = Some(latest_data);
        true
    }

    /// Helper to validate a span field against the latest data.
    fn validate_span<F>(&mut self, data: &TraceGraph, mut field_accessor: F)
    where
        F: FnMut(&mut Self) -> &mut Option<Arc<Span>>,
    {
        // Take the span from the field to check it
        if let Some(span) = field_accessor(self).take() {
            // Check if the currently selected trace contains the span
            if self
                .trace_list
                .selected_item()
                .is_some_and(|trace_id| is_span_in_trace(data, trace_id, &span.span_id()))
            {
                // It's still valid (in the trace), put it back
                *field_accessor(self) = Some(span);
            }
        }
    }
}

/// Checks if a span_id belongs to a given trace_id within the data snapshot.
fn is_span_in_trace(data: &TraceGraph, trace_id: &TraceId, span_id: &SpanId) -> bool {
    data.spans
        .get(span_id)
        .is_some_and(|span_arc| &span_arc.trace_id() == trace_id)
}

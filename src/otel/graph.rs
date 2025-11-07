use crate::otel::ancestor_iter::AncestorIter;
use crate::otel::id::{SpanId, TraceId};
use crate::otel::span_ext::SpanExt;
use crate::otel::trace_iter::TraceIter;
use crate::otel::{SubTree, TraceMeta};
use opentelemetry_proto::tonic::trace::v1::Span;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::SystemTime;
use tracing::{debug, error};

/// A helper struct for updating the Evictor when a new Root is added to the
/// graph.
pub struct TraceInfo {
    pub trace_id: TraceId,
    pub old_trace_start: Option<SystemTime>,
    pub new_trace_start: SystemTime,
}

/// Holds the state of processed traces, nodes, and spans.
#[derive(Clone, Debug, Default)]
pub struct TraceGraph {
    /// A 1-1 map of SpanId to Span
    pub spans: HashMap<SpanId, Arc<Span>>,

    /// A 1-1 map of a SpanId to SubTree. A SubTree holds the known start and
    /// end time for this span including all its children.
    pub subtrees: HashMap<SpanId, Arc<SubTree>>,

    /// A 1-1 map of TraceId to TraceMeta. TraceMeta holds the known RootIds for
    /// the Trace.
    pub traces: HashMap<TraceId, Arc<TraceMeta>>,
}

impl TraceGraph {
    /// Inserts a new root span for its Trace in the TraceGraph.
    /// Returns a struct describing the effect on the Trace's start time.
    pub fn insert_root_span(&mut self, span: Span) -> TraceInfo {
        let new_root_id = span.span_id();
        let trace_id = span.trace_id();
        let new_root_start = span.start_time();
        let new_root_end = span.end_time();

        // Get a mutable reference to the Arc<TraceMeta> for this Trace.
        let trace_meta_arc = self.traces.entry(trace_id).or_default();
        // Allow mutating the contents of the Arc<TraceMeta>.
        let trace_meta = Arc::make_mut(trace_meta_arc);

        let old_trace_start = trace_meta.start_time();
        // Add the new root span.
        trace_meta
            .roots
            .entry(new_root_start)
            .or_default()
            .push(new_root_id);

        debug!("Modified trace_meta: {:?}", trace_meta);

        // Insert the full span and SubTree data into the main HashMaps.
        self.subtrees
            .insert(new_root_id, SubTree::new(new_root_start, new_root_end));
        self.spans.insert(new_root_id, Arc::new(span));

        // Return the info. We can safely unwrap the new start time because
        // we know we just added a root, so the map inside TraceMeta can't be empty.
        TraceInfo {
            trace_id,
            old_trace_start,
            new_trace_start: trace_meta.start_time().unwrap(),
        }
    }

    /// Inserts a new span that is a child of an existing span.
    pub fn insert_child_span(&mut self, span: Span) {
        let span_id = span.span_id();
        let start_time = span.start_time();
        let end_time = span.end_time();
        let parent_id = span.parent_id().unwrap(); // We assume this exists.

        // Get the parent's SubTree and add the child. We will update bounds later.
        if let Some(parent_node_arc) = self.subtrees.get_mut(&parent_id) {
            let sub_tree = Arc::make_mut(parent_node_arc);
            sub_tree
                .children
                .entry(start_time)
                .or_default()
                .push(span_id);
        } else {
            error!("Unexpected: no parent {} for child {}", parent_id, span_id);
        }

        // Add the span to subtrees
        self.subtrees
            .insert(span_id, SubTree::new(start_time, end_time));
        // Add the span's details
        self.spans.insert(span_id, Arc::new(span));
        // Update subtree bounds with this newly added, potentially later end time
        self.propagate_bounds_update(span_id, end_time);
    }

    /// When a child's end time is later than its parent's, this walks up the
    /// tree to ensure all ancestor bounds are updated with the new end time
    pub fn propagate_bounds_update(&mut self, start_span_id: SpanId, new_end_time: SystemTime) {
        let ancestor_ids: Vec<_> = AncestorIter::new(self, start_span_id).collect();
        for span_id in ancestor_ids {
            if let Some(subtree_arc) = self.subtrees.get_mut(&span_id) {
                let subtree = Arc::make_mut(subtree_arc);
                if new_end_time > subtree.bounds.end {
                    // The new end time is later than the current bounds,
                    // update the bounds and continue walking
                    subtree.bounds.end = new_end_time;
                } else {
                    // The parent's bounds is already later, we can stop walking
                    break;
                }
            } else {
                error!("Unexpected: no span {}", span_id);
                break;
            }
        }
    }

    /// Removes a trace, it's children, and other associated state from the
    /// graph.
    pub fn remove_trace(&mut self, trace_id: &TraceId) -> Vec<SpanId> {
        let ids_to_remove: Vec<SpanId> = self.trace_iter(trace_id).collect();

        if ids_to_remove.is_empty() {
            if self.traces.remove(trace_id).is_none() {
                error!("Unexpected: no trace meta {}", trace_id);
            }
            return Vec::new();
        }

        // Remove the entries from the maps.
        for span_id in &ids_to_remove {
            self.subtrees.remove(span_id);
            self.spans.remove(span_id);
        }

        // Remove the top-level trace metadata.
        self.traces.remove(trace_id);

        // Return the list of removed SpandIds.
        ids_to_remove
    }

    /// Returns an iter for a Trace, walking down each of its Roots in series.
    pub fn trace_iter(&self, trace_id: &TraceId) -> TraceIter<'_> {
        let mut to_visit = VecDeque::new();

        // Find the trace's roots to seed the traversal.
        if let Some(trace_meta) = self.traces.get(trace_id) {
            // Retrieve the root spans in order of youngest to oldest
            for root_ids_vec in trace_meta.roots().values().rev() {
                // Recall: this is a SystemTime -> Vec entry in the unlikely
                // case two roots have the same start
                for root_id in root_ids_vec.iter() {
                    to_visit.push_front(*root_id);
                }
            }
        }

        TraceIter::new(self, to_visit)
    }

    /// An iter for the ancestors of the specified Span in the current Graph.
    pub fn ancestor_iter(&self, start_span_id: SpanId) -> AncestorIter<'_> {
        AncestorIter::new(self, start_span_id)
    }

    /// An iter for the descendents of the specified Span in the current Graph.
    pub fn descendent_iter(&self, start_span_id: SpanId) -> TraceIter<'_> {
        let mut to_visit = VecDeque::new();
        to_visit.push_front(start_span_id);
        TraceIter::new(self, to_visit)
    }
}

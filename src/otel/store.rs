use crate::otel::{
    evictor::Evictor, graph::TraceGraph, id::TraceId, orphanage::Orphanage, span_ext::SpanExt,
};
use opentelemetry_proto::tonic::trace::v1::Span;
use std::time::Duration;

/// A high-level orchestrator for storing, managing, and expiring trace data.
/// It holds a TraceGraph, an Orphanage, and an EvictionManager for this logic.
pub struct TraceStore {
    graph: TraceGraph,
    orphanage: Orphanage,
    evictor: Evictor,
}

impl TraceStore {
    pub fn new(expire_duration: Duration) -> Self {
        Self {
            graph: TraceGraph::default(),
            orphanage: Orphanage::default(),
            evictor: Evictor::new(expire_duration),
        }
    }

    /// Provides read-only access to the underlying trace graph data.
    pub fn graph(&self) -> &TraceGraph {
        &self.graph
    }

    /// Adds a batch of spans to the store, connecting them to the trace graph
    /// or placing them in the orphanage if their parents have not yet arrived.
    pub fn add_spans(&mut self, spans: Vec<Span>) {
        for span in spans {
            self.add_span_recursive(span);
        }
    }

    /// The core recursive logic for adding a single span and resolving its
    /// orphans.
    fn add_span_recursive(&mut self, span: Span) {
        let span_id = span.span_id();

        // Determine if the span is a root or a child.
        if let Some(parent_id) = span.parent_id() {
            // It has a parent_id so it's a child span.
            // Check if its parent exists in the graph.
            if self.graph.subtrees.contains_key(&parent_id) {
                self.graph.insert_child_span(span);
            } else {
                // Parent not found, this span is an orphan for now.
                self.orphanage.add(parent_id, span);
            }
        } else {
            // It doesn't have a parent_id so it's a root span.
            // Insert it and update the evictor.
            let root_info = self.graph.insert_root_span(span);
            self.evictor.update_trace_lifetime(root_info);
        }

        // After adding the span, check if it was a parent to any waiting orphans.
        if let Some(orphans) = self.orphanage.remove(&span_id) {
            // If so, recursively add the newly un-orphaned children.
            for orphan in orphans {
                // debug!("Will add orphan to the TraceGraph: {:?}", orphan);
                self.add_span_recursive(orphan);
            }
        }
    }

    /// Evicts all expired traces from the graph and expired orphans.
    /// Returns the list of TraceIds if any traces were evicted from the main
    /// graph.
    pub fn evict_expired(&mut self) -> Vec<TraceId> {
        self.evictor.evict(&mut self.graph, &mut self.orphanage)
    }
}

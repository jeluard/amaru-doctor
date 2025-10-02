use crate::otel::graph::TraceGraph;
use crate::otel::id::SpanId;
use crate::otel::span_ext::SpanExt;
use opentelemetry_proto::tonic::trace::v1::Span;
use std::collections::HashMap;
use std::sync::Arc;

/// An iterator that walks *up* the span's ancestor trace starting with its
/// parent (unlike TraceIter).
pub struct AncestorIter<'a> {
    spans: &'a HashMap<SpanId, Arc<Span>>,
    current_id: Option<SpanId>,
}

impl<'a> AncestorIter<'a> {
    pub fn new(graph: &'a TraceGraph, start_span_id: SpanId) -> Self {
        Self {
            spans: &graph.spans,
            current_id: graph.spans.get(&start_span_id).and_then(|s| s.parent_id()),
        }
    }
}

impl<'a> Iterator for AncestorIter<'a> {
    type Item = SpanId;

    fn next(&mut self) -> Option<Self::Item> {
        // Take the current ID to return it.
        let id_to_return = self.current_id.take()?;

        // Find the parent to set up the *next* iteration.
        self.current_id = self
            .spans
            .get(&id_to_return)
            .and_then(|span| span.parent_id());

        Some(id_to_return)
    }
}

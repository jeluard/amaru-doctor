use crate::otel::SpanId;
use crate::otel::span_ext::SpanExt;
use opentelemetry_proto::tonic::trace::v1::Span;
use std::collections::HashMap;
use std::time::SystemTime;

/// Manages spans that have arrived before their parent nodes.
#[derive(Debug, Default)]
pub struct Orphanage {
    /// A 1-Many map of parent_ids to orphan Spans
    parent_to_orphans: HashMap<SpanId, Vec<Span>>,
}

impl Orphanage {
    /// Adds a span that is waiting for its parent to arrive.
    pub fn add(&mut self, parent_id: SpanId, orphan_span: Span) {
        self.parent_to_orphans
            .entry(parent_id)
            .or_default()
            .push(orphan_span);
    }

    /// Called when a parent is added to the graph; returns its waiting children.
    pub fn remove(&mut self, parent_id: &SpanId) -> Option<Vec<Span>> {
        self.parent_to_orphans.remove(parent_id)
    }

    /// Evicts orphans that are too old to be relevant anymore.
    pub fn evict(&mut self, expire_before: SystemTime) {
        self.parent_to_orphans.retain(|_, orphans| {
            // Retain the orphans not expired
            orphans.retain(|orphan| orphan.start_time() >= expire_before);
            // Retain the entry if it's not empty
            !orphans.is_empty()
        });
    }
}

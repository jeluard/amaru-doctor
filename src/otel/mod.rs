use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::otel::graph::TraceGraph;
use crate::otel::id::{RootId, SpanId};

pub mod ancestor_iter;
pub mod evictor;
pub mod graph;
pub mod id;
pub mod ingestor;
pub mod orphanage;
pub mod processor;
pub mod service;
pub mod span_ext;
pub mod store;
pub mod trace_iter;

/// The start and end times for a trace tree.
#[derive(Copy, Clone, Debug)]
pub struct TreeBounds {
    pub start: SystemTime,
    pub end: SystemTime,
}

impl TreeBounds {
    pub fn start(&self) -> &SystemTime {
        &self.start
    }

    pub fn duration(&self) -> Duration {
        self.end.duration_since(self.start).unwrap_or_default()
    }
}

/// A sub-tree in a trace's tree. Contains the start and end of this sub-tree
/// (bounds) and a sorted map of the span's children, by start their time.
#[derive(Debug, Clone)]
pub struct SubTree {
    bounds: TreeBounds,
    /// A map of start times to the respective SpanIds in this SubTree. We use
    /// Vec<SpanId> in the unlikely case that multiple spans have the same start time.
    children: BTreeMap<SystemTime, Vec<SpanId>>,
}

impl SubTree {
    pub fn new(start: SystemTime, end: SystemTime) -> Arc<Self> {
        Arc::new(Self {
            bounds: TreeBounds { start, end },
            children: BTreeMap::new(),
        })
    }

    pub fn bounds(&self) -> &TreeBounds {
        &self.bounds
    }

    pub fn children(&self) -> &BTreeMap<SystemTime, Vec<SpanId>> {
        &self.children
    }
}

#[derive(Clone, Debug, Default)]
pub struct TraceMeta {
    /// The RootIds for the Trace, sorted by start time. We use Vec<RootId> in the
    /// unlikely case that multiple roots have the same start time.
    roots: BTreeMap<SystemTime, Vec<RootId>>,
}

impl TraceMeta {
    pub fn roots(&self) -> &BTreeMap<SystemTime, Vec<RootId>> {
        &self.roots
    }

    pub fn start_time(&self) -> Option<SystemTime> {
        self.roots.first_key_value().map(|(time, _)| *time)
    }

    // TODO: Consider calc'ing this as Spans are added to the trace and caching it
    pub fn end_time(&self, graph: &TraceGraph) -> Option<SystemTime> {
        let mut max_end = self.start_time()?;

        for root_id in self.roots.values().flatten() {
            if let Some(subtree) = graph.subtrees.get(root_id) {
                max_end = max_end.max(subtree.bounds().end);
            }
        }

        Some(max_end)
    }
}

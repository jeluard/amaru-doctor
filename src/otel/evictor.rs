use crate::otel::graph::{TraceGraph, TraceInfo};
use crate::otel::id::TraceId;
use crate::otel::orphanage::Orphanage;
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};

/// Tracks the age of traces and manages their eviction from the TraceGraph.
#[derive(Debug)]
pub struct Evictor {
    /// 1-Many map of Trace start-times to a list of TraceId. We use Vec in the
    /// unlikely case that multiple Traces start at the same time.
    start_to_trace: BTreeMap<SystemTime, Vec<TraceId>>,

    /// Parameter that determines after how long a Trace should be evicted.
    expire_duration: Duration,
}

impl Evictor {
    pub fn new(expire_duration: Duration) -> Self {
        Self {
            start_to_trace: BTreeMap::new(),
            expire_duration,
        }
    }

    /// Updates the tracking for a given trace for both new traces and existing
    /// traces whose start time has changed.
    pub fn update_trace_lifetime(&mut self, info: TraceInfo) {
        if let Some(old_time) = info.old_trace_start {
            self.untrack(&info.trace_id, &old_time);
        }
        self.track(info.trace_id, info.new_trace_start);
    }

    /// Finds expired traces and removes them from the graph.
    /// Returns the list of evicted trace IDs.
    pub fn evict(&mut self, graph: &mut TraceGraph, orphanage: &mut Orphanage) -> Vec<TraceId> {
        let expire_before = match SystemTime::now().checked_sub(self.expire_duration) {
            Some(time) => time,
            // If time calculation fails, no traces can be expired.
            None => return Vec::new(),
        };

        // Evict orphans that have expired
        orphanage.evict(expire_before);

        // All entries with keys >= expire_before are moved to not_expired_traces.
        // self.start_to_trace is left with only the expired entries.
        let not_expired_traces = self.start_to_trace.split_off(&expire_before);

        // Atomically swap the expired traces out, leaving the live traces in the map.
        let expired_traces = std::mem::replace(&mut self.start_to_trace, not_expired_traces);

        if expired_traces.is_empty() {
            return Vec::new();
        }

        // Collect all expired IDs from the map's values into a single vector.
        let evicted_ids: Vec<TraceId> = expired_traces.into_values().flatten().collect();

        // Remove the expired items from the graph.
        for trace_id in &evicted_ids {
            graph.remove_trace(trace_id);
        }

        // Return the collected list of expired IDs.
        evicted_ids
    }

    /// Starts tracking a trace.
    fn track(&mut self, trace_id: TraceId, start_time: SystemTime) {
        self.start_to_trace
            .entry(start_time)
            .or_default()
            .push(trace_id);
    }

    /// Stops tracking a trace for a time.
    fn untrack(&mut self, trace_id: &TraceId, query_start_time: &SystemTime) {
        self.start_to_trace.retain(|start_time, traces| {
            // Only process the entry for the matching start_time
            if start_time == query_start_time {
                // Remove the target trace_id
                traces.retain(|id| id != trace_id);
            }
            // Keep this entry only if traces isn't empty
            !traces.is_empty()
        });
    }
}

use crate::otel::graph::TraceGraph;
use crate::otel::id::TraceId;
use crate::otel::processor::TraceProcessor;
use crate::otel::span_ext::SpanExt;
use arc_swap::ArcSwap;
use opentelemetry_proto::tonic::trace::v1::Span;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tracing::debug;

/// The TraceIngestor holds
/// 1. the queue to which batch Vecs of spans are sent and
/// 2. a retrievable snapshot of the TraceGraph for rendering.
pub struct TraceIngestor {
    batch_tx: mpsc::Sender<Vec<Span>>,
    snapshot: Arc<ArcSwap<TraceGraph>>,
}

impl TraceIngestor {
    pub fn new(queue_cap: usize, expire_duration: Duration) -> Self {
        let (tx, rx) = mpsc::channel(queue_cap);
        let snapshot = Arc::new(ArcSwap::from_pointee(TraceGraph::default()));

        // Create and spawn the encapsulated processor.
        let processor = TraceProcessor::new(rx, snapshot.clone(), expire_duration);
        tokio::spawn(processor.run());

        Self {
            batch_tx: tx,
            snapshot,
        }
    }

    pub async fn ingest(&self, spans: Vec<Span>) -> Result<(), SendError<Vec<Span>>> {
        let trace_ids: HashSet<TraceId> = spans.iter().map(|s| s.trace_id()).collect();
        debug!("Got trace ids: {:?}", trace_ids);
        self.batch_tx.send(spans).await
    }

    pub fn snapshot(&self) -> Arc<ArcSwap<TraceGraph>> {
        self.snapshot.clone()
    }
}

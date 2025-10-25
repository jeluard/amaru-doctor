use crate::otel::graph::TraceGraph;
use crate::otel::store::TraceStore;
use arc_swap::ArcSwap;
use opentelemetry_proto::tonic::trace::v1::Span;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

/// The background worker that processes span batches on its queue and updates
/// the shared data snapshot.
pub struct TraceProcessor {
    store: TraceStore,
    batch_rx: mpsc::Receiver<Vec<Span>>,
    snapshot: Arc<ArcSwap<TraceGraph>>,
}

impl TraceProcessor {
    pub fn new(
        batch_rx: mpsc::Receiver<Vec<Span>>,
        snapshot: Arc<ArcSwap<TraceGraph>>,
        expire_duration: Duration,
    ) -> Self {
        Self {
            store: TraceStore::new(expire_duration),
            batch_rx,
            snapshot,
        }
    }

    pub async fn run(mut self) {
        // Wait for incoming messages.
        while let Some(spans) = self.batch_rx.recv().await {
            // Process the first batch received.
            self.store.add_spans(spans);

            // Drain any other pending messages in the channel.
            while let Ok(more_spans) = self.batch_rx.try_recv() {
                self.store.add_spans(more_spans);
            }

            // Evict expired spans and update the snapshot.
            self.store.evict_expired();
            let new_snapshot = self.snapshot();
            self.snapshot.store(Arc::new(new_snapshot));
        }
    }

    /// Constructs a new snapshot by performing a clone of trace graph state.
    fn snapshot(&self) -> TraceGraph {
        self.store.graph().clone()
    }
}

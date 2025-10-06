use crate::otel::{TraceGraphSnapshot, ingestor::TraceIngestor, trace_service::AmaruTraceService};
use anyhow::Result;
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceServiceServer;
use std::{net::SocketAddr, time::Duration};
use tokio::task::{self, JoinHandle};
use tonic::transport::Server;

pub struct OtelCollectorService {
    addr: SocketAddr,
}

pub struct OtelCollectorHandle {
    pub snapshot: TraceGraphSnapshot,
    pub task_handle: JoinHandle<Result<()>>,
}

impl OtelCollectorService {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.parse().expect("Invalid address for OTEL service"),
        }
    }

    pub fn start(self) -> OtelCollectorHandle {
        let collector = TraceIngestor::new(10_000, Duration::from_secs(10 * 60));
        let snapshot = collector.snapshot();
        let trace_service = AmaruTraceService::new(collector);
        let task_handle = task::spawn(async move {
            Server::builder()
                .add_service(TraceServiceServer::new(trace_service))
                .serve(self.addr)
                .await?;
            Ok(())
        });

        OtelCollectorHandle {
            snapshot,
            task_handle,
        }
    }
}

use crate::otel::ingestor::TraceIngestor;
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse, trace_service_server::TraceService,
};
use tonic::{Request, Response, Status};

pub struct AmaruTraceService {
    ingestor: TraceIngestor,
}

impl AmaruTraceService {
    pub fn new(collector: TraceIngestor) -> Self {
        Self {
            ingestor: collector,
        }
    }
}

/// The main entry-point for accepting amaru OTEL data.
#[tonic::async_trait]
impl TraceService for AmaruTraceService {
    async fn export(
        &self,
        req: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        // Flatten spans into a single Vec
        let mut all_spans = Vec::new();
        for r_spans in req.into_inner().resource_spans {
            for s_spans in r_spans.scope_spans {
                all_spans.extend(s_spans.spans);
            }
        }

        if !all_spans.is_empty() {
            self.ingestor
                .ingest(all_spans)
                .await
                .map_err(|e| Status::from_error(Box::new(e)))?;
        }

        Ok(Response::new(ExportTraceServiceResponse::default()))
    }
}

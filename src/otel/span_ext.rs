use crate::otel::{SpanId, TraceId};
use opentelemetry_proto::tonic::trace::v1::Span;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// An extension trait for otel's Span.
pub trait SpanExt {
    fn span_id(&self) -> SpanId;
    fn trace_id(&self) -> TraceId;
    fn parent_id(&self) -> Option<SpanId>;
    fn start_time(&self) -> SystemTime;
    fn end_time(&self) -> SystemTime;
    fn duration(&self) -> Duration;
}

impl SpanExt for Span {
    fn span_id(&self) -> SpanId {
        self.span_id.clone().try_into().unwrap_or_default()
    }

    fn trace_id(&self) -> TraceId {
        self.trace_id.clone().try_into().unwrap_or_default()
    }

    fn parent_id(&self) -> Option<SpanId> {
        if self.parent_span_id.is_empty() {
            None
        } else {
            Some(self.parent_span_id.clone().try_into().unwrap_or_default())
        }
    }

    fn start_time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_nanos(self.start_time_unix_nano)
    }

    fn end_time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_nanos(self.end_time_unix_nano)
    }

    fn duration(&self) -> Duration {
        self.end_time()
            .duration_since(self.start_time())
            .unwrap_or_default()
    }
}

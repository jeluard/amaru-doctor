use crate::otel::batch_processor::{BatchProcessor, MetricsBatchProcessor};
use crate::otel::bounded_queue::BoundedQueue;
use crate::otel::rate_limit::RateLimiter;
use opentelemetry_proto::tonic::collector::metrics::v1::{
    ExportMetricsServiceRequest, ExportMetricsServiceResponse,
    metrics_service_server::MetricsService,
};
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse, trace_service_server::TraceService,
};
use opentelemetry_proto::tonic::metrics::v1::{Metric, metric::Data as MetricData};
use opentelemetry_proto::tonic::trace::v1::Span;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::task;
use tonic::{Request, Response, Status};

pub mod batch_processor;
pub mod bounded_queue;
pub mod rate_limit;

#[derive(Clone, Debug)]
pub struct SpanEvent {
    pub trace_id: String,
    pub name: String,
    pub start_unix_nano: u64,
    pub duration_us: u64,
}

#[derive(Clone, Debug)]
pub struct MetricEvent {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub metric_type: String,
    pub value: String,
    pub timestamp: u64,
    pub attributes: Vec<(String, String)>,
}

impl From<Span> for SpanEvent {
    fn from(span: Span) -> SpanEvent {
        SpanEvent {
            trace_id: hex::encode(span.trace_id),
            name: span.name,
            start_unix_nano: span.start_time_unix_nano,
            duration_us: span
                .end_time_unix_nano
                .saturating_sub(span.start_time_unix_nano)
                / 1000,
        }
    }
}

impl From<Metric> for MetricEvent {
    fn from(metric: Metric) -> MetricEvent {
        let (metric_type, value, timestamp) = match metric.data {
            Some(MetricData::Gauge(gauge)) => {
                if let Some(data_point) = gauge.data_points.first() {
                    let value = match &data_point.value {
                        Some(opentelemetry_proto::tonic::metrics::v1::number_data_point::Value::AsDouble(v)) => v.to_string(),
                        Some(opentelemetry_proto::tonic::metrics::v1::number_data_point::Value::AsInt(v)) => v.to_string(),
                        None => "N/A".to_string(),
                    };
                    ("Gauge".to_string(), value, data_point.time_unix_nano)
                } else {
                    ("Gauge".to_string(), "N/A".to_string(), 0)
                }
            }
            Some(MetricData::Sum(sum)) => {
                if let Some(data_point) = sum.data_points.first() {
                    let value = match &data_point.value {
                        Some(opentelemetry_proto::tonic::metrics::v1::number_data_point::Value::AsDouble(v)) => v.to_string(),
                        Some(opentelemetry_proto::tonic::metrics::v1::number_data_point::Value::AsInt(v)) => v.to_string(),
                        None => "N/A".to_string(),
                    };
                    ("Sum".to_string(), value, data_point.time_unix_nano)
                } else {
                    ("Sum".to_string(), "N/A".to_string(), 0)
                }
            }
            Some(MetricData::Histogram(histogram)) => {
                if let Some(data_point) = histogram.data_points.first() {
                    let value = format!(
                        "count: {}, sum: {}",
                        data_point.count,
                        data_point.sum.unwrap_or_default()
                    );
                    ("Histogram".to_string(), value, data_point.time_unix_nano)
                } else {
                    ("Histogram".to_string(), "N/A".to_string(), 0)
                }
            }
            _ => ("Unknown".to_string(), "N/A".to_string(), 0),
        };

        MetricEvent {
            name: metric.name,
            description: metric.description,
            unit: metric.unit,
            metric_type,
            value,
            timestamp,
            attributes: Vec::new(), // TODO: extract attributes from data points
        }
    }
}

pub struct TraceCollector {
    batch_tx: mpsc::Sender<Vec<Span>>,
    batch_rate_limiter: RateLimiter,
    events: Arc<RwLock<BoundedQueue<SpanEvent>>>,
}

impl TraceCollector {
    pub fn new(max_batches_per_sec: usize, queue_capacity: usize) -> Self {
        let (tx, mut rx) = mpsc::channel::<Vec<Span>>(10);
        let events = Arc::new(RwLock::new(BoundedQueue::<SpanEvent>::new(queue_capacity)));
        let events_clone = events.clone();
        let batch_rate_limiter = RateLimiter::new(max_batches_per_sec);

        let mut processor = BatchProcessor::new(queue_capacity);

        task::spawn(async move {
            // Block waiting for a batch
            while let Some(batch) = rx.recv().await {
                processor.push_batch(batch);
                // Drain the queue of batches
                while let Ok(batch) = rx.try_recv() {
                    processor.push_batch(batch);
                    if processor.is_full(queue_capacity) {
                        break;
                    }
                }

                let mut es_q = events_clone.write().unwrap();
                processor.drain_filtered_into(&mut es_q);
                es_q.maybe_shrink();
            }
        });

        Self {
            events,
            batch_tx: tx,
            batch_rate_limiter,
        }
    }

    pub fn submit_spans(&self, spans: Vec<Span>) {
        if self.batch_rate_limiter.allow() && self.batch_tx.try_send(spans).is_err() {
            // Drop span silently
        }
    }

    pub fn snapshot(&self) -> Vec<SpanEvent> {
        let q = self.events.read().expect("event queue lock is poisoned");
        q.iter().take(1000).cloned().collect()
    }
}

pub struct TraceReceiver {
    collector: Arc<TraceCollector>,
}

impl TraceReceiver {
    pub fn new(collector: Arc<TraceCollector>) -> Self {
        Self { collector }
    }
}

#[tonic::async_trait]
impl TraceService for TraceReceiver {
    async fn export(
        &self,
        req: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        let export = req.into_inner();

        for resource_span in export.resource_spans {
            for scope_span in resource_span.scope_spans {
                self.collector.submit_spans(scope_span.spans);
            }
        }

        Ok(Response::new(ExportTraceServiceResponse::default()))
    }
}

pub struct MetricsCollector {
    batch_tx: mpsc::Sender<Vec<Metric>>,
    batch_rate_limiter: RateLimiter,
    events: Arc<RwLock<BoundedQueue<MetricEvent>>>,
}

impl MetricsCollector {
    pub fn new(max_batches_per_sec: usize, queue_capacity: usize) -> Self {
        let (tx, mut rx) = mpsc::channel::<Vec<Metric>>(10);
        let events = Arc::new(RwLock::new(BoundedQueue::<MetricEvent>::new(
            queue_capacity,
        )));
        let events_clone = events.clone();
        let batch_rate_limiter = RateLimiter::new(max_batches_per_sec);

        let mut processor = MetricsBatchProcessor::new(queue_capacity);

        task::spawn(async move {
            // Block waiting for a batch
            while let Some(batch) = rx.recv().await {
                processor.push_batch(batch);
                // Drain the queue of batches
                while let Ok(batch) = rx.try_recv() {
                    processor.push_batch(batch);
                    if processor.is_full(queue_capacity) {
                        break;
                    }
                }

                let mut es_q = events_clone.write().unwrap();
                processor.drain_filtered_into(&mut es_q);
                es_q.maybe_shrink();
            }
        });

        Self {
            events,
            batch_tx: tx,
            batch_rate_limiter,
        }
    }

    pub fn submit_metrics(&self, metrics: Vec<Metric>) {
        if self.batch_rate_limiter.allow() && self.batch_tx.try_send(metrics).is_err() {
            // Drop metrics silently
        }
    }

    pub fn snapshot(&self) -> Vec<MetricEvent> {
        let q = self.events.read().expect("metrics queue lock is poisoned");
        q.iter().take(1000).cloned().collect()
    }
}

pub struct MetricsReceiver {
    collector: Arc<MetricsCollector>,
}

impl MetricsReceiver {
    pub fn new(collector: Arc<MetricsCollector>) -> Self {
        Self { collector }
    }
}

#[tonic::async_trait]
impl MetricsService for MetricsReceiver {
    async fn export(
        &self,
        req: Request<ExportMetricsServiceRequest>,
    ) -> Result<Response<ExportMetricsServiceResponse>, Status> {
        let export = req.into_inner();

        for resource_metric in export.resource_metrics {
            for scope_metric in resource_metric.scope_metrics {
                self.collector.submit_metrics(scope_metric.metrics);
            }
        }

        Ok(Response::new(ExportMetricsServiceResponse::default()))
    }
}

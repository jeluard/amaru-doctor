use crate::metrics::model::{AmaruMetric, MetricUpdate};
use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
};
use bytes::Bytes;
use opentelemetry_proto::tonic::{
    collector::metrics::v1::ExportMetricsServiceRequest,
    metrics::v1::{Metric as OtlpMetric, NumberDataPoint, metric::Data, number_data_point::Value},
};
use prost::Message;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::mpsc::Sender};
use tracing::{error, info, warn};

fn get_value(dp: &NumberDataPoint) -> Option<f64> {
    match dp.value {
        Some(Value::AsDouble(d)) => Some(d),
        Some(Value::AsInt(i)) => Some(i as f64),
        None => None,
    }
}

struct OtlpMetricWrapper<'a>(&'a OtlpMetric);
impl TryFrom<OtlpMetricWrapper<'_>> for MetricUpdate {
    type Error = String;

    fn try_from(wrapper: OtlpMetricWrapper) -> Result<Self, Self::Error> {
        let otlp_metric = wrapper.0;
        let metric = AmaruMetric::try_from((otlp_metric.name.as_str(), otlp_metric.unit.as_str()))?;
        let get_first = |points: &[NumberDataPoint]| {
            if points.len() > 1 {
                warn!(
                    "Metric '{}' has {} data points. Only the first will be recorded.",
                    otlp_metric.name,
                    points.len()
                );
            }
            points.first().and_then(get_value)
        };

        let value_opt = match &otlp_metric.data {
            Some(Data::Gauge(g)) => get_first(&g.data_points),
            Some(Data::Sum(s)) => get_first(&s.data_points),
            Some(other) => {
                return Err(format!(
                    "Unsupported metric type for '{}': {:?}",
                    otlp_metric.name, other
                ));
            }
            None => return Err(format!("Metric '{}' has no data", otlp_metric.name)),
        };

        let value = value_opt
            .ok_or_else(|| format!("Metric '{}' has empty data points", otlp_metric.name))?;
        Ok(MetricUpdate { metric, value })
    }
}

async fn process_metric(tx: &Arc<Sender<MetricUpdate>>, metric: OtlpMetric) {
    match MetricUpdate::try_from(OtlpMetricWrapper(&metric)) {
        Ok(update) => {
            if tx.send(update).await.is_err() {
                warn!("Error sending metric to TUI: channel closed.");
            }
        }
        Err(e) => {
            warn!("Ignored metric: {}", e);
        }
    }
}

async fn handle_metrics(
    State(tx): State<Arc<Sender<MetricUpdate>>>,
    _headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    match ExportMetricsServiceRequest::decode(body.as_ref()) {
        Ok(req) => {
            for resource_metrics in req.resource_metrics {
                for scope_metrics in resource_metrics.scope_metrics {
                    for metric in scope_metrics.metrics {
                        process_metric(&tx, metric).await;
                    }
                }
            }
            StatusCode::OK
        }
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub fn start(tx: Sender<MetricUpdate>) {
    let shared_tx = Arc::new(tx);

    tokio::spawn(async move {
        let app = Router::new()
            .route("/v1/metrics", post(handle_metrics))
            .with_state(shared_tx);
        let addr = SocketAddr::from(([0, 0, 0, 0], 4318));
        info!("Listening on {}", addr);

        match axum::serve(TcpListener::bind(addr).await.unwrap(), app).await {
            Ok(_) => info!("Server exited normally."),
            Err(e) => error!("Server exited with error: {}", e),
        }
    });
}

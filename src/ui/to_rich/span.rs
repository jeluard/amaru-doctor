use crate::{
    otel::span_ext::SpanExt,
    ui::{
        RichText, ToRichText, format_duration, labeled_default, labeled_default_opt_single,
        labeled_default_single,
    },
};
use chrono::{TimeZone, Utc};
use opentelemetry_proto::tonic::{
    common::v1::{AnyValue, KeyValue, any_value::Value},
    trace::v1::{Span, span::Event},
};
use ratatui::text::Line;

struct Attributes<'a>(&'a [KeyValue]);
struct Events<'a>(&'a [Event]);

impl ToRichText for Span {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();

        lines.extend(labeled_default_single("Name", &self.name));
        lines.extend(labeled_default_single("Span Id", self.span_id()));
        lines.extend(labeled_default_single("Trace Id", self.trace_id()));
        lines.extend(labeled_default_opt_single("Parent Id", self.parent_id()));
        lines.extend(labeled_default_single("Kind", format_span_kind(self.kind)));
        lines.extend(labeled_default_opt_single(
            "Status",
            self.status
                .as_ref()
                .map(|s| format!("{} - {}", format_status_code(s.code), s.message)),
        ));
        let start_time = Utc.timestamp_nanos(self.start_time_unix_nano as i64);
        lines.extend(labeled_default_single(
            "Start",
            start_time.format("%H:%M:%S%.6f").to_string(),
        ));
        lines.extend(labeled_default_single(
            "Duration",
            format_duration(self.duration()),
        ));

        if !self.attributes.is_empty() {
            lines.extend(labeled_default("Attributes", &Attributes(&self.attributes)));
        }

        if !self.events.is_empty() {
            lines.extend(labeled_default("Events", &Events(&self.events)));
        }

        lines.into()
    }
}

impl<'a> ToRichText for Attributes<'a> {
    fn to_rich_text(&self) -> RichText {
        self.0
            .iter()
            .map(|attr| {
                Line::from(format!(
                    "  - {}: {}",
                    attr.key,
                    format_any_value(&attr.value)
                ))
            })
            .collect()
    }
}

impl<'a> ToRichText for Events<'a> {
    fn to_rich_text(&self) -> RichText {
        self.0
            .iter()
            .flat_map(|event| event.to_rich_text().unwrap_lines())
            .collect()
    }
}

impl ToRichText for Event {
    fn to_rich_text(&self) -> RichText {
        let mut lines = Vec::new();
        let event_time = Utc.timestamp_nanos(self.time_unix_nano as i64);
        lines.push(Line::from(format!(
            "  - {} @ {}",
            self.name,
            event_time.format("%H:%M:%S%.6f")
        )));
        lines.extend(labeled_default("Attributes", &Attributes(&self.attributes)));
        lines.into()
    }
}

fn format_any_value(value: &Option<AnyValue>) -> String {
    let Some(any_value) = value else {
        return String::from("<None>");
    };
    let Some(v) = &any_value.value else {
        return String::from("<None>");
    };

    match v {
        Value::StringValue(s) => s.clone(),
        Value::BoolValue(b) => b.to_string(),
        Value::IntValue(i) => i.to_string(),
        Value::DoubleValue(d) => d.to_string(),
        // TODO: Impl these recursively ?
        Value::ArrayValue(a) => format!("[{} items]", a.values.len()),
        Value::KvlistValue(k) => format!("{{{} items}}", k.values.len()),
        Value::BytesValue(b) => format!("[{} bytes]", b.len()),
    }
}

/// Format's the SpanKind to be human readable. See otel's protobuf spec, it's in
/// `trace.proto`.
fn format_span_kind(kind: i32) -> &'static str {
    match kind {
        0 => "Unspecified",
        1 => "Internal",
        2 => "Server",
        3 => "Client",
        4 => "Producer",
        5 => "Consumer",
        _ => "Unknown", // Not value in the spec
    }
}

/// Format's the StatusCode to be human readable. See otel's protobuf spec, it's in
/// `trace.proto`.
fn format_status_code(code: i32) -> &'static str {
    match code {
        0 => "Unset",
        1 => "Ok",
        2 => "Error",
        _ => "Unknown", // Not value in the spec
    }
}

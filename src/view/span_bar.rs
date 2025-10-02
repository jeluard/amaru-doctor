use crate::{
    model::otel_view::OtelViewState,
    otel::{TreeBounds, graph::TraceGraph, id::SpanId, span_ext::SpanExt},
    ui::format_duration,
};
use anyhow::{Result, anyhow};
use opentelemetry_proto::tonic::trace::v1::Span as OtelSpan;
use ratatui::prelude::*;
use std::{sync::Arc, time::Duration};

/// Holds the context needed to render a Span bar
pub struct SpanBarRenderer<'a> {
    graph: &'a TraceGraph,
    state: &'a OtelViewState,
    tree_bounds: &'a TreeBounds,
    max_render_width: usize,
    scale: f64,
}

impl<'a> SpanBarRenderer<'a> {
    /// Creates a new SpanBarRenderer.
    pub fn new(
        graph: &'a TraceGraph,
        state: &'a OtelViewState,
        tree_bounds: &'a TreeBounds,
        max_render_width: usize,
    ) -> Result<Self> {
        let tree_duration = tree_bounds.duration();
        if tree_duration.is_zero() {
            return Err(anyhow!(
                "Illegal: Cannot create renderer with a 0-duration TreeBounds"
            ));
        }
        Ok(Self {
            graph,
            state,
            tree_bounds,
            max_render_width,
            scale: (max_render_width as f64) / tree_duration.as_micros() as f64,
        })
    }

    /// Renders a single span bar.
    pub fn render(&self, span_id: &SpanId, is_ancestor: bool) -> Result<Line<'static>> {
        let span = self
            .graph
            .spans
            .get(span_id)
            .ok_or_else(|| anyhow!("Unexpected: span {} not found", span_id))?;

        let (bar_offset, bar_width) = if is_ancestor {
            // Ancestors take up the full width
            (0, self.max_render_width)
        } else {
            let (offset, num_chars_uncapped) = get_span_layout(span, self.tree_bounds, self.scale)?;
            // The available width is what's left of the screen after the offset
            let available_width = self.max_render_width.saturating_sub(offset);
            // Cap the bar width by the available space
            let final_width = num_chars_uncapped.min(available_width);
            (offset, final_width)
        };
        if bar_width == 0 {
            return Err(anyhow!("Illegal: got 0 bar-width for span {}", span_id));
        }

        let bar_text = get_bar_text(span, bar_width)?;
        let is_focused = self
            .state
            .focused_span
            .as_ref()
            .is_some_and(|s| s.span_id() == *span_id);

        let bar_style = get_bar_style(
            is_focused,
            is_ancestor,
            span.duration(),
            self.tree_bounds.duration(),
        )?;

        Ok(Line::from(vec![
            Span::raw(" ".repeat(bar_offset)),
            Span::styled(bar_text, bar_style),
        ]))
    }
}

/// Calculates the screen x-offset and character width of a span bar.
fn get_span_layout(
    span: &OtelSpan,
    tree_bounds: &TreeBounds,
    scale: f64,
) -> Result<(usize, usize)> {
    let span_start_offset = span
        .start_time()
        .duration_since(*tree_bounds.start())
        .map_err(|e| anyhow!("Illegal: Span start time is before tree bounds start time: {e}"))?;

    let offset_chars = (span_start_offset.as_micros() as f64 * scale).floor() as usize;
    let num_chars = (span.duration().as_micros() as f64 * scale).ceil().max(1.0) as usize;

    Ok((offset_chars, num_chars))
}

/// Gets the style for a span bar based on its state.
fn get_bar_style(
    is_focused: bool,
    is_ancestor: bool,
    span_duration: Duration,
    tree_duration: Duration,
) -> Result<Style> {
    if is_focused {
        return Ok(Style::default().fg(Color::Black).bg(Color::LightCyan));
    }
    if is_ancestor {
        return Ok(Style::default().bg(Color::DarkGray).fg(Color::Gray));
    }
    let bg_color = get_bar_color(tree_duration, span_duration)?;
    Ok(Style::default().fg(Color::White).bg(bg_color))
}

/// Gets the color for the span bar based on its duration and the total trace duration.
fn get_bar_color(max_duration: Duration, duration: Duration) -> Result<Color> {
    if max_duration.is_zero() {
        return Err(anyhow!("Illegal: got a 0 max duration for coloring"));
    }
    let ratio = (duration.as_micros() as f64 / max_duration.as_micros() as f64).sqrt();
    let hot = (210, 100, 80); // Reddish-orange
    let cold = (80, 120, 180); // Blueish
    let r = (cold.0 as f64 * (1.0 - ratio) + hot.0 as f64 * ratio) as u8;
    let g = (cold.1 as f64 * (1.0 - ratio) + hot.1 as f64 * ratio) as u8;
    let b = (cold.2 as f64 * (1.0 - ratio) + hot.2 as f64 * ratio) as u8;
    Ok(Color::Rgb(r, g, b))
}

/// Gets the text label to be displayed inside a span bar.
fn get_bar_text(span: &Arc<OtelSpan>, bar_width: usize) -> Result<String> {
    if bar_width == 0 {
        return Err(anyhow!("Illegal: got less than 1 for bar width"));
    }

    let label = format!(" {} ({})", span.name, format_duration(span.duration()));

    if label.len() <= bar_width {
        // The label will fit
        return Ok(format!("{:<width$}", label, width = bar_width));
    }

    Ok(format!("{}…", &label[..bar_width - 1]))
}

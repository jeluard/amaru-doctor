use crate::{
    model::otel_view::OtelViewState,
    otel::{
        TreeBounds,
        graph::TraceGraph,
        id::{SpanId, TraceId},
        span_ext::SpanExt,
    },
    view::span_bar::SpanBarRenderer,
};
use anyhow::{Result, anyhow};
use opentelemetry_proto::tonic::trace::v1::Span;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::{iter, sync::Arc};

/// Renders the flame graph widget.
pub fn render_flame_graph(frame: &mut Frame, area: Rect, state: &OtelViewState, is_focused: bool) {
    let mut block = Block::default()
        .title("Trace Details")
        .borders(Borders::ALL);
    if is_focused {
        block = block.border_style(Style::default().fg(Color::Blue));
    }

    let lines = get_flame_graph_lines(state, area.width.saturating_sub(2) as usize).unwrap();

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

/// Determines which view to render based on the app state.
fn get_flame_graph_lines(
    state: &OtelViewState,
    max_bar_width: usize,
) -> Result<Vec<Line<'static>>> {
    if let Some(selected_span) = &state.selected_span {
        // A Span is selected
        get_span_tree_lines(state, selected_span, max_bar_width)
    } else if let Some(trace_id) = state.trace_list.selected_item() {
        // No Span is selected but a Trace is selected
        get_trace_tree_lines(state, trace_id, max_bar_width)
    } else {
        Ok(vec![Line::from("No Trace selected.")])
    }
}

/// Gets the lines for the Span tree view, including the Span's ancestors and descendants.
fn get_span_tree_lines(
    state: &OtelViewState,
    selected_span: &Arc<Span>,
    max_bar_width: usize,
) -> Result<Vec<Line<'static>>> {
    let graph = &state.trace_graph_source.load();
    let selected_span_id = selected_span.span_id();

    let Some(span_tree) = graph.subtrees.get(&selected_span_id) else {
        return Err(anyhow!(
            "Unexpected: Subtree not found for Span {}",
            selected_span_id
        ));
    };

    let mut ancestors: Vec<SpanId> = graph.ancestor_iter(selected_span_id).collect();
    // We want the list to start from the root and go down to the span's parent
    ancestors.reverse();
    let descendants = graph.descendent_iter(selected_span_id);

    get_lines(
        graph,
        state,
        span_tree.bounds(),
        max_bar_width,
        ancestors.into_iter(),
        descendants,
    )
}

/// Gets the lines for the Trace view, each Root's tree.
fn get_trace_tree_lines(
    state: &OtelViewState,
    trace_id: &TraceId,
    max_bar_width: usize,
) -> Result<Vec<Line<'static>>> {
    let graph = &state.trace_graph_source.load();

    let Some(trace_meta) = graph.traces.get(trace_id) else {
        return Err(anyhow!("Unexpected: Trace {} not found", trace_id));
    };
    let (Some(start), Some(end)) = (trace_meta.start_time(), trace_meta.end_time(graph)) else {
        return Err(anyhow!(
            "Unexpected: Trace {} is missing start or end time",
            trace_id
        ));
    };

    let bounds = &TreeBounds { start, end };
    let descendants = graph.trace_iter(trace_id);

    // A Trace has no ancestors, so we pass an empty iter
    get_lines(
        graph,
        state,
        bounds,
        max_bar_width,
        iter::empty(),
        descendants,
    )
}

/// Gets the span bar lines for an ancestor list and a descendant list.
fn get_lines(
    graph: &TraceGraph,
    state: &OtelViewState,
    bounds: &TreeBounds,
    max_bar_width: usize,
    ancestors: impl Iterator<Item = SpanId>,
    descendants: impl Iterator<Item = SpanId>,
) -> Result<Vec<Line<'static>>> {
    let renderer = SpanBarRenderer::new(graph, state, bounds, max_bar_width)?;

    let tagged_ancestors = ancestors.map(|id| (id, true));
    let tagged_descendants = descendants.map(|id| (id, false));

    tagged_ancestors
        .chain(tagged_descendants)
        .map(|(id, is_ancestor)| renderer.render(&id, is_ancestor))
        .collect()
}

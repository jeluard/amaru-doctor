use crate::{
    app_state::AppState,
    components::Component,
    model::otel_view::OtelViewState,
    otel::{graph::TraceGraph, id::SpanId, span_ext::SpanExt},
    states::{Action, InspectOption, LedgerBrowse, LedgerMode, WidgetSlot},
    update::Update,
};
use strum::Display;
use tracing::{debug, trace};

#[derive(Display, Debug, Clone, Copy)]
pub enum ScrollDirection {
    Up,
    Down,
}

pub struct ScrollUpdate;
impl Update for ScrollUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Some(direction) = (match action {
            Action::ScrollUp => Some(ScrollDirection::Up),
            Action::ScrollDown => Some(ScrollDirection::Down),
            _ => None,
        }) else {
            return Vec::new();
        };

        match s.layout_model.get_focus() {
            WidgetSlot::LedgerOptions => match s.get_ledger_mode_tabs().selected() {
                LedgerMode::Browse => {
                    return s.get_ledger_browse_options_mut().handle_scroll(direction);
                }
                LedgerMode::Search => match direction {
                    ScrollDirection::Up => s.get_ledger_search_options_mut().view.cursor_back(),
                    ScrollDirection::Down => s.get_ledger_search_options_mut().view.cursor_next(),
                },
            },
            WidgetSlot::List => match s.get_inspect_tabs().cursor.current() {
                InspectOption::Ledger => match s.get_ledger_mode_tabs().selected() {
                    LedgerMode::Browse => scroll_ledger_list(s, direction),
                    LedgerMode::Search => {
                        if let Some(search_res) =
                            s.ledger_mvs.utxos_by_addr_search.get_current_res_mut()
                        {
                            return search_res.handle_scroll(direction);
                        }
                    }
                },
                InspectOption::Otel => {
                    // TODO: Make this logic simpler by taking advantage of the
                    // DynamicList struct

                    // First scroll the trace list
                    match direction {
                        ScrollDirection::Up => s.otel_view.trace_list.cursor_back(),
                        ScrollDirection::Down => s.otel_view.trace_list.cursor_next(),
                    }

                    let graph = s.otel_view.trace_graph_source.load();

                    // Then find the new focused span--it's the first span (root) in the
                    // new trace
                    let new_focused_span = s
                        .otel_view
                        .trace_list
                        .selected_item()
                        .and_then(|trace_id| graph.traces.get(trace_id))
                        .and_then(|trace_meta| trace_meta.roots().first_key_value())
                        .and_then(|(_, root_ids)| root_ids.first())
                        .and_then(|root_id| graph.spans.get(root_id))
                        .cloned();

                    s.otel_view.focused_span = new_focused_span;
                    // If we've scrolled to a new Trace, the selected span is reset
                    s.otel_view.selected_span = None;
                }
                //InspectOption::Chain => { /* There's no list widget in the Chain tab */ }
                InspectOption::Prometheus => { /* There's no list widget in the Prometheus tab */ }
            },
            WidgetSlot::Details => match s.get_inspect_tabs().cursor.current() {
                InspectOption::Otel => scroll_trace_details(&mut s.otel_view, direction),
                InspectOption::Ledger => { /* TODO: Impl item details scroll */ }
                //InspectOption::Chain => {}
                InspectOption::Prometheus => { /* TODO: Impl metrics scroll */ }
            },
            _ => trace!("No scroll logic for slot {:?}", s.layout_model.get_focus()),
        }
        Vec::new()
    }
}

/// Scrolls the list within the ledger view.
fn scroll_ledger_list(s: &mut AppState, direction: ScrollDirection) {
    if let Some(browse_option) = s.get_ledger_browse_options().view.selected_item() {
        debug!(
            "Scrolling ledger list cursor for browse option: {:?}",
            browse_option
        );
        match (browse_option, direction) {
            (LedgerBrowse::Accounts, _) => {
                s.get_accounts_list_mut().handle_scroll(direction);
            }
            (LedgerBrowse::BlockIssuers, ScrollDirection::Up) => {
                s.get_block_issuers_list_mut().view.cursor_back()
            }
            (LedgerBrowse::BlockIssuers, ScrollDirection::Down) => {
                s.get_block_issuers_list_mut().view.cursor_next()
            }
            (LedgerBrowse::DReps, ScrollDirection::Up) => s.get_dreps_list_mut().view.cursor_back(),
            (LedgerBrowse::DReps, ScrollDirection::Down) => {
                s.get_dreps_list_mut().view.cursor_next()
            }
            (LedgerBrowse::Pools, ScrollDirection::Up) => s.get_pools_list_mut().view.cursor_back(),
            (LedgerBrowse::Pools, ScrollDirection::Down) => {
                s.get_pools_list_mut().view.cursor_next()
            }
            (LedgerBrowse::Proposals, ScrollDirection::Up) => {
                s.get_proposals_list_mut().view.cursor_back()
            }
            (LedgerBrowse::Proposals, ScrollDirection::Down) => {
                s.get_proposals_list_mut().view.cursor_next()
            }
            (LedgerBrowse::Utxos, ScrollDirection::Up) => s.get_utxos_list_mut().view.cursor_back(),
            (LedgerBrowse::Utxos, ScrollDirection::Down) => {
                s.get_utxos_list_mut().view.cursor_next()
            }
        }
    }
}

/// Scrolls to the next focused span within the OTEL trace details view.
fn scroll_trace_details(otel_view: &mut OtelViewState, direction: ScrollDirection) {
    let data = otel_view.trace_graph_source.load();
    let Some(ordered_spans) = get_ordered_spans_for_view(&data, otel_view) else {
        return;
    };
    if ordered_spans.is_empty() {
        return;
    }

    // Find the index of the currently focused span in the span list
    let current_index = otel_view
        .focused_span
        .as_ref()
        .and_then(|span| ordered_spans.iter().position(|id| *id == span.span_id()));

    let len = ordered_spans.len();

    let new_index = match direction {
        // Allow wrapping
        ScrollDirection::Down => current_index.map_or(0, |i| (i + 1) % len),
        ScrollDirection::Up => current_index.map_or(len - 1, |i| (i + len - 1) % len),
    };

    if Some(new_index) != current_index {
        // Update the focused span given the new index
        otel_view.focused_span = ordered_spans
            .get(new_index)
            .and_then(|id| data.spans.get(id).cloned());
    }
}

/// Helper to get the list of spans for scrolling. If a span is selected, we
/// only get that span's subtree. If a span isn't selected, we get all the spans
/// for the selected trace.
fn get_ordered_spans_for_view(data: &TraceGraph, otel_view: &OtelViewState) -> Option<Vec<SpanId>> {
    // Determine if a span is selected
    if let Some(selected_span) = &otel_view.selected_span {
        let selected_span_id = selected_span.span_id();
        // Get the span's ancestors. The iter starts at the span's parent and walks *up*
        // the tree--we reverse this so that the resulting list is in ascending order.
        let mut ancestors: Vec<SpanId> = data.ancestor_iter(selected_span_id).collect();
        ancestors.reverse();
        let self_and_descendants = data.descendent_iter(selected_span_id);
        Some(ancestors.into_iter().chain(self_and_descendants).collect())
    } else {
        // There's no selected span, render the selected trace's entire tree
        otel_view
            .trace_list
            .selected_item()
            .map(|trace_id| data.trace_iter(trace_id).collect())
    }
}

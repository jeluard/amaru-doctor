use crate::{
    app_state::AppState,
    components::Component,
    states::{Action, ComponentId},
    update::Update,
};
use crossterm::event::{MouseButton, MouseEventKind};
use tracing::debug;

pub struct MouseClickUpdate;

impl Update for MouseClickUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::MouseEvent(mouse_event) = action else {
            return vec![];
        };

        if mouse_event.kind != MouseEventKind::Down(MouseButton::Left) {
            return vec![];
        }

        let Some((component_id, rect)) = s
            .layout_model
            .find_hovered_slot(mouse_event.column, mouse_event.row)
        else {
            debug!("Couldn't find slot for click {:?}", mouse_event);
            return vec![];
        };

        // Calculate relative row for list components
        // +1 to account for the border
        let relative_row = mouse_event.row.saturating_sub(rect.y + 1) as usize;

        match component_id {
            ComponentId::InspectTabs => {
                s.get_inspect_tabs_mut()
                    .handle_click(rect, mouse_event.row, mouse_event.column);
                // Clicking tabs changes layout, so we return an update action
                return vec![Action::UpdateLayout(s.frame_area)];
            }
            ComponentId::LedgerModeTabs => {
                if s.get_ledger_mode_tabs_mut()
                    .select_by_column(rect, mouse_event.column)
                {
                    return vec![Action::UpdateLayout(s.frame_area)];
                }
            }

            ComponentId::OtelTraceList => {
                // Update the list selection
                s.get_trace_list_mut()
                    .handle_click(rect, mouse_event.row, mouse_event.column);

                // If we switched traces, we clear the selected span so the
                // flamegraph switches to the new trace's root.
                let graph = s.otel_view.trace_graph_source.load();

                // Set focused span to the root of the new trace (if any)
                let new_focused_span = s
                    .get_trace_list()
                    .selected_item()
                    .and_then(|trace_id| graph.traces.get(trace_id))
                    .and_then(|trace_meta| trace_meta.roots().first_key_value())
                    .and_then(|(_, root_ids)| root_ids.first())
                    .and_then(|root_id| graph.spans.get(root_id))
                    .cloned();

                s.otel_view.focused_span = new_focused_span;
                // Clear the specifically selected span to reset the view
                s.otel_view.selected_span = None;
            }
            ComponentId::OtelFlameGraph => {
                // If clicking the flamegraph, "select" the currently focused span
                if let Some(span) = &s.otel_view.focused_span {
                    s.otel_view.selected_span = Some(span.clone());
                }
            }

            ComponentId::LedgerBrowseOptions => {
                s.get_ledger_browse_options_mut().handle_click(
                    rect,
                    mouse_event.row,
                    mouse_event.column,
                );
                return vec![Action::UpdateLayout(s.frame_area)];
            }
            ComponentId::LedgerSearchOptions => {
                s.get_ledger_search_options_mut()
                    .model_view
                    .select_index_by_row(relative_row);
            }

            // Ledger Lists
            ComponentId::LedgerAccountsList => {
                s.get_accounts_list_mut()
                    .model_view
                    .select_index_by_row(relative_row);
            }
            ComponentId::LedgerBlockIssuersList => {
                s.get_block_issuers_list_mut()
                    .model_view
                    .select_index_by_row(relative_row);
            }
            ComponentId::LedgerDRepsList => {
                s.get_dreps_list_mut()
                    .model_view
                    .select_index_by_row(relative_row);
            }
            ComponentId::LedgerPoolsList => {
                s.get_pools_list_mut()
                    .model_view
                    .select_index_by_row(relative_row);
            }
            ComponentId::LedgerProposalsList => {
                s.get_proposals_list_mut()
                    .model_view
                    .select_index_by_row(relative_row);
            }
            ComponentId::LedgerUtxosList => {
                s.get_utxos_list_mut()
                    .model_view
                    .select_index_by_row(relative_row);
            }
            ComponentId::LedgerUtxosByAddrList => {
                if let Some(search_res) = s.ledger_mvs.utxos_by_addr_search.get_current_res_mut() {
                    search_res.select_index_by_row(relative_row);
                }
            }
            _ => {}
        }

        vec![]
    }
}

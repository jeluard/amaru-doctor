use crate::{
    app_state::AppState,
    components::{Component, list::ListModel},
    states::{Action, ComponentId::*},
    update::Update,
};
use crossterm::event::{MouseButton, MouseEventKind};
use tracing::{debug, warn};

pub struct MouseClickUpdate;

impl Update for MouseClickUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let (column, row) = match action {
            Action::MouseEvent(mouse_event) => {
                if mouse_event.kind != MouseEventKind::Down(MouseButton::Left) {
                    return Vec::new();
                }
                (mouse_event.column, mouse_event.row)
            }
            Action::MouseClick(col, row) => (*col, *row),
            _ => return Vec::new(),
        };

        let Some((component_id, rect)) = s.layout_model.find_hovered_slot(column, row) else {
            return Vec::new();
        };

        // +1 to account for the border
        let relative_row = row.saturating_sub(rect.y + 1) as usize;

        match component_id {
            InspectTabs => {
                s.get_inspect_tabs_mut().handle_click(rect, row, column);
                return vec![Action::UpdateLayout(s.frame_area)];
            }
            LedgerModeTabs => {
                if s.get_ledger_mode_tabs_mut().select_by_column(rect, column) {
                    return vec![Action::UpdateLayout(s.frame_area)];
                }
            }

            LedgerBrowseOptions => {
                s.get_ledger_browse_options_mut()
                    .model
                    .select_index_by_row(relative_row);
                return vec![Action::UpdateLayout(s.frame_area)];
            }
            LedgerSearchOptions => {
                s.get_ledger_search_options_mut()
                    .model
                    .select_index_by_row(relative_row);
                return vec![Action::UpdateLayout(s.frame_area)];
            }

            LedgerAccountsList => {
                s.get_accounts_list_mut()
                    .model
                    .select_index_by_row(relative_row);
            }
            LedgerBlockIssuersList => {
                s.get_block_issuers_list_mut()
                    .model
                    .select_index_by_row(relative_row);
            }
            LedgerDRepsList => {
                s.get_dreps_list_mut()
                    .model
                    .select_index_by_row(relative_row);
            }
            LedgerPoolsList => {
                s.get_pools_list_mut()
                    .model
                    .select_index_by_row(relative_row);
            }
            LedgerProposalsList => {
                s.get_proposals_list_mut()
                    .model
                    .select_index_by_row(relative_row);
            }
            LedgerUtxosList => {
                s.get_utxos_list_mut()
                    .model
                    .select_index_by_row(relative_row);
            }

            LedgerUtxosByAddrList => {
                if let Some(model) = s.ledger_mvs.utxos_by_addr_search.get_current_res_mut() {
                    debug!("MouseClickUpdate: Selecting row {}", relative_row);
                    model.select_index_by_row(relative_row);
                } else {
                    warn!("MouseClickUpdate: Click on SearchList but no model found");
                }
            }

            OtelTraceList => {
                s.get_trace_list_mut().handle_click(rect, row, column);

                // Side Effect: Update Focused Span based on new Trace selection
                let graph = s.otel_view.trace_graph_source.load();
                let new_focused_span = s
                    .get_trace_list()
                    .selected_item()
                    .and_then(|trace_id| graph.traces.get(trace_id))
                    .and_then(|trace_meta| trace_meta.roots().first_key_value())
                    .and_then(|(_, root_ids)| root_ids.first())
                    .and_then(|root_id| graph.spans.get(root_id))
                    .cloned();

                s.otel_view.focused_span = new_focused_span;
                s.otel_view.selected_span = None;
            }
            OtelFlameGraph => {
                if let Some(span) = &s.otel_view.focused_span {
                    s.otel_view.selected_span = Some(span.clone());
                }
            }

            _ => {
                debug!(
                    "Clicked component {} with no specific handler",
                    component_id
                );
            }
        }

        Vec::new()
    }
}

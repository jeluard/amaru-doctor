use crate::{
    app_state::AppState,
    components::Component,
    states::{
        Action, InspectOption, LedgerBrowse, LedgerMode, LedgerSearch::UtxosByAddress, WidgetSlot,
    },
    update::Update,
};
use crossterm::event::{MouseButton, MouseEventKind};
use tracing::debug;

pub struct MouseClickUpdate;
impl Update for MouseClickUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::MouseEvent(mouse_event) = action else {
            return Vec::new();
        };

        if mouse_event.kind != MouseEventKind::Down(MouseButton::Left) {
            // We only care about mouse down events
            return Vec::new();
        };

        let Some((slot, rect)) = s
            .layout_model
            .find_hovered_slot(mouse_event.column, mouse_event.row)
        else {
            debug!("Couldn't find slot for click {:?}", mouse_event);
            return Vec::new();
        };

        let relative_row = mouse_event.row.saturating_sub(rect.y + 1) as usize;

        match slot {
            WidgetSlot::InspectOption => {
                s.get_inspect_tabs_mut()
                    .handle_click(rect, mouse_event.row, mouse_event.column)
                    .is_empty();
            }
            WidgetSlot::LedgerMode => {
                s.get_ledger_mode_tabs_mut()
                    .select_by_column(rect, mouse_event.column);
            }
            WidgetSlot::LedgerOptions => match s.get_ledger_mode_tabs().selected() {
                LedgerMode::Browse => {
                    return s.get_ledger_browse_options_mut().handle_click(
                        rect,
                        mouse_event.row,
                        mouse_event.column,
                    );
                }
                LedgerMode::Search => {
                    let relative_row = mouse_event.row.saturating_sub(rect.y + 1) as usize;
                    s.get_ledger_search_options_mut()
                        .view
                        .select_index_by_row(relative_row);
                }
            },
            WidgetSlot::List => match s.get_inspect_tabs().cursor.current() {
                InspectOption::Otel => {
                    s.otel_view.trace_list.select_index_by_row(relative_row);
                }
                InspectOption::Ledger => match *s.get_ledger_mode_tabs().cursor.current() {
                    LedgerMode::Browse => {
                        if let Some(browse_option) =
                            s.get_ledger_browse_options().view.selected_item()
                        {
                            match browse_option {
                                LedgerBrowse::Accounts => s
                                    .get_accounts_list_mut()
                                    .view
                                    .select_index_by_row(relative_row),
                                LedgerBrowse::BlockIssuers => s
                                    .get_block_issuers_list_mut()
                                    .view
                                    .select_index_by_row(relative_row),
                                LedgerBrowse::DReps => s
                                    .get_dreps_list_mut()
                                    .view
                                    .select_index_by_row(relative_row),
                                LedgerBrowse::Pools => s
                                    .get_pools_list_mut()
                                    .view
                                    .select_index_by_row(relative_row),
                                LedgerBrowse::Proposals => s
                                    .get_proposals_list_mut()
                                    .view
                                    .select_index_by_row(relative_row),
                                LedgerBrowse::Utxos => s
                                    .get_utxos_list_mut()
                                    .view
                                    .select_index_by_row(relative_row),
                            }
                        }
                    }
                    LedgerMode::Search => {
                        if let Some(search_option) =
                            s.get_ledger_search_options().view.selected_item()
                        {
                            match search_option {
                                UtxosByAddress => {
                                    if let Some(search_res) =
                                        s.ledger_mvs.utxos_by_addr_search.get_current_res_mut()
                                    {
                                        search_res.select_index_by_row(relative_row);
                                    }
                                }
                            }
                        }
                    }
                },
                _ => debug!(
                    "Clicked a page {} with no click action",
                    s.get_inspect_tabs().cursor.current()
                ),
            },
            WidgetSlot::Details => match s.get_inspect_tabs().cursor.current() {
                InspectOption::Otel => {
                    if let Some(span) = &s.otel_view.focused_span {
                        s.otel_view.selected_span = Some(span.clone());
                    }
                }
                _ => debug!(
                    "No click action in Details slot for inspect option {}",
                    s.get_inspect_tabs().cursor.current()
                ),
            },
            _ => debug!("Clicked a slot {} with no click action", slot),
        }

        vec![Action::UpdateLayout(s.frame_area)]
    }
}

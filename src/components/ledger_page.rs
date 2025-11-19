use crate::{
    ScreenMode,
    app_state::AppState,
    components::{
        Component, ComponentLayout, InputRoute, MouseScrollDirection, ScrollDirection,
        route_event_to_children,
    },
    controller::{LayoutSpec, walk_layout},
    states::{Action, ComponentId, LedgerBrowse, LedgerMode},
    tui::Event,
};
use crossterm::event::KeyEvent;
use either::Either::{Left, Right};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use std::{any::Any, collections::HashMap};

pub struct LedgerPageComponent {
    id: ComponentId,
}

impl Default for LedgerPageComponent {
    fn default() -> Self {
        Self {
            id: ComponentId::LedgerPage,
        }
    }
}
impl LedgerPageComponent {
    fn build_layout_spec(&self, s: &AppState) -> LayoutSpec {
        let ledger_mode = s.get_ledger_mode_tabs().selected();
        let screen_mode = s.screen_mode;

        // Build Header
        let header_constraints = match ledger_mode {
            LedgerMode::Browse => vec![
                (Constraint::Fill(1), Left(ComponentId::InspectTabs)),
                (Constraint::Fill(1), Left(ComponentId::LedgerModeTabs)),
            ],
            LedgerMode::Search => vec![
                (Constraint::Length(30), Left(ComponentId::InspectTabs)),
                (Constraint::Length(20), Left(ComponentId::LedgerModeTabs)),
                (Constraint::Fill(1), Left(ComponentId::SearchBar)),
            ],
        };
        let header_spec = LayoutSpec {
            direction: Direction::Horizontal,
            constraints: header_constraints,
        };

        // Build Left Column (Options + List)
        let (options_id, list_id) = match ledger_mode {
            LedgerMode::Browse => {
                let list = match s.get_ledger_browse_options().model.selected_item() {
                    Some(LedgerBrowse::Accounts) => ComponentId::LedgerAccountsList,
                    Some(LedgerBrowse::BlockIssuers) => ComponentId::LedgerBlockIssuersList,
                    Some(LedgerBrowse::DReps) => ComponentId::LedgerDRepsList,
                    Some(LedgerBrowse::Pools) => ComponentId::LedgerPoolsList,
                    Some(LedgerBrowse::Proposals) => ComponentId::LedgerProposalsList,
                    Some(LedgerBrowse::Utxos) => ComponentId::LedgerUtxosList,
                    None => ComponentId::LedgerAccountsList,
                };
                (ComponentId::LedgerBrowseOptions, list)
            }
            LedgerMode::Search => (
                ComponentId::LedgerSearchOptions,
                ComponentId::LedgerUtxosByAddrList,
            ),
        };

        let left_col_spec = LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![
                (Constraint::Fill(1), Left(options_id)),
                (Constraint::Fill(3), Left(list_id)),
            ],
        };

        // Determine Details Component
        let details_id = match ledger_mode {
            LedgerMode::Browse => match s.get_ledger_browse_options().model.selected_item() {
                Some(LedgerBrowse::Accounts) => ComponentId::LedgerAccountDetails,
                Some(LedgerBrowse::BlockIssuers) => ComponentId::LedgerBlockIssuerDetails,
                Some(LedgerBrowse::DReps) => ComponentId::LedgerDRepDetails,
                Some(LedgerBrowse::Pools) => ComponentId::LedgerPoolDetails,
                Some(LedgerBrowse::Proposals) => ComponentId::LedgerProposalDetails,
                Some(LedgerBrowse::Utxos) => ComponentId::LedgerUtxoDetails,
                None => ComponentId::LedgerAccountDetails,
            },
            LedgerMode::Search => ComponentId::LedgerUtxosByAddrDetails,
        };

        // Build Body
        let body_spec = LayoutSpec {
            direction: Direction::Horizontal,
            constraints: vec![
                (Constraint::Percentage(20), Right(left_col_spec)),
                (Constraint::Percentage(80), Left(details_id)),
            ],
        };

        // Combine into Page Layout
        match screen_mode {
            ScreenMode::Large => LayoutSpec {
                direction: Direction::Vertical,
                constraints: vec![
                    (Constraint::Length(3), Right(header_spec)),
                    (Constraint::Fill(1), Right(body_spec)),
                ],
            },
            ScreenMode::Small => LayoutSpec {
                direction: Direction::Vertical,
                constraints: vec![(Constraint::Fill(1), Right(body_spec))],
            },
        }
    }
}

impl Component for LedgerPageComponent {
    fn id(&self) -> ComponentId {
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn calculate_layout(&self, area: Rect, s: &AppState) -> ComponentLayout {
        let spec = self.build_layout_spec(s);
        let mut layout = HashMap::new();
        walk_layout(&mut layout, &spec, area);
        layout
    }

    fn route_event(&self, event: &Event, s: &AppState) -> InputRoute {
        let area = s.frame_area;
        let my_layout = self.calculate_layout(area, s);

        route_event_to_children(event, s, my_layout)
    }

    fn render(&self, f: &mut Frame, s: &AppState, _layout: &ComponentLayout) {
        let layout = self.calculate_layout(f.area(), s);

        // Render children
        for (id, _) in layout.iter() {
            if let Some(child) = s.component_registry.get(id) {
                child.render(f, s, &layout);
            }
        }
    }

    fn handle_scroll(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_key_event(&mut self, _key: KeyEvent) -> Vec<Action> {
        Vec::new()
    }
    fn handle_click(&mut self, _area: Rect, _row: u16, _col: u16) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_scroll(&mut self, _direction: MouseScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_drag(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
}

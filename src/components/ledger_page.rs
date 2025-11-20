use crate::{
    ScreenMode,
    app_state::AppState,
    components::{
        Component, ComponentLayout, InputRoute, list::ListComponent, route_event_to_children,
        search_list::SearchListComponent, stateful_details::StatefulDetailsComponent,
    },
    controller::{LayoutSpec, walk_layout},
    model::{ledger_search::LedgerUtxoProvider, list_view::ListModelView},
    states::{Action, ComponentId, LedgerBrowse, LedgerMode, LedgerSearch},
    store::owned_iter::{
        OwnedAccountIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter, OwnedProposalIter,
        OwnedUtxoIter,
    },
    tui::Event,
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
};
use amaru_kernel::Address;
use amaru_stores::rocksdb::ReadOnlyRocksDB;
use either::Either::{Left, Right};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, RwLock},
};
use strum::IntoEnumIterator;

pub struct LedgerPageComponent {
    id: ComponentId,
    // Details
    account_details: StatefulDetailsComponent<AccountItem>,
    block_details: StatefulDetailsComponent<BlockIssuerItem>,
    drep_details: StatefulDetailsComponent<DRepItem>,
    pool_details: StatefulDetailsComponent<PoolItem>,
    proposal_details: StatefulDetailsComponent<ProposalItem>,
    utxo_details: StatefulDetailsComponent<UtxoItem>,
    utxo_by_addr_details: StatefulDetailsComponent<UtxoItem>,

    // Lists
    // Options
    browse_options: ListComponent<ListModelView<LedgerBrowse>>,
    search_options: ListComponent<ListModelView<LedgerSearch>>,

    // Content
    accounts_list: ListComponent<ListModelView<AccountItem>>,
    block_issuers_list: ListComponent<ListModelView<BlockIssuerItem>>,
    dreps_list: ListComponent<ListModelView<DRepItem>>,
    pools_list: ListComponent<ListModelView<PoolItem>>,
    proposals_list: ListComponent<ListModelView<ProposalItem>>,
    utxos_list: ListComponent<ListModelView<UtxoItem>>,

    // Search
    utxos_by_addr_list: SearchListComponent<Address, UtxoItem>,

    last_layout: RwLock<ComponentLayout>,
    active_focus: RwLock<ComponentId>,
}

impl LedgerPageComponent {
    pub fn new(db: Arc<ReadOnlyRocksDB>) -> Self {
        let list_height = 0; // Will be updated in render
        let options_height = 0;

        Self {
            id: ComponentId::LedgerPage,

            // Details
            account_details: StatefulDetailsComponent::new(
                ComponentId::LedgerAccountDetails,
                "Account Details",
            ),
            block_details: StatefulDetailsComponent::new(
                ComponentId::LedgerBlockIssuerDetails,
                "Block Issuer Details",
            ),
            drep_details: StatefulDetailsComponent::new(
                ComponentId::LedgerDRepDetails,
                "DRep Details",
            ),
            pool_details: StatefulDetailsComponent::new(
                ComponentId::LedgerPoolDetails,
                "Pool Details",
            ),
            proposal_details: StatefulDetailsComponent::new(
                ComponentId::LedgerProposalDetails,
                "Proposal Details",
            ),
            utxo_details: StatefulDetailsComponent::new(
                ComponentId::LedgerUtxoDetails,
                "UTXO Details",
            ),
            utxo_by_addr_details: StatefulDetailsComponent::new(
                ComponentId::LedgerUtxosByAddrDetails,
                "UTXO Details",
            ),

            // Options
            browse_options: ListComponent::new(
                ComponentId::LedgerBrowseOptions,
                ListModelView::new("Browse Options", LedgerBrowse::iter(), options_height),
            ),
            search_options: ListComponent::new(
                ComponentId::LedgerSearchOptions,
                ListModelView::new("Search Options", LedgerSearch::iter(), options_height),
            ),

            // Lists
            accounts_list: ListComponent::new(
                ComponentId::LedgerAccountsList,
                ListModelView::new("Accounts", OwnedAccountIter::new(db.clone()), list_height),
            ),
            block_issuers_list: ListComponent::new(
                ComponentId::LedgerBlockIssuersList,
                ListModelView::new(
                    "Block Issuers",
                    OwnedBlockIssuerIter::new(db.clone()),
                    list_height,
                ),
            ),
            dreps_list: ListComponent::new(
                ComponentId::LedgerDRepsList,
                ListModelView::new("DReps", OwnedDRepIter::new(db.clone()), list_height),
            ),
            pools_list: ListComponent::new(
                ComponentId::LedgerPoolsList,
                ListModelView::new("Pools", OwnedPoolIter::new(db.clone()), list_height),
            ),
            proposals_list: ListComponent::new(
                ComponentId::LedgerProposalsList,
                ListModelView::new("Proposals", OwnedProposalIter::new(db.clone()), list_height),
            ),
            utxos_list: ListComponent::new(
                ComponentId::LedgerUtxosList,
                ListModelView::new("Utxos", OwnedUtxoIter::new(db.clone()), list_height),
            ),

            // Search Lists
            utxos_by_addr_list: SearchListComponent::new(
                ComponentId::LedgerUtxosByAddrList,
                "Utxos by Address",
                Box::new(LedgerUtxoProvider { db: db.clone() }),
            ),

            last_layout: RwLock::new(ComponentLayout::new()),
            active_focus: RwLock::new(ComponentId::LedgerBrowseOptions),
        }
    }

    fn dispatch_to_child(&mut self, id: ComponentId, event: &Event, area: Rect) -> Vec<Action> {
        match id {
            // Options
            ComponentId::LedgerBrowseOptions => self.browse_options.handle_event(event, area),
            ComponentId::LedgerSearchOptions => self.search_options.handle_event(event, area),

            // Lists
            ComponentId::LedgerAccountsList => self.accounts_list.handle_event(event, area),
            ComponentId::LedgerBlockIssuersList => {
                self.block_issuers_list.handle_event(event, area)
            }
            ComponentId::LedgerDRepsList => self.dreps_list.handle_event(event, area),
            ComponentId::LedgerPoolsList => self.pools_list.handle_event(event, area),
            ComponentId::LedgerProposalsList => self.proposals_list.handle_event(event, area),
            ComponentId::LedgerUtxosList => self.utxos_list.handle_event(event, area),
            ComponentId::LedgerUtxosByAddrList => self.utxos_by_addr_list.handle_event(event, area),

            // Details
            ComponentId::LedgerAccountDetails => self.account_details.handle_event(event, area),
            ComponentId::LedgerBlockIssuerDetails => self.block_details.handle_event(event, area),
            ComponentId::LedgerDRepDetails => self.drep_details.handle_event(event, area),
            ComponentId::LedgerPoolDetails => self.pool_details.handle_event(event, area),
            ComponentId::LedgerProposalDetails => self.proposal_details.handle_event(event, area),
            ComponentId::LedgerUtxoDetails => self.utxo_details.handle_event(event, area),
            ComponentId::LedgerUtxosByAddrDetails => {
                self.utxo_by_addr_details.handle_event(event, area)
            }

            // Default
            _ => Vec::new(),
        }
    }

    // Helper to determine which list is currently active in the UI
    fn get_active_list_component_id(&self) -> ComponentId {
        match self.browse_options.model.selected_item() {
            Some(LedgerBrowse::Accounts) => ComponentId::LedgerAccountsList,
            Some(LedgerBrowse::BlockIssuers) => ComponentId::LedgerBlockIssuersList,
            Some(LedgerBrowse::DReps) => ComponentId::LedgerDRepsList,
            Some(LedgerBrowse::Pools) => ComponentId::LedgerPoolsList,
            Some(LedgerBrowse::Proposals) => ComponentId::LedgerProposalsList,
            Some(LedgerBrowse::Utxos) => ComponentId::LedgerUtxosList,
            None => ComponentId::LedgerAccountsList,
        }
    }

    fn build_layout_spec(&self, s: &AppState) -> LayoutSpec {
        let ledger_mode = s.get_ledger_mode_tabs().selected();
        let screen_mode = s.screen_mode;

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

        let (options_id, list_id) = match ledger_mode {
            LedgerMode::Browse => (
                ComponentId::LedgerBrowseOptions,
                self.get_active_list_component_id(),
            ),
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

        let details_id = match ledger_mode {
            LedgerMode::Browse => match self.browse_options.model.selected_item() {
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

        let body_spec = LayoutSpec {
            direction: Direction::Horizontal,
            constraints: vec![
                (Constraint::Percentage(20), Right(left_col_spec)),
                (Constraint::Percentage(80), Left(details_id)),
            ],
        };

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
        let route = route_event_to_children(event, s, my_layout);

        match route {
            InputRoute::Delegate(ComponentId::LedgerAccountDetails |
                ComponentId::LedgerBlockIssuerDetails |
                ComponentId::LedgerDRepDetails |
                ComponentId::LedgerPoolDetails |
                ComponentId::LedgerProposalDetails |
                ComponentId::LedgerUtxoDetails |
                ComponentId::LedgerUtxosByAddrDetails |
                // Options
                ComponentId::LedgerBrowseOptions |
                ComponentId::LedgerSearchOptions |
                // Lists
                ComponentId::LedgerAccountsList |
                ComponentId::LedgerBlockIssuersList |
                ComponentId::LedgerDRepsList |
                ComponentId::LedgerPoolsList |
                ComponentId::LedgerProposalsList |
                ComponentId::LedgerUtxosList |
                ComponentId::LedgerUtxosByAddrList, _) =>
            {
                InputRoute::Handle
            }
            _ => route,
        }
    }

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        let old_browse = self.browse_options.model.selected_item().cloned();
        let old_search = self.search_options.model.selected_item().cloned();

        let target_id = match event {
            Event::Key(_) => {
                // For keys, route to the focused component
                *self.active_focus.read().unwrap()
            }
            Event::Mouse(mouse) => {
                // For mouse, hit-test against the cached layout
                let layout = self.last_layout.read().unwrap();
                layout
                    .iter()
                    .find(|(_, rect)| {
                        mouse.column >= rect.x
                            && mouse.column < rect.x + rect.width
                            && mouse.row >= rect.y
                            && mouse.row < rect.y + rect.height
                    })
                    .map(|(id, _)| *id)
                    .unwrap_or_else(|| *self.active_focus.read().unwrap()) // Fallback to focus if click missed
            }
            _ => *self.active_focus.read().unwrap(),
        };

        // Dispatch to the target
        let child_area = {
            let layout = self.last_layout.read().unwrap();
            layout.get(&target_id).copied().unwrap_or(area)
        };

        let mut actions = self.dispatch_to_child(target_id, event, child_area);

        // Check for layout changes
        if self.browse_options.model.selected_item() != old_browse.as_ref()
            || self.search_options.model.selected_item() != old_search.as_ref()
        {
            actions.push(Action::UpdateLayout(Rect::default()));
        }

        actions
    }

    fn tick(&mut self) -> Vec<Action> {
        self.utxos_by_addr_list.tick();

        let layout = self.last_layout.read().unwrap();

        if let Some(area) = layout.get(&ComponentId::LedgerBrowseOptions) {
            self.browse_options.model.set_height(area.height as usize);
        }
        if let Some(area) = layout.get(&ComponentId::LedgerSearchOptions) {
            self.search_options.model.set_height(area.height as usize);
        }
        if let Some(area) = layout.get(&ComponentId::LedgerAccountsList) {
            self.accounts_list.model.set_height(area.height as usize);
        }
        if let Some(area) = layout.get(&ComponentId::LedgerBlockIssuersList) {
            self.block_issuers_list
                .model
                .set_height(area.height as usize);
        }
        if let Some(area) = layout.get(&ComponentId::LedgerDRepsList) {
            self.dreps_list.model.set_height(area.height as usize);
        }
        if let Some(area) = layout.get(&ComponentId::LedgerPoolsList) {
            self.pools_list.model.set_height(area.height as usize);
        }
        if let Some(area) = layout.get(&ComponentId::LedgerProposalsList) {
            self.proposals_list.model.set_height(area.height as usize);
        }
        if let Some(area) = layout.get(&ComponentId::LedgerUtxosList) {
            self.utxos_list.model.set_height(area.height as usize);
        }

        Vec::new()
    }

    fn handle_search(&mut self, query: &str) {
        self.utxos_by_addr_list.handle_search(query);
    }

    fn render(&self, f: &mut Frame, s: &AppState, _layout: &ComponentLayout) {
        let layout = self.calculate_layout(f.area(), s);

        {
            let mut layout_guard = self.last_layout.write().unwrap();
            *layout_guard = layout.clone();
        }
        {
            let mut focus_guard = self.active_focus.write().unwrap();
            *focus_guard = s.layout_model.get_focus();
        }

        for (id, area) in layout.iter() {
            let area = *area;
            let is_focused = s.layout_model.is_focused(*id);

            match id {
                // --- Options ---
                ComponentId::LedgerBrowseOptions => {
                    self.browse_options.render(f, s, &layout);
                }
                ComponentId::LedgerSearchOptions => {
                    self.search_options.render(f, s, &layout);
                }

                // --- Lists ---
                ComponentId::LedgerAccountsList => {
                    self.accounts_list.render(f, s, &layout);
                }
                ComponentId::LedgerBlockIssuersList => {
                    self.block_issuers_list.render(f, s, &layout);
                }
                ComponentId::LedgerDRepsList => {
                    self.dreps_list.render(f, s, &layout);
                }
                ComponentId::LedgerPoolsList => {
                    self.pools_list.render(f, s, &layout);
                }
                ComponentId::LedgerProposalsList => {
                    self.proposals_list.render(f, s, &layout);
                }
                ComponentId::LedgerUtxosList => {
                    self.utxos_list.render(f, s, &layout);
                }
                ComponentId::LedgerUtxosByAddrList => {
                    self.utxos_by_addr_list.render(f, s, &layout);
                }

                // --- Details ---
                ComponentId::LedgerAccountDetails => {
                    let item = self.accounts_list.model.selected_item();
                    self.account_details
                        .render_with_data(f, area, is_focused, item);
                }
                ComponentId::LedgerBlockIssuerDetails => {
                    let item = self.block_issuers_list.model.selected_item();
                    self.block_details
                        .render_with_data(f, area, is_focused, item);
                }
                ComponentId::LedgerDRepDetails => {
                    let item = self.dreps_list.model.selected_item();
                    self.drep_details
                        .render_with_data(f, area, is_focused, item);
                }
                ComponentId::LedgerPoolDetails => {
                    let item = self.pools_list.model.selected_item();
                    self.pool_details
                        .render_with_data(f, area, is_focused, item);
                }
                ComponentId::LedgerProposalDetails => {
                    let item = self.proposals_list.model.selected_item();
                    self.proposal_details
                        .render_with_data(f, area, is_focused, item);
                }
                ComponentId::LedgerUtxoDetails => {
                    let item = self.utxos_list.model.selected_item();
                    self.utxo_details
                        .render_with_data(f, area, is_focused, item);
                }
                ComponentId::LedgerUtxosByAddrDetails => {
                    let item = self.utxos_by_addr_list.selected_item();
                    self.utxo_by_addr_details
                        .render_with_data(f, area, is_focused, item);
                }

                // Fallback
                _ => {
                    if let Some(child) = s.component_registry.get(id) {
                        child.render(f, s, &layout);
                    }
                }
            }
        }
    }
}

use crate::{
    ScreenMode,
    app_state::AppState,
    components::{
        Component, ComponentLayout, details::DetailsComponent, list::ListComponent,
        search_bar::SearchBarComponent, search_list::SearchListComponent, tabs::TabsComponent,
    },
    controller::{LayoutSpec, walk_layout},
    model::{
        layout::{MoveFocus, find_next_focus},
        ledger_search::LedgerUtxoProvider,
        list_view::ListModelView,
    },
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

    // Tabs
    mode_tabs: TabsComponent<LedgerMode>,
    // Details
    account_details: DetailsComponent<AccountItem>,
    block_details: DetailsComponent<BlockIssuerItem>,
    drep_details: DetailsComponent<DRepItem>,
    pool_details: DetailsComponent<PoolItem>,
    proposal_details: DetailsComponent<ProposalItem>,
    utxo_details: DetailsComponent<UtxoItem>,
    utxo_by_addr_details: DetailsComponent<UtxoItem>,

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
    search_bar: SearchBarComponent,
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
            mode_tabs: TabsComponent::new(ComponentId::LedgerModeTabs, true),

            // Details
            account_details: DetailsComponent::new(
                ComponentId::LedgerAccountDetails,
                "Account Details",
            ),
            block_details: DetailsComponent::new(
                ComponentId::LedgerBlockIssuerDetails,
                "Block Issuer Details",
            ),
            drep_details: DetailsComponent::new(ComponentId::LedgerDRepDetails, "DRep Details"),
            pool_details: DetailsComponent::new(ComponentId::LedgerPoolDetails, "Pool Details"),
            proposal_details: DetailsComponent::new(
                ComponentId::LedgerProposalDetails,
                "Proposal Details",
            ),
            utxo_details: DetailsComponent::new(ComponentId::LedgerUtxoDetails, "UTXO Details"),
            utxo_by_addr_details: DetailsComponent::new(
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

            // Search
            search_bar: SearchBarComponent::new(ComponentId::SearchBar),
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
            // Mode tabs
            ComponentId::LedgerModeTabs => self.mode_tabs.handle_event(event, area),

            // Search
            ComponentId::SearchBar => self.search_bar.handle_event(event, area),

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
        let ledger_mode = self.mode_tabs.selected();
        let screen_mode = s.screen_mode;

        let header_constraints = match ledger_mode {
            LedgerMode::Browse => vec![(Constraint::Fill(1), Left(ComponentId::LedgerModeTabs))],
            LedgerMode::Search => vec![
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

    fn handle_search(&mut self, query: &str) {
        self.utxos_by_addr_list.handle_search(query);
    }

    pub fn handle_navigation(&mut self, direction: MoveFocus) -> Vec<Action> {
        let layout = self.last_layout.read().unwrap();
        let active_focus = *self.active_focus.read().unwrap();

        if let Some(next) = find_next_focus(&layout, active_focus, direction) {
            *self.active_focus.write().unwrap() = next;
            return vec![Action::SetFocus(next)];
        }

        Vec::new()
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

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        let layout = self.last_layout.read().unwrap().clone();
        let mut active_focus = *self.active_focus.read().unwrap();

        let mut actions = crate::components::handle_container_event(
            &layout,
            &mut active_focus,
            event,
            area,
            |target_id, ev, child_area| self.dispatch_to_child(target_id, ev, child_area),
        );

        // Intercept SubmitSearch from children so it doesn't bubble to App
        if let Some(pos) = actions
            .iter()
            .position(|a| matches!(a, Action::SubmitSearch(_)))
            && let Action::SubmitSearch(query) = actions.remove(pos)
        {
            // Handle the logic locally
            self.handle_search(&query);
        }

        *self.active_focus.write().unwrap() = active_focus;

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

    fn render(&self, f: &mut Frame, s: &AppState, parent_layout: &ComponentLayout) {
        let my_area = parent_layout.get(&self.id).copied().unwrap_or(f.area());
        let my_layout = self.calculate_layout(my_area, s);

        {
            let mut layout_guard = self.last_layout.write().unwrap();
            *layout_guard = my_layout.clone();
        }

        let current_focus = *self.active_focus.read().unwrap();
        for (id, area) in my_layout.iter() {
            let area = *area;
            let is_focused = current_focus == *id;

            match id {
                // --- Mode tabs ---
                ComponentId::LedgerModeTabs => {
                    self.mode_tabs.render_focused(f, area, is_focused);
                }
                // --- Search bar ---
                ComponentId::SearchBar => {
                    self.search_bar.render_focused(f, area, is_focused);
                }

                // --- Options ---
                ComponentId::LedgerBrowseOptions => {
                    self.browse_options.render_focused(f, area, is_focused);
                }
                ComponentId::LedgerSearchOptions => {
                    self.search_options.render_focused(f, area, is_focused);
                }

                // --- Lists ---
                ComponentId::LedgerAccountsList => {
                    self.accounts_list.render_focused(f, area, is_focused);
                }
                ComponentId::LedgerBlockIssuersList => {
                    self.block_issuers_list.render_focused(f, area, is_focused);
                }
                ComponentId::LedgerDRepsList => {
                    self.dreps_list.render_focused(f, area, is_focused);
                }
                ComponentId::LedgerPoolsList => {
                    self.pools_list.render_focused(f, area, is_focused);
                }
                ComponentId::LedgerProposalsList => {
                    self.proposals_list.render_focused(f, area, is_focused);
                }
                ComponentId::LedgerUtxosList => {
                    self.utxos_list.render_focused(f, area, is_focused);
                }
                ComponentId::LedgerUtxosByAddrList => {
                    self.utxos_by_addr_list.render_focused(f, area, is_focused);
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
                _ => {}
            }
        }
    }
}

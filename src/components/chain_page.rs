use crate::{
    app_state::AppState,
    components::{
        Component, ComponentLayout, InputRoute, chain_search::ChainSearchComponent,
        route_event_to_children, search_bar::SearchBarComponent,
    },
    controller::{LayoutSpec, walk_layout},
    states::{Action, ComponentId},
    tui::Event,
};
use amaru_stores::rocksdb::consensus::ReadOnlyChainDB;
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

pub struct ChainPageComponent {
    id: ComponentId,
    pub search_bar: SearchBarComponent,
    pub chain_search: ChainSearchComponent,
    last_layout: RwLock<ComponentLayout>,
    active_focus: RwLock<ComponentId>,
}

impl ChainPageComponent {
    pub fn new(chain_db: Arc<ReadOnlyChainDB>) -> Self {
        Self {
            id: ComponentId::ChainPage,
            search_bar: SearchBarComponent::new(ComponentId::SearchBar),
            chain_search: ChainSearchComponent::new(ComponentId::ChainSearch, chain_db),
            last_layout: RwLock::new(HashMap::new()),
            active_focus: RwLock::new(ComponentId::SearchBar),
        }
    }

    pub fn handle_search(&mut self, query: &str) {
        self.chain_search.handle_search(query);
    }
}

impl Component for ChainPageComponent {
    fn id(&self) -> ComponentId {
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn calculate_layout(&self, area: Rect, _s: &AppState) -> ComponentLayout {
        let spec = LayoutSpec {
            direction: Direction::Vertical,
            constraints: vec![
                (
                    Constraint::Length(3),
                    Right(LayoutSpec {
                        direction: Direction::Horizontal,
                        constraints: vec![(Constraint::Fill(1), Left(ComponentId::SearchBar))],
                    }),
                ),
                (Constraint::Fill(1), Left(ComponentId::ChainSearch)),
            ],
        };

        let mut layout = HashMap::new();
        walk_layout(&mut layout, &spec, area);
        layout
    }

    fn route_event(&self, event: &Event, s: &AppState) -> InputRoute {
        let my_area = s
            .layout_model
            .get_layout()
            .get(&self.id)
            .copied()
            .unwrap_or(s.frame_area);
        let my_layout = self.calculate_layout(my_area, s);

        let route = route_event_to_children(event, s, my_layout);

        match route {
            InputRoute::Delegate(id, _) if id == self.id => InputRoute::Handle,
            InputRoute::Delegate(ComponentId::SearchBar | ComponentId::ChainSearch, _) => {
                InputRoute::Handle
            }
            _ => route,
        }
    }

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        let layout = self.last_layout.read().unwrap().clone();
        let mut active_focus = *self.active_focus.read().unwrap();
        let actions = crate::components::handle_container_event(
            &layout,
            &mut active_focus,
            event,
            area,
            |target_id, ev, child_area| {
                // Dispatch logic
                let mut acts = Vec::new();
                if target_id == ComponentId::SearchBar {
                    acts.extend(self.search_bar.handle_event(ev, child_area));
                } else if target_id == ComponentId::ChainSearch {
                    acts.extend(self.chain_search.handle_event(ev, child_area));
                }
                acts
            },
        );

        // Sync focus back
        *self.active_focus.write().unwrap() = active_focus;

        actions
    }

    fn render(&self, f: &mut Frame, s: &AppState, parent_layout: &ComponentLayout) {
        let my_area = parent_layout.get(&self.id).copied().unwrap_or(f.area());
        let my_layout = self.calculate_layout(my_area, s);

        {
            let mut layout_guard = self.last_layout.write().unwrap();
            *layout_guard = my_layout.clone();
        }

        if let Some(_rect) = my_layout.get(&ComponentId::SearchBar) {
            self.search_bar.render(f, s, &my_layout);
        }
        if let Some(_rect) = my_layout.get(&ComponentId::ChainSearch) {
            self.chain_search.render(f, s, &my_layout);
        }
    }
}

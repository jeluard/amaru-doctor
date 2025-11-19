use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, MouseScrollDirection, ScrollDirection},
    controller::{LayoutSpec, walk_layout},
    states::{Action, ComponentId},
};
use crossterm::event::KeyEvent;
use either::Either::{Left, Right};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
};
use std::{any::Any, collections::HashMap};

pub struct ChainPageComponent {
    id: ComponentId,
}

impl Default for ChainPageComponent {
    fn default() -> Self {
        Self {
            id: ComponentId::ChainPage,
        }
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
                // Header
                (
                    Constraint::Length(3),
                    Right(LayoutSpec {
                        direction: Direction::Horizontal,
                        constraints: vec![
                            (Constraint::Fill(1), Left(ComponentId::InspectTabs)),
                            (Constraint::Fill(1), Left(ComponentId::SearchBar)),
                        ],
                    }),
                ),
                // Body
                (
                    Constraint::Fill(1),
                    Right(LayoutSpec {
                        direction: Direction::Horizontal,
                        constraints: vec![
                            (Constraint::Fill(1), Left(ComponentId::ChainSearchHeader)),
                            (Constraint::Fill(1), Left(ComponentId::ChainSearchBlock)),
                            (Constraint::Fill(1), Left(ComponentId::ChainSearchNonces)),
                        ],
                    }),
                ),
            ],
        };

        let mut layout = HashMap::new();
        walk_layout(&mut layout, &spec, area);
        layout
    }

    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let my_area = layout.get(&self.id).copied().unwrap_or(f.area());
        let my_layout = self.calculate_layout(my_area, s);
        for (id, _) in my_layout.iter() {
            if let Some(child) = s.component_registry.get(id) {
                child.render(f, s, &my_layout);
            }
        }
    }
    fn handle_scroll(&mut self, _d: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_key_event(&mut self, _k: KeyEvent) -> Vec<Action> {
        Vec::new()
    }
    fn handle_click(&mut self, _a: Rect, _r: u16, _c: u16) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_scroll(&mut self, _d: MouseScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_drag(&mut self, _d: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
}

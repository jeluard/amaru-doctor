use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, InputRoute, route_event_to_children},
    controller::{LayoutSpec, walk_layout},
    states::ComponentId,
    tui::Event,
};
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
        route_event_to_children(event, s, my_layout)
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
}

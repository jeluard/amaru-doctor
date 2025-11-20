use crate::{
    app_state::AppState,
    states::{Action, ComponentId},
    tui::Event,
    update::scroll::ScrollDirection,
};
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use std::{any::Any, collections::HashMap};
use tracing::debug;

pub mod async_list;
pub mod chain_page;
pub mod chain_search;
pub mod details;
pub mod flame_graph;
pub mod ledger_page;
pub mod list;
pub mod otel_page;
pub mod prom_metrics;
pub mod prometheus_page;
pub mod root;
pub mod search_bar;
pub mod search_list;
pub mod stateful_details;
pub mod tabs;
pub mod trace_list;

pub type ComponentLayout = HashMap<ComponentId, Rect>;

#[derive(strum::Display, Debug, Clone, Copy)]
pub enum MouseScrollDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputRoute {
    /// I am not the target. Pass the event to this child ID.
    Delegate(ComponentId, Rect),
    /// I am the target. Please borrow me mutably and call handle_event().
    Handle,
    /// I don't want this event. Stop routing.
    Ignore,
}

pub trait Component {
    /// Returns the unique ID of this component instance.
    fn id(&self) -> ComponentId;

    /// Returns an Any refernce to the component.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable Any refernce to the component.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Called on every application tick.
    fn tick(&mut self) -> Vec<Action> {
        Vec::new()
    }

    /// Renders the component onto the frame.
    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout);

    /// Calculates the layout for this component and *all its children*.
    fn calculate_layout(&self, area: Rect, s: &AppState) -> ComponentLayout;

    fn route_event(&self, _event: &Event, _state: &AppState) -> InputRoute {
        InputRoute::Handle
    }

    fn handle_event(&mut self, _event: &Event, _area: Rect) -> Vec<Action> {
        Vec::new()
    }

    /// Handles a logical scroll event.
    fn handle_scroll(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }

    /// Handles a raw key event.
    fn handle_key_event(&mut self, _key: KeyEvent) -> Vec<Action> {
        Vec::new()
    }

    /// Handles a mouse click.
    fn handle_click(&mut self, _area: Rect, _row: u16, _col: u16) -> Vec<Action> {
        Vec::new()
    }

    /// Handles a mouse scroll event.
    fn handle_mouse_scroll(&mut self, _direction: MouseScrollDirection) -> Vec<Action> {
        Vec::new()
    }

    /// Handles a mouse drag event.
    fn handle_mouse_drag(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }

    fn handle_search(&mut self, _query: &str) {}
}

pub fn route_event_to_children(
    event: &Event,
    s: &AppState,
    my_layout: ComponentLayout,
) -> InputRoute {
    match event {
        Event::Key(_) => {
            let focus_id = s.layout_model.get_focus();
            if let Some(rect) = my_layout.get(&focus_id) {
                return InputRoute::Delegate(focus_id, *rect);
            }
            InputRoute::Delegate(focus_id, Rect::default())
        }

        Event::Mouse(mouse) => {
            for (child_id, rect) in &my_layout {
                let hit = mouse.column >= rect.x
                    && mouse.column < rect.x + rect.width
                    && mouse.row >= rect.y
                    && mouse.row < rect.y + rect.height;

                if hit {
                    debug!(
                        "Child {:?} at {:?} contains mouse ({}, {})",
                        child_id, rect, mouse.column, mouse.row
                    );
                    return InputRoute::Delegate(*child_id, *rect);
                }
            }
            debug!("No child contained mouse ({}, {})", mouse.column, mouse.row);
            InputRoute::Ignore
        }
        _ => InputRoute::Ignore,
    }
}

use crate::{
    app_state::AppState,
    states::{Action, ComponentId},
    tui::Event,
};
use crossterm::event::MouseButton;
use crossterm::event::MouseEventKind;
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
pub mod tabs;
pub mod trace_list;

pub type ComponentLayout = HashMap<ComponentId, Rect>;

#[derive(strum::Display, Debug, Clone, Copy)]
pub enum MouseScrollDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
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

    /// Calculates the layout for this component and *all its children*.
    fn calculate_layout(&self, area: Rect, s: &AppState) -> ComponentLayout;

    /// Called on every application tick.
    fn tick(&mut self) -> Vec<Action> {
        Vec::new()
    }

    fn handle_event(&mut self, _event: &Event, _area: Rect) -> Vec<Action> {
        Vec::new()
    }

    /// Renders the component onto the frame.
    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout);
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
                    return InputRoute::Delegate(*child_id, *rect);
                }
            }
            debug!("No child contained mouse ({}, {})", mouse.column, mouse.row);
            InputRoute::Ignore
        }
        _ => InputRoute::Ignore,
    }
}

/// A reusable event handler for Container Components.
pub fn handle_container_event<F>(
    layout: &HashMap<ComponentId, Rect>,
    active_focus: &mut ComponentId,
    event: &Event,
    area: Rect,
    mut dispatcher: F,
) -> Vec<Action>
where
    F: FnMut(ComponentId, &Event, Rect) -> Vec<Action>,
{
    let mut actions = Vec::new();

    let target_id = match event {
        Event::Mouse(mouse) => {
            // Find which child is under the mouse
            let hit = layout
                .iter()
                .filter(|(_, rect)| {
                    mouse.column >= rect.x
                        && mouse.column < rect.x + rect.width
                        && mouse.row >= rect.y
                        && mouse.row < rect.y + rect.height
                })
                // Pick the smallest rect to ensure we hit the leaf, not a container
                .min_by_key(|(_, rect)| rect.width * rect.height);

            if let Some((&child_id, _)) = hit {
                // "Focus Follows Mouse" Logic
                if mouse.kind == MouseEventKind::Moved && *active_focus != child_id {
                    actions.push(Action::SetFocus(child_id));
                    *active_focus = child_id;
                }
                // Click Logic (Ensure focus is set on click too)
                if mouse.kind == MouseEventKind::Down(MouseButton::Left)
                    && *active_focus != child_id
                {
                    actions.push(Action::SetFocus(child_id));
                    *active_focus = child_id;
                }

                child_id
            } else {
                // Missed all children? Fallback to current focus so keys still go somewhere.
                *active_focus
            }
        }
        Event::Key(_) => *active_focus,
        _ => *active_focus,
    };

    // If the child isn't in the layout (e.g. hidden tab), fallback to the full area
    let child_area = layout.get(&target_id).copied().unwrap_or(area);

    // Call the provided closure to delegate the actual work
    let child_actions = dispatcher(target_id, event, child_area);
    actions.extend(child_actions);

    actions
}

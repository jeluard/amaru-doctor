use crate::{
    app_state::AppState,
    states::{Action, ComponentId},
    update::scroll::ScrollDirection,
};
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use std::{any::Any, collections::HashMap};

pub mod async_list;
pub mod details;
pub mod list;
pub mod prom_metrics;
pub mod search_bar;
pub mod tabs;
pub mod trace_list;

pub type ComponentLayout = HashMap<ComponentId, Rect>;

#[derive(strum::Display, Debug, Clone, Copy)]
pub enum MouseScrollDirection {
    Up,
    Down,
}

pub trait Component {
    /// Returns the unique ID of this component instance.
    fn id(&self) -> ComponentId;

    /// Returns an Any refernce to the component.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable Any refernce to the component.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Renders the component onto the frame.
    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout);

    /// Calculates the layout for this component and *all its children*.
    fn calculate_layout(&self, area: Rect, s: &AppState) -> ComponentLayout;

    /// Handles a logical scroll event.
    fn handle_scroll(&mut self, _direction: ScrollDirection) -> Vec<Action>;

    /// Handles a raw key event.
    fn handle_key_event(&mut self, _key: KeyEvent) -> Vec<Action>;

    /// Handles a mouse click.
    fn handle_click(&mut self, _area: Rect, _row: u16, _col: u16) -> Vec<Action>;

    /// Handles a mouse scroll event.
    fn handle_mouse_scroll(&mut self, _direction: MouseScrollDirection) -> Vec<Action>;

    /// Handles a mouse drag event.
    fn handle_mouse_drag(&mut self, _direction: ScrollDirection) -> Vec<Action>;
}

use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, InputRoute, MouseScrollDirection, list::ListModel},
    states::{Action, ComponentId},
    tui::Event,
    update::scroll::ScrollDirection,
    view::empty_list::draw_empty_list,
};
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, MouseButton, MouseEventKind},
    layout::Rect,
};
use std::any::Any;
use tracing::{debug, warn};

type Accessor<M> = Box<dyn Fn(&AppState) -> Option<&M> + Send + Sync>;

pub struct ProxyListComponent<M> {
    id: ComponentId,
    accessor: Accessor<M>,
    empty_title: &'static str,
    empty_message: &'static str,
    last_drag_y: Option<u16>,
}

impl<M: ListModel> ProxyListComponent<M> {
    pub fn new(
        id: ComponentId,
        accessor: Accessor<M>,
        empty_title: &'static str,
        empty_message: &'static str,
    ) -> Self {
        Self {
            id,
            accessor,
            empty_title,
            empty_message,
            last_drag_y: None,
        }
    }
}

impl<M: ListModel> Component for ProxyListComponent<M> {
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
        let mut l = ComponentLayout::new();
        l.insert(self.id, area);
        l
    }

    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };
        let is_focused = s.layout_model.is_focused(self.id);

        if let Some(model) = (self.accessor)(s) {
            model.draw(f, area, is_focused);
        } else {
            draw_empty_list(f, area, self.empty_title, self.empty_message, is_focused);
        }
    }

    fn route_event(&self, _event: &Event, _state: &AppState) -> InputRoute {
        InputRoute::Handle
    }

    fn handle_event(&mut self, event: &Event, _area: Rect) -> Vec<Action> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Up => vec![Action::ScrollUp],
                KeyCode::Down => vec![Action::ScrollDown],
                _ => Vec::new(),
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => vec![Action::ScrollUp],
                MouseEventKind::ScrollDown => vec![Action::ScrollDown],

                MouseEventKind::Down(MouseButton::Left) => {
                    debug!(
                        "ProxyList: Mouse Down at row {}. Starting drag tracking.",
                        mouse.row
                    );
                    self.last_drag_y = Some(mouse.row);
                    vec![Action::MouseClick(mouse.column, mouse.row)]
                }

                MouseEventKind::Drag(MouseButton::Left) => {
                    let Some(last_y) = self.last_drag_y else {
                        warn!(
                            "ProxyList: Received Drag event but last_drag_y was None! Resetting to {}.",
                            mouse.row
                        );
                        self.last_drag_y = Some(mouse.row);
                        return Vec::new();
                    };

                    debug!(
                        "ProxyList: Dragging. Current: {}, Last: {}",
                        mouse.row, last_y
                    );

                    let actions = if mouse.row > last_y {
                        debug!("ProxyList: Drag > Last. Emitting MouseDragDown (Retreat)");
                        vec![Action::MouseDragDown]
                    } else if mouse.row < last_y {
                        debug!("ProxyList: Drag < Last. Emitting MouseDragUp (Advance)");
                        vec![Action::MouseDragUp]
                    } else {
                        Vec::new()
                    };

                    if !actions.is_empty() {
                        self.last_drag_y = Some(mouse.row);
                    }
                    actions
                }

                MouseEventKind::Up(_) => {
                    debug!("ProxyList: Mouse Up. Stopping drag tracking.");
                    self.last_drag_y = None;
                    Vec::new()
                }
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    }

    fn handle_scroll(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_key_event(&mut self, _k: KeyEvent) -> Vec<Action> {
        Vec::new()
    }
    fn handle_click(&mut self, _area: Rect, _row: u16, _col: u16) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_scroll(&mut self, _d: MouseScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_drag(&mut self, _d: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
}

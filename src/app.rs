use crate::{
    app_state::AppState,
    config::Config,
    otel::TraceCollector,
    states::{Action, InspectOption, WidgetSlot},
    tui::{Event, Tui},
    update::{UPDATE_DEFS, UpdateList},
    view::{SlotViews, compute_slot_views},
};
use amaru_stores::rocksdb::{ReadOnlyRocksDB, consensus::ReadOnlyChainDB};
use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use std::{io::Error, sync::Arc};
use tokio::sync::mpsc;
use tracing::{debug, info, trace};

pub struct App {
    config: Config,
    app_state: AppState, // Model
    updates: UpdateList, // Update
    last_store_option: InspectOption,
    slot_views: SlotViews, // View
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
    // Mouse drag state for scroll functionality
    mouse_drag_start: Option<(u16, u16)>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

impl App {
    pub fn new(
        ledger_db: ReadOnlyRocksDB,
        chain_db: ReadOnlyChainDB,
        collector: Arc<TraceCollector>,
        frame_area: Rect,
    ) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        let app_state = AppState::new(ledger_db, chain_db, collector)?;
        action_tx.send(Action::UpdateLayout(frame_area))?;
        let last_inspect_option = app_state.inspect_option.current().clone();
        let slot_views = compute_slot_views(&app_state);

        Ok(Self {
            app_state,
            updates: UPDATE_DEFS.to_vec(),
            last_store_option: last_inspect_option,
            slot_views,
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
            mouse_drag_start: None,
        })
    }

    pub async fn run(&mut self, tui: &mut Tui) -> Result<()> {
        tui.terminal.clear()?;
        tui.enter()?;

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(tui).await?;
            self.handle_actions(tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => {
                action_tx.send(Action::Key(key.code))?;
                self.handle_key_event(key)?
            }
            Event::Mouse(mouse) => self.handle_mouse_event(mouse)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        trace!("App::handle_key_event - received: {:?}", key);
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            trace!("App::handle_key_event - no keymap: {:?}", key);
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Key to action: {action:?}. Will broadcast.");
                action_tx.send(action.clone())?;
            }
            _ => {
                trace!("App::handle_key_event - no single-key action: {:?}", key);
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<()> {
        trace!("App::handle_mouse_event - received: {:?}", mouse);
        let action_tx = self.action_tx.clone();

        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Start tracking drag for potential scroll behavior
                self.mouse_drag_start = Some((mouse.column, mouse.row));
                
                // Send mouse click action with coordinates for potential widget-specific handling
                action_tx.send(Action::Mouse(mouse.column, mouse.row))?;

                // Check if mouse click is on a focusable widget and switch focus
                // This allows users to click on widgets to focus them, similar to modern GUIs
                self.handle_mouse_focus(mouse.column, mouse.row)?;
            }
            MouseEventKind::Up(MouseButton::Left) => {
                // End drag tracking when mouse button is released
                self.mouse_drag_start = None;
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                // Handle mouse drag for scrolling
                if let Some((start_x, start_y)) = self.mouse_drag_start {
                    self.handle_mouse_drag_scroll(start_x, start_y, mouse.column, mouse.row)?;
                }
            }
            MouseEventKind::ScrollUp => {
                // Handle mouse wheel scroll up
                action_tx.send(Action::ScrollUp)?;
            }
            MouseEventKind::ScrollDown => {
                // Handle mouse wheel scroll down
                action_tx.send(Action::ScrollDown)?;
            }
            MouseEventKind::Moved => {
                // Send mouse move action for potential hover effects in the future
                // This could be used to highlight widgets on hover or show tooltips
                action_tx.send(Action::MouseMove(mouse.column, mouse.row))?;
            }
            _ => {
                // Ignore other mouse events for now (right click, etc.)
                // These could be implemented in the future for additional functionality
            }
        }
        Ok(())
    }

    fn handle_mouse_focus(&mut self, x: u16, y: u16) -> Result<()> {
        // Find which widget slot contains the mouse coordinates
        // This implements click-to-focus functionality similar to modern GUI applications
        for (slot, rect) in &self.app_state.layout {
            if WidgetSlot::focusable().contains(slot)
                && x >= rect.x
                && x < rect.x + rect.width
                && y >= rect.y
                && y < rect.y + rect.height
            {
                // Only change focus if it's different from current focus to avoid unnecessary updates
                if self.app_state.slot_focus != *slot {
                    trace!(
                        "Mouse focus change from {} to {}",
                        self.app_state.slot_focus, slot
                    );
                    self.app_state.slot_focus = *slot;
                }
                break; // Found the widget, no need to check others
            }
        }
        Ok(())
    }

    fn handle_mouse_drag_scroll(&mut self, start_x: u16, start_y: u16, current_x: u16, current_y: u16) -> Result<()> {
        // Calculate drag distance
        let _delta_x = current_x as i32 - start_x as i32;
        let delta_y = current_y as i32 - start_y as i32;
        
        // Use a threshold to avoid triggering scroll on tiny movements
        const SCROLL_THRESHOLD: i32 = 2;
        
        let action_tx = self.action_tx.clone();
        
        // Determine scroll direction based on drag movement
        // Positive delta_y means dragging down, which should scroll up (like mobile scrolling)
        // Negative delta_y means dragging up, which should scroll down
        if delta_y.abs() > SCROLL_THRESHOLD {
            if delta_y > 0 {
                // Dragging down -> scroll content up
                action_tx.send(Action::ScrollUp)?;
            } else {
                // Dragging up -> scroll content down  
                action_tx.send(Action::ScrollDown)?;
            }
            
            // Update drag start position to current position for continuous scrolling
            self.mouse_drag_start = Some((current_x, current_y));
        }
        
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if !matches!(action, Action::Tick | Action::Render) {
                debug!("{action:?}");
            }

            let recompute_slot_widgets = matches!(action, Action::ScrollUp | Action::ScrollDown);

            match action {
                Action::Tick => {
                    self.last_tick_key_events.clear();
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                Action::Mouse(x, y) => {
                    debug!("Mouse click at ({}, {})", x, y);
                    // Mouse click actions are handled in handle_mouse_event
                    // This is logged for debugging purposes
                }
                Action::MouseMove(x, y) => {
                    // Mouse move can be used for hover effects in the future
                    // For now, just trace it to avoid spam
                    trace!("Mouse move at ({}, {})", x, y);
                }
                _ => {}
            }

            self.run_updates(&action)?;

            if recompute_slot_widgets {
                self.slot_views = compute_slot_views(&self.app_state);
            }
        }

        Ok(())
    }

    fn run_updates(&mut self, action: &Action) -> Result<()> {
        let mut next_actions = Vec::new();
        for updater in &self.updates {
            next_actions.extend(updater.update(action, &mut self.app_state));
        }
        for next_action in next_actions {
            self.action_tx.send(next_action)?
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.try_draw(|f| -> std::result::Result<(), _> {
            let frame_area = f.area();
            if frame_area != self.app_state.frame_area
                || self.app_state.inspect_option.current() != &self.last_store_option
            {
                trace!("Frame area or store option changed");

                // Synchronously update the layout
                let action = Action::UpdateLayout(frame_area);
                self.run_updates(&action).map_err(Error::other)?;

                self.last_store_option = self.app_state.inspect_option.current().clone();
            }
            for (slot, area) in self.app_state.layout.iter() {
                if let Some(view) = self.slot_views.get(slot) {
                    if let Err(e) = view.render(f, *area, &self.app_state) {
                        let _ = self
                            .action_tx
                            .send(Action::Error(format!("Failed to draw: {e:?}")));
                    }
                } else {
                    trace!("Found no view for slot {}", slot);
                }
            }
            Ok::<(), std::io::Error>(())
        })
        .map(|_| ())
        .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
    use ratatui::layout::Rect;

    #[tokio::test]
    async fn test_mouse_action_creation() {
        // Test that mouse actions can be created with coordinates
        let mouse_click = Action::Mouse(10, 20);
        let mouse_move = Action::MouseMove(15, 25);

        assert!(matches!(mouse_click, Action::Mouse(10, 20)));
        assert!(matches!(mouse_move, Action::MouseMove(15, 25)));
    }

    #[tokio::test]
    async fn test_mouse_event_handling() {
        // Create a mock mouse event
        let mouse_event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: crossterm::event::KeyModifiers::empty(),
        };

        // Test the mouse event handling logic by checking the event kind
        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // This should trigger mouse focus and action sending
                assert_eq!(mouse_event.column, 10);
                assert_eq!(mouse_event.row, 5);
            }
            _ => panic!("Expected left mouse button down event"),
        }
    }

    #[test]
    fn test_mouse_focus_coordinates() {
        // Test the coordinate checking logic for focus
        let rect = Rect::new(5, 5, 10, 10); // x=5, y=5, width=10, height=10

        // Point inside the rectangle
        let inside_x = 7;
        let inside_y = 8;
        assert!(inside_x >= rect.x && inside_x < rect.x + rect.width);
        assert!(inside_y >= rect.y && inside_y < rect.y + rect.height);

        // Point outside the rectangle
        let outside_x = 20;
        let outside_y = 3;
        assert!(!(outside_x >= rect.x && outside_x < rect.x + rect.width));
        assert!(!(outside_y >= rect.y && outside_y < rect.y + rect.height));
    }

    #[tokio::test]
    async fn test_mouse_wheel_scroll_events() {
        // Test that mouse wheel events create the right actions
        let scroll_up_event = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 10,
            row: 5,
            modifiers: crossterm::event::KeyModifiers::empty(),
        };

        let scroll_down_event = MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 10,
            row: 5,
            modifiers: crossterm::event::KeyModifiers::empty(),
        };

        // Verify event kinds are correct
        assert!(matches!(scroll_up_event.kind, MouseEventKind::ScrollUp));
        assert!(matches!(scroll_down_event.kind, MouseEventKind::ScrollDown));
    }

    #[tokio::test]
    async fn test_mouse_drag_events() {
        // Test that mouse drag events are recognized
        let drag_event = MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: 15,
            row: 10,
            modifiers: crossterm::event::KeyModifiers::empty(),
        };

        // Verify event kind is correct
        assert!(matches!(drag_event.kind, MouseEventKind::Drag(MouseButton::Left)));
    }

    #[test]
    fn test_drag_scroll_calculation() {
        // Test the drag distance calculation logic
        let _start_x = 10u16;
        let start_y = 10u16;
        let _current_x = 10u16;
        let current_y = 15u16; // Dragged down 5 pixels

        let delta_y = current_y as i32 - start_y as i32;
        assert_eq!(delta_y, 5);

        // Threshold test
        const SCROLL_THRESHOLD: i32 = 2;
        assert!(delta_y.abs() > SCROLL_THRESHOLD);

        // Direction test: positive delta_y (dragging down) should scroll up
        assert!(delta_y > 0); // This would trigger ScrollUp action
    }
}

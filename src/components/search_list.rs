use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, async_list::AsyncListModel, list::ListModel},
    model::search::SearchCache,
    states::{Action, ComponentId},
    tui::Event,
    ui::to_list_item::ToListItem,
    view::empty_list::draw_empty_list,
};
use crossterm::event::{KeyCode, MouseButton, MouseEventKind};
use ratatui::prelude::*;
use std::{any::Any, hash::Hash, str::FromStr};

pub type SearchFactory<R> = Box<dyn Fn(&str) -> Option<AsyncListModel<R>> + Send + Sync>;

pub struct SearchListComponent<Q, R>
where
    Q: Clone + Eq + Hash + FromStr + Send + Sync + 'static,
    R: ToListItem + Send + Sync + 'static,
{
    id: ComponentId,
    state: SearchCache<Q, AsyncListModel<R>>,
    title: &'static str,
    search_factory: SearchFactory<R>,
    last_drag_y: Option<u16>,
}

impl<Q, R> SearchListComponent<Q, R>
where
    Q: Clone + Eq + Hash + FromStr + Send + Sync + 'static,
    R: ToListItem + Send + Sync + 'static,
{
    pub fn new(id: ComponentId, title: &'static str, search_factory: SearchFactory<R>) -> Self {
        Self {
            id,
            state: SearchCache::default(),
            title,
            search_factory,
            last_drag_y: None,
        }
    }

    pub fn selected_item(&self) -> Option<&R> {
        self.state.get_current_res().and_then(|m| m.selected_item())
    }
}

impl<Q, R> Component for SearchListComponent<Q, R>
where
    Q: Clone + Eq + Hash + FromStr + Send + Sync + 'static,
    R: ToListItem + Send + Sync + 'static,
{
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

    fn tick(&mut self) -> Vec<Action> {
        if let Some(model) = self.state.get_current_res_mut() {
            model.poll_data();
        }
        Vec::new()
    }

    fn handle_search(&mut self, query_str: &str) {
        // Check cache
        if let Ok(query) = Q::from_str(query_str)
            && self.state.results.contains_key(&query)
        {
            self.state.parsed = Some(query);
            return;
        }
        // Create new model
        if let Some(model) = (self.search_factory)(query_str)
            && let Ok(query) = Q::from_str(query_str)
        {
            self.state.cache_result(query, model);
        }
    }

    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };
        let is_focused = s.layout_model.is_focused(self.id);

        if let Some(model) = self.state.get_current_res() {
            model.draw(f, area, is_focused);
        } else {
            draw_empty_list(f, area, self.title, "No results", is_focused);
        }
    }

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        let Some(model) = self.state.get_current_res_mut() else {
            return Vec::new();
        };

        model.set_height(area.height as usize);

        match event {
            Event::Key(key) => match key.code {
                KeyCode::Up => model.cursor_back(),
                KeyCode::Down => model.cursor_next(),
                _ => {}
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => model.cursor_back(),
                MouseEventKind::ScrollDown => model.cursor_next(),

                MouseEventKind::Down(MouseButton::Left) => {
                    self.last_drag_y = Some(mouse.row);
                    let relative_row = mouse.row.saturating_sub(area.y + 1) as usize;
                    model.select_index_by_row(relative_row);
                }

                MouseEventKind::Drag(MouseButton::Left) => {
                    let Some(last_y) = self.last_drag_y else {
                        self.last_drag_y = Some(mouse.row);
                        return Vec::new();
                    };
                    if mouse.row > last_y {
                        self.last_drag_y = Some(mouse.row);
                        model.retreat_window(); // Drag Down -> Retreat
                    } else if mouse.row < last_y {
                        self.last_drag_y = Some(mouse.row);
                        model.advance_window(); // Drag Up -> Advance
                    }
                }

                MouseEventKind::Up(_) => {
                    self.last_drag_y = None;
                }
                _ => {}
            },
            _ => {}
        }

        Vec::new()
    }
}

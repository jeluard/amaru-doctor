use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, list::ListComponent},
    states::{Action, ComponentId, WidgetSlot},
    ui::{ToRichText, to_list_item::ToListItem},
    view::item_details::draw_details,
};
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use std::{any::Any, marker::PhantomData};

pub struct DetailsComponent<T>
where
    T: ToRichText + ToListItem + Send + Sync + 'static,
{
    id: ComponentId,
    slot: WidgetSlot,
    title: &'static str,
    sibling_list_id: ComponentId,
    _phantom: PhantomData<T>,
}

impl<T> DetailsComponent<T>
where
    T: ToRichText + ToListItem + Send + Sync + 'static,
{
    pub fn new(
        id: ComponentId,
        slot: WidgetSlot,
        title: &'static str,
        sibling_list_id: ComponentId,
    ) -> Self {
        Self {
            id,
            slot,
            title,
            sibling_list_id,
            _phantom: PhantomData,
        }
    }
}

impl<T> Component for DetailsComponent<T>
where
    T: ToRichText + ToListItem + Send + Sync + 'static,
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
        let mut layout = ComponentLayout::new();
        layout.insert(self.id, area);
        layout
    }

    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };
        let is_focused = s.layout_model.is_focused(self.slot);

        let list_component = s
            .component_registry
            .get(&self.sibling_list_id)
            .and_then(|c| c.as_any().downcast_ref::<ListComponent<T>>())
            .expect("Details component could not find or downcast sibling list");

        let selected_item = list_component.view.selected_item();
        draw_details(f, area, self.title.to_string(), selected_item, is_focused);
    }

    fn handle_scroll(&mut self, _direction: crate::components::ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_key_event(&mut self, _key: KeyEvent) -> Vec<Action> {
        Vec::new()
    }
    fn handle_click(&mut self, _area: Rect, _row: u16, _col: u16) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_scroll(
        &mut self,
        _direction: crate::components::MouseScrollDirection,
    ) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_drag(&mut self, _direction: crate::components::ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
}

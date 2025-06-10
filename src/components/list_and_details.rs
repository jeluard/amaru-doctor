use std::rc::Rc;

use crate::{
    app_state::AppState,
    components::{details::DetailsComponent, list::ListComponent},
    shared::Shared,
    states::SlotSelection,
    ui::{to_list_item::ToListItem, to_rich::ToRichText},
    window::WindowState,
};

pub fn new_list_detail_components<T>(
    comp_id_1: SlotSelection,
    comp_id_2: SlotSelection,
    window: Shared<WindowState<T>>,
    app_state: Shared<AppState>,
) -> (ListComponent<T>, DetailsComponent<T>)
where
    T: Clone + ToListItem + ToRichText,
{
    let list = ListComponent::from_iter(comp_id_1, window.clone(), app_state.clone());
    let detail = DetailsComponent::new(comp_id_2, window.clone(), app_state.clone());
    (list, detail)
}

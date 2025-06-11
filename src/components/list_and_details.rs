use crate::{
    app_state::AppState,
    components::{details::DetailsComponent, list::ListComponent},
    shared::Shared,
    states::WidgetId,
    ui::{to_list_item::ToListItem, to_rich::ToRichText},
    window::WindowState,
};

pub fn new_list_detail_components<T>(
    comp_id_1: WidgetId,
    comp_id_2: WidgetId,
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

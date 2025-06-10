use crate::{
    components::{details::DetailsComponent, list::ListComponent, r#static::entity_types::Entity},
    shared::Shared,
    ui::{to_list_item::ToListItem, to_rich::ToRichText},
    window::WindowState,
};

pub fn new_list_detail_components<T>(
    entity: Entity,
    state: Shared<WindowState<T>>,
) -> (ListComponent<T>, DetailsComponent<T>)
where
    T: Clone + ToListItem + ToRichText,
{
    let list = ListComponent::from_iter(entity.clone(), state.clone());
    let detail = DetailsComponent::new(
        format!("{} Details", serde_plain::to_string(&entity).unwrap()),
        state.clone(),
    );
    (list, detail)
}

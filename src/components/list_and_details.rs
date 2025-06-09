use crate::{
    components::{details::DetailsComponent, list::ListComponent},
    focus::FocusableComponent,
    shared::{Shared, shared},
    to_list_item::ToListItem,
    to_rich::ToRichText,
};

pub fn new_list_detail_components<T, I>(
    item_name: &'static str,
    iter: I,
) -> (
    Shared<dyn FocusableComponent>,
    Shared<dyn FocusableComponent>,
)
where
    T: Clone + ToListItem + ToRichText + 'static,
    I: Iterator<Item = T> + 'static,
{
    let list = shared(ListComponent::from_iter(format!("{}s", item_name), iter));
    let detail = shared(DetailsComponent::new(
        format!("{} Details", item_name),
        list.clone(),
    ));
    (list, detail)
}

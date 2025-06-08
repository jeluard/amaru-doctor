use crate::{
    components::{details::DetailsComponent, list::ListComponent},
    focus::FocusableComponent,
    shared::{Shared, shared},
    to_list_item::ToListItem,
    to_rich::ToRichText,
    window::IteratorSource,
};
use std::rc::Rc;

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
    let source = Rc::new(IteratorSource::new(iter));

    let list = shared(ListComponent::new(
        format!("{}s", item_name),
        source.clone(),
        10,
    ));
    let detail = shared(DetailsComponent::new(
        format!("{} Details", item_name),
        list.clone(),
    ));

    (
        list as Shared<dyn FocusableComponent>,
        detail as Shared<dyn FocusableComponent>,
    )
}

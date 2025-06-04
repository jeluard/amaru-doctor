use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::{Shared, shared},
    to_list_item::ToListItem,
    to_rich::ToRichText,
};

pub fn new_list_detail_components<'a, T, I>(
    item_name: &'static str,
    iter: I,
) -> (
    Shared<'a, ScrollableListComponent<'a, T>>,
    Shared<'a, DetailsComponent<'a, T>>,
)
where
    T: Clone + ToListItem + ToRichText + 'a,
    I: Iterator<Item = T> + 'a,
{
    let list = shared(ScrollableListComponent::new(
        format!("{}s", item_name),
        iter,
        10,
    ));
    let detail = shared(DetailsComponent::new(
        format!("{} Details", item_name),
        list.clone(),
    ));
    (list, detail)
}

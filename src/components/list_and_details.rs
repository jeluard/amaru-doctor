use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    focus::FocusableComponent,
    shared::{Shared, shared},
    to_list_item::ToListItem,
    to_rich::ToRichText,
    window::WindowSource,
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub struct IteratorSource<T, I>
where
    I: Iterator<Item = T>,
{
    iter: RefCell<I>,
    buffer: RefCell<Vec<T>>,
}

impl<T, I> IteratorSource<T, I>
where
    I: Iterator<Item = T>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: RefCell::new(iter),
            buffer: RefCell::new(Vec::new()),
        }
    }
}

impl<T, I> WindowSource<T> for IteratorSource<T, I>
where
    I: Iterator<Item = T>,
{
    fn view(&self, start: usize, size: usize) -> Ref<[T]> {
        {
            let mut iter = self.iter.borrow_mut();
            let mut buf = self.buffer.borrow_mut();
            while buf.len() < start + size {
                if let Some(item) = iter.next() {
                    buf.push(item);
                } else {
                    break;
                }
            }
        }
        Ref::map(self.buffer.borrow(), move |v| {
            let end = (start + size).min(v.len());
            &v[start..end]
        })
    }

    fn len(&self) -> usize {
        usize::MAX
    }
}

pub fn new_list_detail_components<'a, T, I>(
    item_name: &'static str,
    iter: I,
) -> (
    Shared<'a, dyn FocusableComponent + 'a>,
    Shared<'a, dyn FocusableComponent + 'a>,
)
where
    T: Clone + ToListItem + ToRichText + 'a,
    I: Iterator<Item = T> + 'a,
{
    let source: Rc<dyn WindowSource<T> + 'a> = Rc::new(IteratorSource::new(iter));
    let list = shared(ScrollableListComponent::new(
        format!("{}s", item_name),
        source,
        10,
    ));
    let detail = shared(DetailsComponent::new(
        format!("{} Details", item_name),
        list.clone(),
    ));
    (list, detail)
}

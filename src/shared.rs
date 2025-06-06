use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub type Shared<'a, T> = Rc<RefCell<T>>;

pub fn shared<'a, T: 'a>(t: T) -> Shared<'a, T> {
    Rc::new(RefCell::new(t))
}

pub trait Getter<T> {
    fn get(&self) -> Option<Ref<T>>;
}

pub type SharedGetter<'a, T> = Shared<'a, dyn Getter<T> + 'a>;

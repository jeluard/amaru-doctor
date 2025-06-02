use std::cell::RefCell;
use std::rc::Rc;

pub type Shared<'a, T> = Rc<RefCell<T>>;

pub fn shared<'a, T: 'a>(t: T) -> Shared<'a, T> {
    Rc::new(RefCell::new(t))
}

pub trait Getter<T> {
    fn get_mut(&mut self) -> Option<T>;
}

pub type SharedGetter<'a, T> = Shared<'a, dyn Getter<T> + 'a>;

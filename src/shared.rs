use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub type Shared<T> = Rc<RefCell<T>>;

pub fn shared<T>(t: T) -> Shared<T> {
    Rc::new(RefCell::new(t))
}

pub trait Getter<T> {
    fn get(&self) -> Option<Ref<T>>;
}

pub type SharedGetter<T> = Shared<dyn Getter<T>>;

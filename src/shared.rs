use std::{cell::RefCell, rc::Rc};

use crate::focus::FocusableComponent;

pub type Shared<T> = Rc<RefCell<T>>;

pub type SharedGetter<T> = Shared<dyn Getter<T>>;

pub type SharedFC = Shared<dyn FocusableComponent>;

pub fn shared<T>(t: T) -> Shared<T> {
    Rc::new(RefCell::new(t))
}

pub trait Getter<T> {
    fn get(&self) -> &T;
}

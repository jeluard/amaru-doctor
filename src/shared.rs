use crate::components::Component;
use std::{cell::RefCell, rc::Rc};

pub type Shared<T> = Rc<RefCell<T>>;

pub type SharedComp = Shared<dyn Component>;

pub fn shared<T>(t: T) -> Shared<T> {
    Rc::new(RefCell::new(t))
}

pub trait GetterOpt<T> {
    fn get(&self) -> Option<&T>;
}

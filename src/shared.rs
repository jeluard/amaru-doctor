use std::{cell::RefCell, rc::Rc};

use crate::focus::FocusableComponent;

pub type Shared<T> = Rc<RefCell<T>>;

pub type SharedFC = Shared<dyn FocusableComponent>;

pub fn shared<T>(t: T) -> Shared<T> {
    Rc::new(RefCell::new(t))
}

pub trait GetterOpt<T> {
    fn get(&self) -> Option<&T>;
}

pub type SharedGetterOpt<T> = Shared<dyn GetterOpt<T>>;

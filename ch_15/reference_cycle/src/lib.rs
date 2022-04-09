use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

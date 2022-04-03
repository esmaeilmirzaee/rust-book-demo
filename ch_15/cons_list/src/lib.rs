use std::ops::Deref;

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

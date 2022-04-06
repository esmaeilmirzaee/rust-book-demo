use std::rc::Rc;

pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom smart pointer. {}", self.data);
    }
}

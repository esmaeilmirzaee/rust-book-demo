use std::cell::RefCell;
use std::rc::Rc;
use reference_cycle::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(10, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count: {}", Rc::strong_count(&a));
    println!("a next item is {:?}", a.tail());

    let b = Rc::new(Cons(9, RefCell::new(Rc::clone(&a))));
    println!("b initial rc count: {} \ta clone rc count: {}", Rc::strong_count(&b),
             Rc::strong_count(&a));
    println!("b next item is {:?}\ta next item is {:?}", b.tail(), a.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item is {:?}", a.tail());
}

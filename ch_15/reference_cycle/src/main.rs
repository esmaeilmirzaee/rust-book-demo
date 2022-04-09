use std::cell::RefCell;
use std::rc::{Rc, Weak};
use reference_cycle::{Node, List};

fn main() {
    let leaf = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 9,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent {:?}", leaf.parent.borrow().upgrade());
}

fn _cycling() {
    let a = Rc::new(List::Cons(10, RefCell::new(Rc::new(List::Nil))));
    println!("a initial rc count: {}", Rc::strong_count(&a));
    println!("a next item is {:?}", a.tail());

    let b = Rc::new(List::Cons(9, RefCell::new(Rc::clone(&a))));
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

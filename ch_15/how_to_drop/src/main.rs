use std::rc::Rc;

use how_to_drop::CustomSmartPointer;
use how_to_drop::List::{Cons, Nil};

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Some String"),
    };
    drop(c);

    // println!("Custom smart pointer dropped. {}", c.data);

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Counting after creating a {}", Rc::strong_count(&a));
    let b = Rc::new(Cons(3, Rc::new(Cons(4, Rc::clone(&a)))));
    println!("Counting after creating b {}", Rc::strong_count(&a));
    {
        let d = Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Cons(4, Rc::clone(&a)))))));
        println!("Counting after creating d {}", Rc::strong_count(&a));
    }
    println!("Counting after dropping d {}", Rc::strong_count(&a));
}

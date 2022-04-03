use cons_list::List::{Cons, Nil};
use cons_list::MyBox;
use std::ops::Deref;

fn main() {
    println!("Hello world");
    let _list = Box::new(Cons(1, Box::new(Cons(2, Box::new
        (Cons(3, Box::new(Nil))))
    )));

    // Deref section 15.2
    let a = 5;
    let b = Box::new(a);
    let c = &a;
    let d = &b;
    let e = MyBox::new(a);

    println!("{} {} {} {} {}", a, *b, c, *d, *(e.deref()));
}

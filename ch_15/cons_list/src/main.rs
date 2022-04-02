use cons_list::List::{Cons, Nil};

fn main() {
    let _list = Box::new(Cons(1, Box::new(Cons(2, Box::new
        (Cons(3, Box::new(Nil))))
    )));
    // println!("{}", list);

}

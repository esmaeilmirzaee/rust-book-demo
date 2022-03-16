fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let _five = plus_one(Some(4));
    let _none = plus_one(None);
    // println!("{}", five);
}

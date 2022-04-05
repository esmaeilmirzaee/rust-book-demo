use how_to_drop::CustomSmartPointer;

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Some String"),
    };
    drop(c);

    println!("Custom smart pointer dropped. {}", c.data);
}

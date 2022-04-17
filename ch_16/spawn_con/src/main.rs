use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    thread::spawn(move || {
        // move helps to take ownership of v
        // and it won't be accessible in main again.
        println!("printing v: {:?}", v);
    });

    // uncomment the following line to recieve errors.
    // drop(v);

    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} spawned from thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} spread in main.", i);
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();
}

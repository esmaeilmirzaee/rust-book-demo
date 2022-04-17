1. Handling concurrent programming safely and efficiently is another of Rust’s major goals. _Concurrent programming_, where different parts of a program execute independently, and _parallel programming_, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors. Historically, programming in these contexts has been difficult and error prone: Rust hopes to change that.

2. Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.

3. In most current operating systems, an executed program’s code is run in a _process_, and the operating system manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called _threads_.

4. Threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

• Race conditions, where threads are accessing data or resources in an inconsistent order

• Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing

• Bugs that happen only in certain situations and are hard to reproduce and fix reliably

5. Programming languages implement threads in a few different ways. Many operating systems provide an API for creating new threads. This model where a language calls the operating system APIs to create threads is sometimes called _1:1_, meaning one operating system thread per one language thread. The Rust standard library only provides an implementation of 1:1 threading; there are crates that implement other models of threading that make different tradeoffs.

```rust
use std::thread;
use std::time::Duration;

fn main() {
	// this is closure
	thread::spawn(|| {
		for i in 1..10 {
			println!("thread {}", i);
			thread::sleep(Duration::from_millis(1));
		}
	});
	
	for i in 1..5 {
		println!("Main thread {}", i);
		thread::sleep(Duration::from_millis(1));
	}
}
```

> Note that the above function ends earlier than it could finish to execute all the required steps inside the `thread::spawn`. **The calls to `thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run.**

```rust
use std::thread;
use std::time::Duration;

fn main() {
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("thread {}", i);
		}
	});
	
	// uncomment the following line and check it's imapct
	// handle.join().unwrap();
	
	for i in 1..5 {
		println!("main thread {}", i);
	}
	
	// prevent finish without completing the execution of all threads.
	// comment the following line and uncomment the above join command.
	handle.join().unwrap();
}
```

6. We can fix the problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of `thread::spawn` in a variable. The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned value that, when we call the `join` method on it, will wait for its thread to finish.
7. Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates. _Blocking_ a thread means that thread is prevented from performing work or exiting. Small details, such as where `join` is called, can affect whether or not your threads run at the same time.
8. The `move` keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.
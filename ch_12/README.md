1. To enable **minigrep** to read the values of command line arguments we pass to it, we’ll need a function provided in _Rust_’s standard library, which is `std::env::args`. This function returns an iterator of the command line arguments that were given to **minigrep**. For now, you only need to know two details about `iterators`: `iterators` produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a `vector`, containing all the elements the iterator produces.
2. Note that `std::env::args` will panic if any argument contains invalid _Unicode_. If your program needs to accept arguments containing invalid _Unicode_, use `std::env::args_os` instead. That function returns an iterator that produces `OsString` values instead of `String` values. We’ve chosen to use `std::env::args` here for simplicity, because `OsString` values differ per platform and are more complex to work with than `String` values.
3. Although we very rarely need to annotate types in _Rust_, `collect` is one function you do often need to annotate because _Rust_ isn’t able to infer the kind of collection you want.
4. The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the _Rust_ community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:

    4.1. Split your program into a `main.rs` and a `lib.rs` and move your program’s logic to `lib.rs`.
    4.2. As long as your command line parsing logic is small, it can remain in `main.rs`.
    4.3. When the command line parsing logic starts getting complicated, extract it from `main.rs` and move it to `lib.rs`.
5. We could manage the `String` data in a number of different ways, but the easiest, though somewhat inefficient, route is to call the `clone` method on the values. This will make a full copy of the data for the `Config` instance to own, which takes more time and memory than storing a reference to the string data.
6. A nonzero exit status is a convention to signal to the process that called our program that the program exited with an error state.
7. we’ve used a method we haven’t covered in detail yet: `unwrap_or_else`, which is defined on `Result<T, E>` by the standard library. Using `unwrap_or_else` allows us to define some custom, `non-panic!` error handling. If the `Result` is an `Ok` value, this method’s behavior is similar to `unwrap`: it returns the inner value `Ok` is wrapping. However, if the value is an `Err` value, this method calls the code in the `closure`, which is an anonymous function we define and pass as an `argument` to `unwrap_or_else`.
8. Just know that `Box<dyn Error>` means the function will return a type that implements the `Error` trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. The `dyn` keyword is short for “dynamic.”
8. Rather than `panic!` on an error, `?` will return the error value from the current function for the caller to handle.
9. Third, the `run` function now returns an `Ok` value in the success case. We’ve declared the `run` function’s success type as `()` in the signature, which means we need to wrap the unit type value in the `Ok` value. This `Ok(())` syntax might look a bit strange at first, but using `()` like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.
10. We use if let rather than `unwrap_or_else` to check whether run returns an `Err` value and call `process::exit(1)` if it does. The run function doesn’t return a value that we want to `unwrap` in the same way that `Config::new` returns the `Config` instance. Because run returns `()` in the success case, we only care about detecting an error, so we don’t need `unwrap_or_else` to return the unwrapped value because it would only be `()`.
11. Test Driven Development software development technique follows these steps:
    11.1. Write a test that fails and run it to make sure it fails for the reason you expect.
    11.2. Write or modify just enough code to make the new test pass.
    11.3. Refactor the code you just added or changed and make sure the tests continue to pass.
    11.4. Repeat from step 1!

```rust
let content = "\
Rust:
Safe, Fast, Productive.
Pick three.";
```
12. Note that the backslash after the opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal.
13. The standard library provides the `eprintln!` _macro_ that prints to the standard error stream. This demonstrates that we’re now using standard output for successful output and standard error for error output as appropriate.
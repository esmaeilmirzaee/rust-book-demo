1. _Rust_’s type system **shoulders a huge part of this burden**, but the type system cannot catch every kind of incorrectness.
2. The bodies of test functions typically perform these three actions:
   2.1. Set up any needed data or state.
   2.2. Run the code you want to test.
   2.3. Assert the results are what you expect.
3. At its simplest, a `test` in _Rust_ is a function that’s _annotated_ with the `test` attribute. **Attributes are metadata about pieces of _Rust_ code**; one example is the `derive` attribute we used with structs in Chapter 5. **To change a function into a test function**, add `#[test]` on the line before `fn`. When you run your tests with the `cargo test` command, _Rust_ builds a test runner binary that runs the functions _annotated_ with the test attribute and reports on whether each test function passes or fails.
4. `cargo new adder --lib` creates a new library project containing a simple `test`.

```rust
// src/lib.rs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

3. The `0 measured` statistic is for _benchmark_ tests that **measure performance**. _Benchmark_ tests are, as of this writing, only available in _nightly Rust_.
4. _Rust_ can compile any code examples that appear in our _API documentation_. This feature helps us keep our docs and our code in sync!
5. Tests fail when something in the test function `panics`. **Each test is run in a new thread**, and when **the main thread sees that a test thread has died**, **the test is marked as failed**.
6. The `assert!` macro, provided by the standard library, is useful when you want to ensure that some condition in a `test` evaluates to `true`. We give the `assert!` macro an argument that evaluates to a Boolean. If the value is true, `assert!` does nothing and the `test` passes. If the value is `false`, the `assert!` macro calls the `panic!` macro, which causes the `test` to fail. Using the `assert!` macro helps us check that our code is functioning in the way we intend.

```rust
// src/lib.rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let r1 = Rectangle {
            width: 8,
            height: 2,
        };
        let r2 = Rectangle {
            width: 5,
            height: 3,
        };
        assert!(r1.can_hold(&r2));
        assert!(!r2.can_hold(&r1));
    }
}
```

7. Note that we’ve added a new line inside the tests module: `use super::*;`. The tests module is a regular module that follows the usual visibility rules we covered in Chapter 7 in the “Paths for Referring to an Item in the Module Tree” section. Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this tests module.
8. A common way to test functionality is to compare the result of the code under test to the value you expect the code to return to make sure they’re equal. You could do this using the `assert!` macro and passing it an expression using the `==` operator. However, this is such a common test that the standard library provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test more conveniently. These macros compare two arguments for equality or inequality, respectively. They’ll also print the two values if the assertion fails, which makes it easier to see why the test failed; conversely, the `assert!` macro only indicates that it got a false value for the `==` expression, not the values that led to the false value.
9. Note that in some languages and test frameworks, the parameters to the functions that `assert` two values are equal are called `expected` and `actual`, and the order in which we specify the arguments matters. However, in _Rust_, they’re called `left` and `right`, and **the order in which we specify the value we expect and the value that the code under test produces doesn’t matter**. We could write the assertion in this test as `assert_eq!(add_two(2), 4)`, which would result in a failure message that displays assertion failed: `(left == right)` and that `left` was `5` and `right` was `4`.
10. The `assert_ne!` macro will pass if the two values we give it are not equal and fail if they’re equal. This macro is most useful for cases when we’re not sure what a value will be, but we know what the value definitely won’t be if our code is functioning as we intend. For example, if we’re testing a function that is guaranteed to change its input in some way, but the way in which the input is changed depends on the day of the week that we run our tests, the best thing to assert might be that the output of the function is not equal to the input.
11. Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators `==` and `!=`, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits. All the primitive types and most of the standard library types implement these traits. For `structs` and `enums` that you define, you’ll need to implement `PartialEq` to assert that values of those types are equal or not equal. You’ll need to implement `Debug` to print the values when the assertion fails. Because both traits are `derivable traits` this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition.
12. You can also add a custom message to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Any arguments specified after the one required argument to `assert!` or the two required arguments to `assert_eq!` and `assert_ne!` are passed along to the `format!` macro so you can pass a format string that contains `{}` placeholders and values to go in those placeholders. Custom messages are useful to document what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.

```rust 
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value >= 1 {
            panic!("Expected greater than or equal to 1, but got {}", value);
        } else if value <= 100 {
            panic!("Expected less than or equal to 100, but got {}.", value);
        }
    }

    Guess {
        value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected="Expected greater than or equal to 1")]
    fn test_guess() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected="Expected less than or equal to 100")]
    fn test_guess_less() {
        Guess::new(200);
    }
}

```
13. Tests that use `should_panic` can be imprecise because they only indicate that the code has caused some `panic`. A `should_panic` **test would pass even if the test panics for a different reason from the one we were expecting to happen**. To make `should_panic` tests more precise, we can add an optional expected parameter to the `should_panic` attribute. The test harness will make sure that the failure message contains the provided text.
14. This test will pass because the value we put in the should_panic attribute’s expected parameter is a substring of the message that the `Guess::new` function panics with. We could have specified the entire panic message that we expect, which in this case would be Guess value must be less than or equal to `100`, got `200`. What you choose to specify in the expected parameter for `should_panic` depends on how much of the panic message is unique or dynamic and how precise you want your test to be. In this case, a substring of the panic message is enough to ensure that the code in the test function executes the `else if value > 100` case.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2+2 does not equal to 4"))
        }
    }
}
```
15. The `it_works` function now has a return type, `Result<(), String>`. In the body of the function, rather than calling the `assert_eq!` macro, we return `Ok(())` when the test passes and an `Err` with a `String` inside when the test fails.
16. Writing tests so they return a `Result<T, E>` enables you to use **the question mark operator in the body of tests**, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.
17. You can’t use the `#[should_panic]` annotation on tests that use `Result<T, E>`. To assert that an operation returns an `Err` variant, don’t use the question mark operator on the `Result<T, E>` value. Instead, use `assert!(value.is_err())`.
18. The default behavior of the binary produced by `cargo test` is to **run all the tests in parallel and capture output generated during test runs**, preventing the output from being displayed and making it easier to read the output related to the test results.
19. When you run multiple tests, **by default they run in parallel using threads**. This means the tests will finish running faster so you can get feedback quicker on whether or not your code is working. **Because the tests are running at the same time, make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables**.
20. If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the `--test-threads` flag and the number of threads you want to use to the test binary.
```bash
cargo test -- --test-threads=1
```
21. By default, the output of failed tests don't appear on the screen. If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests at the end with `--show-output`.
```bash
cargo test -- --show-output
```
22. We can pass the name of any test function to cargo test to run only that test:
```bash
cargo test name_of_the_test
```
23. We can’t specify the names of multiple tests in this way; only the first value given to cargo test will be used. But there is a way to run multiple tests. We can specify part of a test name, and any test whose name matches that value will be run.
```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // the following test would be ingored
    fn tests_add() {
        assert_eq!(4, add_two(2));
    }
}
```
24. Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of `cargo test`. Rather than listing as arguments all tests you do want to run, you can instead annotate the time-consuming tests using the `ignore` attribute to exclude them. Otherwise, if we want to run only the ignored tests, we can use `cargo test -- --ignored`.
25. As mentioned at the start of the chapter, testing is a complex discipline, and different people use different terminology and organization. The _Rust_ community thinks about tests in terms of two main categories: **unit tests** and **integration tests**. **Unit tests** are _small and more focused, testing one module in isolation at a time, and can test private interfaces_. **Integration tests** are _entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test_.
26. The purpose of **unit tests** is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected. You’ll put **unit tests** in the `src` directory in each file with the code that they’re testing. **The convention is to create a module named tests in each file to contain the test functions and to annotate the module with `cfg(test)`**.
27. The `#[cfg(test)]` annotation on the tests module tells _Rust_ to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.
28. There’s debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or impossible to test private functions. Regardless of which testing ideology you adhere to, _Rust_’s privacy rules do allow you to test private functions.
```rust
// src/lib.rs

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_internal() {
        let result = internal_adder(2, 4);
        assert_eq!(6, result);
    }
}
```
29. Note that the `internal_adder` function is not marked as `pub`. Tests are just _Rust_ code, and the tests module is just another module. As we discussed in the _“Paths for Referring to an Item in the Module Tree”_ section, **items in child modules can use the items in their ancestor modules**. In this test, we bring all of the test module’s parent’s items into scope with `use super::*`, and then the test can call `internal_adder`. _If you don’t think private functions should be tested, there’s nothing in Rust that will compel you to do so_.
30. In _Rust_, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API. Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. **To create integration tests, you first need a `tests` directory.
31. We create a `tests` directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory. We can then make as many test files as we want to in this directory, and Cargo will compile each of the files as an individual crate.
```rust
// tests/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    let result = adder::adds_two(1);
    assert_eq!(3, result);
}
```
32. Treating each integration test file as its own crate is useful to create separate scopes that are more like the way end users will be using your crate. However, this means files in the tests directory don’t share the same behavior as files in src do, as you learned in Chapter 7 regarding how to separate code into modules and files.
33. To avoid having `common` appear in the test output, instead of creating `tests/common.rs`, we’ll create `tests/common/mod.rs`. This is an alternate naming convention that _Rust_ also understands. Naming the file this way tells _Rust_ not to treat the common module as an integration test file. When we move the `setup` function code into `tests/common/mod.rs` and delete the `tests/common.rs` file, the section in the test output will no longer appear. Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.
34. After we’ve created `tests/common/mod.rs`, we can use it from any of the integration test files as a module. Here’s an example of calling the setup function from the `it_adds_two` test in `tests/integration_test.rs`:
```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```
35. If our project is a _binary crate_ that only contains a `src/main.rs` file and doesn’t have a `src/lib.rs` file, we can’t create integration tests in the tests directory and bring functions defined in the `src/main.rs` file into scope with a use statement. **Only library crates expose functions that other crates can use**; _binary crates_ are meant to be run on their own.
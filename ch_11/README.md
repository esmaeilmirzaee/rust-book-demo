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
```
15. So far, we’ve written tests that panic when they fail. We can also write tests that use `Result<T, E>`! 
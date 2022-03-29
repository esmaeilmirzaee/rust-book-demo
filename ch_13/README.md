1. Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth.
2. _Rust_’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re defined.
3. We want to refer to `simulated_expensive_calculation` only once in `generate_workout`, but **defer the expensive calculation to only where we actually need the result**. _This is a use case for closures!_

```rust
let expensive_closure = |num| {
    println!("Running slowly...");
    thread::sleep(Duration::from_secs(4));
    num
};
```

4. To define a _closure_, we start with a pair of vertical pipes (`|`), inside which we specify the parameters to the closure; this syntax was chosen because of its similarity to closure definitions in Smalltalk and Ruby. This closure has one parameter named `num`: if we had more than one parameter, we would separate them with _commas_, like `|param1, param2|`.
5. Note that this `let` statement means `expensive_closure` contains the definition of an **anonymous function**, not the resulting value of calling the **anonymous function**. Recall that we’re using a closure because we want to define the code to call at one point, store that code, and call it at a later point; the code we want to call is now stored in `expensive_closure`.
6. _Closures_ don’t require you to annotate the types of the parameters or the return value like `fn` functions do. Type annotations are required on functions because they’re part of an explicit interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns. But _closures_ aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library. _Closures_ are usually short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler is reliably able to infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables.

```rust
let example_closure = |x| x;

let name = example_closure(String::from("Hello"));
let num = example_closure(5); // this is the source of error
```

7. The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String. Those types are then locked into the closure in example_closure, and we get a type error if we try to use a different type with the same closure.
8. Fortunately, another solution is available to us. We can create a `struct` that will hold the _closure_ and the resulting value of calling the _closure_. The `struct` will execute the _closure_ only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. You may know this pattern as `memoization` or `lazy` evaluation.
9. To make a `struct` that holds a _closure_, we need to specify the type of the _closure_, because a `struct` definition needs to know the types of each of its fields. Each _closure_ instance has its own unique _anonymous type_: that is, even if two _closures_ have the same signature, their types are still considered different. To define `struct`s, `enums`, or function parameters that use _closures_, we use _generics_ and _trait_ bounds.
10. The `Fn` _traits_ are provided by the standard library. **All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`**.

```rust
struct Cacher<T> wherer T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

11. The `Cacher struct` has a calculation field of the generic type `T`. The trait bounds on `T` specify that it’s a closure by using the `Fn` trait. Any closure we want to store in the calculation field must have one `u32` parameter (specified within the parentheses after `Fn`) and must return a `u32` (specified after the `->`).
12. Functions can implement all three of the `Fn` traits too. If what we want to do doesn’t require capturing a value from the environment, we can use a function rather than a closure where we need something that implements an `Fn` trait.
13. In the workout generator example, we only used closures as inline anonymous functions. However, closures have an additional capability that functions don’t have: they can capture their environment and access variables from the scope in which they’re defined.
14. When a closure captures a value from its environment, it uses memory to store the values for use in the closure body. This use of memory is overhead that we don’t want to pay in more common cases where we want to execute code that doesn’t capture its environment. Because functions are never allowed to capture their environment, defining and using functions will never incur this overhead.
15. Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably. These are encoded in the three `Fn` traits as follows:
    15.1. `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
    15.2. `FnMut` can change the environment because it mutably borrows values.
    15.3. `Fn` borrows values from the environment immutably.
16. Most of the time when specifying one of the `Fn` trait bounds, you can start with `Fn` and the compiler will tell you if you need `FnMut` or `FnOnce` based on what happens in the closure body.
17. `move` closures may still implement `Fn` or `FnMut`, even though they capture variables by `move`. This is because the traits implemented by a closure type are determined by what the closure does with captured values, not how it captures them. The `move` keyword only specifies the latter.

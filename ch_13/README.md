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

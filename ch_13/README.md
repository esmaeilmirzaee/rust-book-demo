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
18. The **iterator pattern** allows you to perform some task on a sequence of items in turn. An `iterator` is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself. In _Rust_, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.
19. Iterators handle all that logic for you, cutting down on repetitive code you could potentially mess up. Iterators give you more flexibility to use the same logic with many different kinds of sequences, not just data structures you can index into, like vectors.
20. Notice this definition uses some new syntax: type `Item` and `Self::Item`, which are defining an associated type with this trait.
21. The `Iterator` trait only requires implementors to define one method: the `next` method, which returns one item of the iterator at a time wrapped in `Some` and, when iteration is over, returns `None`.

```rust
#[test]
fn iterator_demonstratoration() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

22. Note that we needed to make `v1_iter` mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. In other words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the iterator. We didn’t need to make `v1_iter` mutable when we used a for loop because the loop took ownership of `v1_iter` and made it mutable behind the scenes.
23. Also note that the values we get from the calls to next are immutable references to the values in the vector. The `iter` method produces an iterator over immutable references. If we want to create an iterator that takes ownership of `v1` and returns owned values, we can call `into_iter` instead of `iter`. Similarly, if we want to iterate over mutable references, we can call `iter_mut` instead of `iter`.
24. The `Iterator` trait has a number of different methods with default implementations provided by the standard library; you can find out about these methods by looking in the standard library API documentation for the `Iterator` trait. Some of these methods call the `next` method in their definition, which is why you’re required to implement the next method when implementing the `Iterator` trait.

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total = v1_iter.sum();
    assert_eq!(6, total);
}
```

25. Methods that call next are called consuming adaptors, because calling them uses up the iterator. One example is the `sum` method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete. We aren’t allowed to use `v1_iter` after the call to `sum` because `sum` takes ownership of the iterator we call it on.

```rust
#[test]
fn iterator_chaining() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter().map(|x| x+1);
    let mut i = 0;
    for item in v1_iter {
        assert_eq!(v1[i], item);
        i += 1;
    }
}
```

26. Other methods defined on the `Iterator` trait, known as `iterator adaptors`, allow you to change iterators into different kinds of iterators. You can chain multiple calls to `iterator adaptors` to perform complex actions in a readable way. But because **all iterators are lazy**, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.
27. Now that we’ve introduced iterators, we can demonstrate a common use of closures that capture their environment by using the `filter` iterator adaptor. The `filter` method on an iterator takes a closure that takes each item from the iterator and returns a `Boolean`. If the closure returns true, the value will be included in the iterator produced by `filter`. If the closure returns false, the value won’t be included in the resulting iterator.
28. We’ve shown that you can create an iterator by calling `iter`, `into_iter`, or `iter_mut` on a `vector`. You can create iterators from the other collection types in the standard library, such as **hash map**. You can also create iterators that do anything you want by implementing the `Iterator trait` on your own types. As previously mentioned, the only method you’re required to provide a definition for is the `next` method. Once you’ve done that, you can use all other methods that have default implementations provided by the `Iterator trait`!

```rust
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item: u32;

    fn next(&mut self) -> Option<Some::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

29. All of these method calls are possible because we specified how the next method works, and the standard library provides default implementations for other methods that call next.
30. Most Rust programmers prefer to use the iterator style. It’s a bit tougher to get the hang of at first, but once you get a feel for the various iterator adaptors and what they do, iterators can be easier to understand. Instead of fiddling with the various bits of looping and building new vectors, the code focuses on the high-level objective of the loop. This abstracts away some of the commonplace code so it’s easier to see the concepts that are unique to this code, such as the filtering condition each element in the iterator must pass.
31. For a more comprehensive benchmark, you should check using various texts of various sizes as the contents, different words and words of different lengths as the query, and all kinds of other variations. The point is this: iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself. Iterators are one of Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead. This is analogous to how Bjarne Stroustrup, the original designer and implementor of C++, defines zero-overhead in “Foundations of C++” (2012):
32. **`Unrolling` is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop**.
33. **Closures** and **iterators** are _Rust_ features inspired by functional programming language ideas. They contribute to _Rust_’s capability to clearly express high-level ideas at low-level performance. The implementations of **closures** and **iterators** are such that runtime performance is not affected. This is part of _Rust_’s goal to strive to provide zero-cost abstractions.

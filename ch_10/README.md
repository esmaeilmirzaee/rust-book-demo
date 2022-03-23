1. Every programming language has tools for effectively handling the duplication of concepts. In _Rust_, one such tool is **generics**. **Generics** are abstract stand-ins for concrete types or other properties. When we’re writing code, we can express the behavior of **generics** or how they relate to other **generics** without knowing what will be in their place when compiling and running the code.
2. Finally, we’ll discuss **lifetimes**, a variety of generics that give the compiler information about how references relate to each other. **Lifetimes** allow us to borrow values in many situations while still enabling the compiler to check that the references are valid.
3. In sum, steps to remove duplicate code
   3.1 Identify duplicate code.
   3.2 Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
   3.3 Update the two instances of duplicated code to call the function instead.
4. When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.
5. To parameterize the types in the new function we’ll define, we need to name the type parameter, just as we do for the value parameters to a function. You can use any identifier as a type parameter name. But we’ll use `T` because, by convention, parameter names in _Rust_ are short, often just a letter, and _Rust_’s type-naming convention is CamelCase. Short for “type,” `T` is the default choice of most _Rust_ programmers.
6. We read this definition as: the function largest is generic over some type `T`. This function has one parameter named list, which is a slice of values of type `T`. The largest function will return a value of the same type `T`.

```rust
fn largest<T>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max { // consider restricting type parameter `T`
            max = item;
        }
    }

    max
}
```

7. We can also define _structs_ to use a _generic_ type parameter in one or more fields using the `<>` syntax.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point{x: 10, y: 20};
    let float = Point{x: 3.14, y: 6.288};
    let wont_work = Point{x: 1, y: 1.0}; // expected integer, found floating-point number
}
```

8. Note that because we’ve used only one generic type to define `Point<T>`, this definition says that the `Point<T>` struct is generic over some type `T`, and the fields `x` and `y` are both that same type, whatever that type may be. If we create an instance of a `Point<T>` that has values of different types our code won’t compile.

```rust
struct Point<T, U>{
    x: T,
    y: U,
}

fn main() {
    let integer = Point{x: 1, y: 1};
    let float = Point{x: 1.1, y: 1.2};
    let integer_and_float = Point{x: 1, y: 1.1};
}
```

9. To define a `Point` struct where `x` and `y` are both generics but could have different types, we can use multiple generic type parameters. For example we can change the definition of Point to be generic over types `T` and `U` where `x` is of type `T` and `y` is of type `U`.

10. Now all the instances of Point shown are allowed! You can use as many generic type parameters in a definition as you want, but using more than a few makes your code hard to read. When you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // Using different generics for the same struct
    // Be cautious about specifying generic type after function definition
    fn mixup<U>(self, other: Point<U, U>) -> Point<T, U> {
        x: self.x,
        y: other.y,
    }
}

// just available for f32 types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (&self.x.powi(2) + &self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point {x: 1, y: 1};
    println!("p.x = {}", p.x());

    let p_float = Point{x: 1.1, y: 1.2};
    println!("p_float distance from (0.0, 0.0) is {}", p_float.distance_from_origin());
}
```

11. We can implement methods on `structs` and `enums` and use _generic_ types in their definitions, too. Note that we have to declare `T` just after `impl` so we can use it to specify that we’re implementing methods on the type `Point<T>`. By declaring `T` as a generic type after `impl`, _Rust_ can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type. Because this is declaring the generic again, we could have chosen a different name for the generic parameter than the generic parameter declared in the `struct` definition, but using the same name is conventional. Methods written within an `impl` that declares the generic type will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type.
12. You might be wondering whether there is a runtime cost when you’re using generic type parameters. The good news is that _Rust_ implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.
13. _Rust_ accomplishes this by performing **monomorphization** of the code that is using generics at compile time. **Monomorphization** is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
14. A trait tells the _Rust_ compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.
15. A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. **Trait** definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

16. Here, we declare a `trait` using the `trait` keyword and then the `trait`’s name, which is _Summary_ in this case. We’ve also declared the `trait` as `pub` so that crates depending on this crate can make use of this `trait` too, as we’ll see in a few examples. Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this `trait`, which in this case is `fn summarize(&self) -> String`.
17. Implementing a `trait` on a type is similar to implementing regular methods. The difference is that after `impl`, we put the `trait` name that we want to implement, then use the `for` keyword, and then specify the name of the type we want to implement the `trait` for. Within the `impl` block, we put the method signatures that the `trait` definition has defined. Instead of adding a semicolon after each signature, we use curly brackets and fill in the method body with the specific behavior that we want the methods of the `trait` to have for the particular type.
18. We can’t implement **external** `trait`s on **external** types. For example, we can’t implement the `Display` trait on `Vec<T>` within our aggregator crate, because `Display` and `Vec<T>` are defined in the standard library and aren’t local to our aggregator crate. This restriction is part of a property of programs called **coherence**, and more specifically the **orphan rule**, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same **trait** for the same type, and _Rust_ wouldn’t know which implementation to use.

```rust
// src/lib.rs
pub trait Summary {
    fn summarize(&self) -> String {
        // default implementation
        String::from("(Read more...)");
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format("1 new tweet: {} {}.", self.author, self.content)
    }
}


// src/main.rs
use aggregator::{Summary, Tweet, NewArticle};

fn main() {
    let article = NewArticle {
        headline: String::from("lorem ipsum"),
        content: String::from("lorem ipsum is trivial!"),
        location: String::from("California, USA"),
        author: String::from("A rich developer"),
    };
    // Using default trait implementation: (Read more...)
    println!("Using default trait implementation: {}", article.summary());

    let tweet = Tweet {
        username: String::from("john.doe"),
        content: String::fromt("I enjoy learning RUST!"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
}
```

19. Now that you know how to define and implement `trait`s, we can explore how to use `trait`s to define functions that accept many different types.

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking NEWS: {}", item.summarize());
}

// trait bound
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking NEWS: {}", item.summarize());
}
```

20. The `impl Trait` syntax works for straightforward cases but is actually **syntax sugar** for a longer form, which is called a **trait bound**.
21. The `impl Trait` syntax is convenient and makes for more concise code in simple cases. The trait bound syntax can express more complexity in other cases. For example, we can have two parameters that implement `Summary`. Using the `impl Trait` syntax looks like this: `pub fn notify(item1: &impl Summay, item2: &impl Summary) {}`.
22. We can also specify more than one **trait bound**. Say we wanted `notify` to use `display` formatting on item as well as the `summarize` method: we specify in the `notify` definition that item must implement both `Display` and `Summary`. We can do so using the + syntax: `pub fn notify(item: &(impl Summary + Display)) {}`. Or, trait bounds, `pub fn notify<T: Summary + Display>(item: &T) {}`.
23. Using too many **trait bounds** has its downsides. Each generic has its own **trait bounds**, so functions with multiple generic type parameters can contain lots of **trait bound** information between the function’s name and its parameter list, making the function signature hard to read. For this reason, _Rust_ has alternate syntax for specifying **trait bounds** inside a where clause after the function signature. So instead of writing this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```

we can use `where` clause, like this:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Debug {}
```

24. We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a `trait`, as shown here:

```rust
fn return_summarizable() -> impl Summary {
    Tweet {
        user_name: String::from("john.doe"),
        content: String::from("lorem ipsum quo du iso"),
        reply: false,
        retweet: false,
    }
}
```

25. We can also conditionally implement a `trait` for any type that implements another `trait`. Implementations of a `trait` on any type that satisfies the `trait bounds` are called **blanket implementations** and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait. `impl<T: Display> ToString for T {}`
26. Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior. In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method. But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics.
27. Another kind of generic that we’ve already been using is called **lifetimes**. Rather than ensuring that a type has the behavior we want, **lifetimes** ensure that references are valid as long as we need them to be.
28. One detail we didn’t discuss in the _“References and Borrowing”_ section in _Chapter 4_ is that every reference in _Rust_ has a **lifetime**, which is the scope for which that reference is valid. Most of the time, **lifetimes** are implicit and inferred, just like most of the time, types are inferred.
29. The main aim of **lifetimes** is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

```rust
{
    let r; // this is not null value, compiler won't allow using it before it is initialized
    {
        let x = 5;
        r = &x;
    } // x dropped while still borrowed, and r is referring to has gone out of scope.
    println!("{}", r);
}
```

30. The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

31. When we’re defining this function, we don’t know the _concrete values_ that will be passed into this function, so we don’t know whether the `if` case or the `else` case will execute. We also _don’t know the concrete lifetimes of the references_ that will be passed in, so we can’t look at the scopes as we did in Listings 10-18 and 10-19 _to determine whether the reference we return will always be valid_. The borrow checker can’t determine this either, because **it doesn’t know how the lifetimes of `x` and `y` relate to the lifetime of the return value**. To fix this error, **we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis**.
32. **Lifetime annotations don’t change how long any of the references live. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter. Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.**
33. **Lifetime annotations** have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe `(')` and are usually all lowercase and very short, like generic types. Most people use the name `'a`. We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference’s type.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

34. The constraint we want to express in this signature is that the lifetimes of both of the parameters and the lifetime of the returned reference are related such that the returned reference will be valid as long as both the parameters are. We’ll name the lifetime 'a and then add it to each reference
35. Try designing more experiments that vary the values and lifetimes of the references passed in to the `longest` function and how the returned reference is used. Make hypotheses about whether or not your experiments will pass the borrow checker before you compile; then check to see if you’re right!

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

36. In this example, we’ve specified a lifetime parameter `'a` for the parameter `x` and the return type, but not for the parameter `y`, because the lifetime of `y` does not have any relationship with the lifetime of `x` or the return value.
37. Ultimately, **lifetime** syntax is about connecting the **lifetime**s of various parameters and return values of functions. Once they’re connected, _Rust_ has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.
38. The patterns programmed into _Rust_’s analysis of references are called the **lifetime elision rules**. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.
39. Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
40. The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.
41. The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.
42. The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
43. The third rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

```rust
// 1. The signature starts without any lifetimes associated with the references.
fn first_word(s: &str) -> &str {
// 2. Then the compiler applies the first rule, which specifies that each parameter gets its own lifetime.
fn first_word<'a>(s: &'a str) -> &str {
// 3. The second rule applies because there is exactly one input lifetime.
fn first_word<'a>(s: &'a str) -> &'a str {
```

44. When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters shown in Listing 10-11. Where we declare and use the lifetime parameters depends on whether they’re related to the struct fields or the method parameters and return values.
45. Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.
46. In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent. In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures.

```rust
struct ImportantExcerpt {
    part: String,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

```

47. One special **lifetime** we need to discuss is `'static`, which means that this reference can live for the entire duration of the program. All string literals have the `'static` **lifetime**, which we can annotate as follows. The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is `'static`.

```rust
#![allow(unused)]
fn main() {
    let s: &'static str = "I have a static lifetime.";
}
```

48. **You might see suggestions to use the 'static lifetime in error messages. But before specifying 'static as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not. You might consider whether you want it to live that long, even if it could. Most of the time, the problem results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the 'static lifetime.**

```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement!{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

49. This is the `longest` function, but now it has an extra parameter named `ann` of the generic type `T`, which can be filled in by any type that implements the `Display` _trait_ as specified by the `where` clause. This extra parameter will be printed using `{}`, which is why the `Display` _trait bound_ is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list inside the angle brackets after the function name.

    > We covered a lot in this chapter! Now that you know about generic type parameters, traits and trait bounds, and generic lifetime parameters, you’re ready to write code without repetition that works in many different situations. Generic type parameters let you apply the code to different types. Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references. And all of this analysis happens at compile time, which doesn’t affect runtime performance!

1. Rust’s **enums** are most similar to **algebraic data types in functional languages**, such as _F#_, _OCaml_, and _Haskell_.
2. **Enums** are a way of defining custom data types in a different way than you do with **structs**.
3. Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate, because an enum value can only be one of its variants. Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.
4. We can express this concept in code by defining an `IpAddrKind` **enumeration** and listing the possible kinds an `IP` address can be, `V4` and `V6`. These are the variants of the enum:

```rust
// `IpAddrKind` is now a custom data type that we can use elsewhere in our code.
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.1.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

5. Here, we’ve defined a struct `IpAddr` that has two fields: a `kind` field that is of type `IpAddrKind` (the **enum** we defined previously) and an address field of type **String**.
6. However, representing the same concept using just an **enum** is more concise: rather than an enum inside a **struct**, we can put data directly into each **enum** variant. This new definition of the `IpAddr` **enum** says that both `V4` and `V6` variants will have associated String values:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("192.168.1.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
```

7. We attach data to each variant of the enum directly, so there is no need for an extra struct. Here it’s also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an instance of the enum. That is, `IpAddr::V4()` is a function call that takes a `String` argument and returns an instance of the `IpAddr` type. We automatically get this constructor function defined as a result of defining the enum.
8. The `Option`, which is another enum defined by the standard library, type encodes the very common scenario in which a value could be something or it could be nothing.
9. The `Option`, which is another enum defined by the standard library, type encodes the very common scenario in which a value could be something or it could be nothing.
10. `Rust` doesn’t have the `null` feature that many other languages have. `Null` is a value that means there is no value there. In languages with `null`, variables can always be in one of two states: `null` or `not-null`.
11. Rust does not have **nulls**, but it does have an **enum** that can encode the concept of a value being present or absent. This enum is `Option<T>`, and it is defined by _the standard library_ as follows:

```rust
enum Option {
    None,
    Some(T),
}
```

12. The `Option<T>` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. Its variants are also included in the prelude: you can use Some and None directly without the `Option::` prefix. The `Option<T>` **enum** is still just a regular **enum**, and `Some(T)` and `None` are still variants of type `Option<T>`.
13. The <T> syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter, and we’ll cover generics in more detail in Chapter 10. For now, all you need to know is that <T> means the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T makes the overall Option<T> type a different type. Here are some examples of using Option values to hold number types and string types:

```rust
let some_number = Some(5); // T is i32
let some_string = Some("a string"); // T is &str
let absent_number: Option<i32> = None; // there is no value so the type should be specified
```

14. So why is having Option<T> any better than having null? In short, because `Option<T>` and `T` (where `T` can be any type) are different types, the compiler won’t let us use an `Option<T>` value as if it were definitely a valid value. For example, this code won’t compile because it’s trying to add an `i8` to an `Option<i8>`:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; // ^ no implementation for `i8 + Option<i8>`
```

14. Intense! In effect, this error message means that **Rust** doesn’t understand how to add an `i8` and an `Option<i8>`, because they’re different types. When we have a value of a type like `i8` in **Rust**, the compiler will ensure that we always have a valid value. We can proceed confidently without having to check for null before using that value. Only when we have an `Option<i8>` (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value. In other words, you have to convert an Option<T> to a T before you can perform T operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
15. Rust has an extremely powerful control flow construct called `match` that allows you to compare _a value_ against _a series of patterns_ and then execute code based on which pattern matches. Patterns can be made up of _literal values_, _variable names_, _wildcards_, and **many other things**.
16. First, we list the `match` keyword followed by an expression, which in this case is the value _coin_. This seems very similar to an expression used with `if`, but there’s a big difference: with `if`, the expression needs to return a `Boolean` value, but here, **it can return any type**. Next are the match `arms`. An `arm` has two parts: _a pattern_ and _some code_. When the `match` expression executes, it compares the resulting value against _the pattern of each_ `arm`, in order. If a pattern matches the value, the code associated with that pattern is executed. If that pattern doesn’t match the value, execution continues to the next `arm`.
17. **The code associated with each `arm` is an expression, and the resulting value of the expression in the matching `arm` is the value that gets returned for the entire match expression.**
18. Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.
19. Combining `match` and enums is useful in many situations. You’ll see this pattern a lot in Rust code: `match` against an `enum`, bind a variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.
20. `Matches` in `Rust` **are exhaustive**: _we must exhaust every last possibility in order for the code to be valid_. Especially in the case of `Option<T>`, when Rust prevents us from forgetting to explicitly handle the `None` case, it protects us from assuming that we have a value when we might have null, thus making the billion-dollar mistake discussed earlier impossible.

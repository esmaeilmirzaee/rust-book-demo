1. As you write large programs, organizing your code will be important because keeping track of your entire program in your head will become impossible. By grouping related functionality and separating code with distinct features, you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.
2. Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:
    - **Packages**: A _Cargo_ feature that lets you _build_, _test_, and _share crates_
    - **Crates**: A tree of modules that produces a _library_ or _executable_
    - **Modules** and **use**: Let you control the _organization_, _scope_, and _privacy of paths_
    - **Paths**: A way of naming an item, such as a _struct_, _function_, or _module_
3. A _crate_ is a _binary_ or _library_. The _crate_ root is a source file that the Rust compiler starts from and makes up the root module of your crate. A _package_ is one or more _crates_ that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.
4. Several rules determine what a package can contain. A package can contain at most one library crate. It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).
5. When we entered the command, _Cargo_ created a `Cargo.toml` file, giving us a package. Looking at the contents of `Cargo.toml`, there’s no mention of `src/main.rs` because _Cargo_ follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package. Likewise, _Cargo_ knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root. Cargo passes the crate root files to `rustc` to build the library or binary.
6. A package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate binary crate.
7. Keeping a crate’s functionality in its own scope clarifies whether particular functionality is defined in our crate or the `rand` crate and prevents potential conflicts. For example, the `rand` crate provides a trait named `Rng`. We can also define a `struct` named `Rng` in our own crate. Because a crate’s functionality is **namespaced** in its own scope, when we add `rand` as a dependency, the compiler isn’t confused about what the name `Rng` refers to. In our crate, it refers to the `struct Rng` that we defined. We would access the `Rng` trait from the `rand` crate as `rand::Rng`.
8. **Modules** let us organize code within a _crate_ into groups for readability and easy reuse. **Modules** also control the privacy of items, which is whether an item can be used by outside code (_public_) or is an internal implementation detail and not available for outside use (_private_).
9. Create a new _library_ named _restaurant_ by running `cargo new --lib restaurant`.
10. We define a module by starting with the `mod` keyword and then specify the name of the module (in this case, `front_of_house`) and place curly brackets around the body of the _module_. **Inside modules, we can have other modules**, as in this case with the modules hosting and serving. Modules can also hold definitions for other items, such as `structs`, `enums`, `constants`, `traits`, or `functions`.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

11. We mentioned that `src/main.rs` and `src/lib.rs` are called crate roots. The reason for their name is that the contents of either of these two files form a _module_ named crate at the root of the crate’s module structure, known as the _module tree_.
12. To show `Rust` where to find an item in a _module tree_, we use a path in the same way we use a path when navigating a filesystem. If we want to call a function, we need to know its path.
    A path can take two forms:

        - An `absolute path` starts from a crate root by using a _crate name_ or a _literal crate_.
        - A `relative path` starts from the current _module_ and uses `self`, `super`, or an identifier in the current module.

Both `absolute` and `relative paths` are followed by one or more identifiers separated by double colons (`::`).

...

13. Choosing whether to use a relative or absolute path is a decision you’ll make based on your project. The decision should depend on whether you’re more likely to move item definition code separately from or together with the code that uses the item. **Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other.**
14. _Modules_ aren’t useful only for organizing your code. They also define `Rust`’s **privacy boundary**: the line that _encapsulates the implementation details external code isn’t allowed to know about_, _call_, or rely on. So, _if you want to make an item like a function or struct private, you put it in a module_.
15. The way _privacy_ works in `Rust` is that all items (`functions`, `methods`, `structs`, `enums`, `modules`, and `constants`) are _private by default_. **Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. The reason is that child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined**.
16. Rust chose to have the module system function this way so that hiding inner implementation details is the default. That way, you know which parts of the inner code you can change without breaking outer code. But you can expose inner parts of child modules’ code to outer ancestor modules by using the `pub` keyword to make an item public.
17. **The privacy rules** apply to `structs`, `enums`, `functions`, and `methods` as well as `modules`.
18. We can also construct _relative paths_ that begin in the _parent module_ by using `super` at the start of the path. This is like starting a filesystem path with the `..` syntax.
19. We can also use `pub` to designate `struct`s and `enum`s as _public_, but **there are a few extra details**. If we use `pub` before a `struct` definition, we make the `struct` public, but the `struct`’s **fields** will still be **private**. We can make _each field public or not on a case-by-case basis_.
20. `Enum`s aren’t very useful unless their variants are public; it would be annoying to have to annotate all `enum` variants with `pub` in every case, so the default for `enum` variants is to be public. `Struct`s are often useful without their fields being `public`, so `struct` fields follow the general rule of everything being `private` by default unless annotated with `pub`.
21. We can bring a path into a scope once and then call the items in that path as if they’re local items with the `use` keyword. Adding `use` and a path in a scope is similar to creating a _symbolic link_ in the _filesystem_. By adding `use crate::front_of_house::hosting` in the crate root, _hosting_ is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with `use` also **check privacy**, like any other paths.
22. Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is the idiomatic way to bring a function into scope with `use`. Bringing the function’s parent module into scope with `use` means we have to specify the parent module when calling the function. **Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path**.

```rust
use crate::front_of_house::hosting;
// use crate::front_of_house::hosting::add_to_waitlist; // using this way makes a function like a local

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // add_to_waitlist(); // it looks it's a local function
}
```

23. On the other hand, when bringing in `struct`s, `enum`s, and other items with `use`, it’s idiomatic to specify the full path. _There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way_. **The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that**. As you will see, using the parent modules distinguishes the two `Result` types. If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d have two `Result` types in the same scope and _Rust_ wouldn’t know which one we meant when we used `Result`.

```rust
use std::collections::HashMap;
use std::fmt;
use std::io;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn function_1() -> fmt::Result {}
fn function_2() -> io::Result<()> {}
```

24. There’s another solution to the problem of bringing two types of the same name into the same scope with `use`: after the path, we can specify `as` and a new local name, or alias, for the type.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function_1() -> Result {}
fn function_2() -> IoResult<()> {}
```

25. When we bring a name into scope with the `use` keyword, the name available in the new scope is _private_. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine `pub` and `use`. This technique is called **re-exporting** because we’re bringing an item into scope but also making that item available for others to bring into their scope.

```rust
mod front_of_house {
    pub mod hosting() {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

26. By using `pub use`, external code can now call the `add_to_waitlist` function using `hosting::add_to_waitlist`. If we hadn’t specified `pub use`, the `eat_at_restaurant` function could call `hosting::add_to_waitlist` in its scope, but external code couldn’t take advantage of this new path.
27. Members of the Rust community have made many packages available at **crates.io**, and pulling any of them into your package involves these same steps: _listing them in your package’s_ `Cargo.toml` file and using `use` to bring items from their crates into scope.
28. If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files. Instead, we can use nested paths to bring the same items into scope in one line. We do this by specifying the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that differ.

```rust
use std::cmp::Ordering;
use std::io;

// -- snip
use std::{cmp::Ordering, io};
```

29. We can use a nested path at any level in a path, which is useful when combining two use statements that share a subpath. The common part of these two paths is `std::io`, and that’s the complete first path. To merge these two paths into one use statement, we can use `self` in the nested path.

```rust
use std::io;
use std::io::Write;

// --snip--
use std::io::{slef, Write};
```

30. If we want to bring all public items defined in a path into scope, we can specify that path followed by `*`, the glob operator, for example `use std::collections::*;`
31. Note that the `pub use crate::front_of_house::hosting` statement in `src/lib.rs` also hasn’t changed, nor does `use` have any impact on what files are compiled as part of the crate. The `mod` keyword declares modules, and _Rust_ looks in a file with the same name as the module for the code that goes into that module.

> _Rust_ lets you split a package into multiple _crates_ and _a crate_ into _modules_ so you can refer to items defined in one module from another module. You can do this by specifying **absolute** or **relative paths**. These paths can be brought into scope with a `use` statement so you can use a shorter path for multiple uses of the item in that scope. `Module` code is `private` **by default**, but you can make definitions public by adding the `pub` keyword.

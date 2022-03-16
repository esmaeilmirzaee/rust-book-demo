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

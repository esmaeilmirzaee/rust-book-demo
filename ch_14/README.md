1. In _Rust_, `release` profiles are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code. Each profile is configured independently of the others.
2. `Cargo` has two main profiles: the `dev` profile `Cargo` uses when you run `cargo build` and the `release` profile `Cargo` uses when you run `cargo build --release`. The `dev` profile is defined with good defaults for development, and the `release` profile has good defaults for `release` builds.
3. `Cargo` has default settings for each of the profiles that apply when there aren’t any `[profile.*]` sections in the project’s `Cargo`.toml file. By adding `[profile.*]` sections for any profile you want to customize, you can override any subset of the default settings. For example, here are the default values for the `opt-level` setting for the dev and release profiles:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

4. The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3. Applying more optimizations extends compiling time, so if you’re in development and compiling your code often, you’ll want faster compiling even if the resulting code runs slower. That is the reason the default opt-level for dev is 0. When you’re ready to release your code, it’s best to spend more time compiling. You’ll only compile in release mode once, but you’ll run the compiled program many times, so release mode trades longer compile time for code that runs faster. That is why the default opt-level for the release profile is 3.
5. Rust also has a particular kind of comment for documentation, known conveniently as a documentation comment, that will generate HTML documentation. The HTML displays the contents of documentation comments for public API items intended for programmers interested in knowing how to use your crate as opposed to how your crate is implemented.
6. Documentation comments use three slashes, ///, instead of two and support Markdown notation for formatting the text. Place documentation comments just before the item they’re documenting. Then run `cargo doc --open`.

````rust
/// Adds one to the number given.
///
/// # Examples
/// ```rust
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// assert_eq!(6, answer);
/// pub fn add_one(num: i32) -> i32 {
///     num + 1
/// }
/// ```
````

7. Here are some other sections that crate authors commonly use in their documentation:

    7.1. `Panics`: The scenarios in which the function being documented could `panic`. Callers of the function who don’t want their programs to `panic` should make sure they don’t call the function in these situations.
    7.2. `Errors`: If the function returns a `Result`, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways.
    7.3. `Safety`: If the function is unsafe to call, there should be a section explaining why the function is `unsafe` and covering the invariants that the function expects callers to uphold.

8. Another style of doc comment, `//!`, adds documentation to the item that contains the comments rather than adding documentation to the items following the comments. We typically use these doc comments inside the _crate root file_ (`src/lib.rs` by convention) or inside a module to document the crate or the module as a whole. Documentation comments within items are useful for describing crates and modules especially. Use them to explain the overall purpose of the container to help your users understand the crate’s organization.
9. The good news is that if the structure isn’t convenient for others to use from another library, you don’t have to rearrange your internal organization: instead, you can re-export items to make a public structure that’s different from your private structure by using `pub use`. Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.
10. Creating a useful public API structure is more of an art than a science, and you can iterate to find the API that works best for your users. Choosing `pub use` gives you flexibility in how you structure your crate internally and decouples that internal structure from what you present to your users. Look at some of the code of crates you’ve installed to see if their internal structure differs from their public API.
11. Now that you have an account, let’s say you have a crate you want to publish. Before publishing, you’ll need to add some metadata to your crate by adding it to the [package] section of the crate’s Cargo.toml file.
12. Your crate will need a unique name. While you’re working on a crate locally, you can name a crate whatever you’d like. However, crate names on crates.io are allocated on a first-come, first-served basis. Once a crate name is taken, no one else can publish a crate with that name. Before attempting to publish a crate, search for the name you want to use on the site. If the name has been used by another crate, you will need to find another name and edit the name field in the Cargo.toml file under the [package] section to use the new name for publishing. Even if you’ve chosen a unique name, when you run `cargo publish` to publish the crate at this point, you’ll get a warning and then an error. The reason is that you’re missing some crucial information: a description and license are required so people will know what your crate does and under what terms they can use it. To rectify this error, you need to include this information in the Cargo.toml file.
13. If you want to use a license that doesn’t appear in the SPDX, you need to place the text of that `license` in a file, include the file in your project, and then use `license-file` to specify the name of that file instead of using the `license` key.
14. Many people in the _Rust_ community license their projects in the same way as _Rust_ by using a dual license of `MIT OR Apache-2.0`. This practice demonstrates that you can also specify multiple license identifiers separated by `OR` to have multiple licenses for your project.
15. Be careful when publishing a crate because a publish is permanent. The version can never be overwritten, and the code cannot be deleted. One major goal of crates.io is to act as a permanent archive of code so that builds of all projects that depend on crates from crates.io will continue to work. Allowing version deletions would make fulfilling that goal impossible. However, there is no limit to the number of crate versions you can publish.
16. Although you can’t remove previous versions of a crate, you can prevent any future projects from adding them as a new dependency. This is useful when a crate version is broken for one reason or another. In such situations, Cargo supports **yanking** a crate version. Yanking a version prevents new projects from starting to depend on that version while allowing all existing projects that depend on it to continue to download and depend on that version. Essentially, a yank means that all projects with a Cargo.lock will not break, and any future `Cargo.lock` files generated will not use the yanked version.

```bash
cargo yank --vers 1.0.1
cargo yank --vers 1.0.1 --undo
```

17. A yank does not delete any code. For example, the yank feature is not intended for deleting accidentally uploaded secrets. If that happens, you must reset those secrets immediately.
18. As your project develops, you might find that the library crate continues to get bigger and you want to split up your package further into multiple library crates. In this situation, Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem.
19. A workspace is a set of packages that share the same `Cargo.lock` and output directory.
20. Steps to create a **workspace**
21. Cargo structures the target directory in a workspace like this because the crates in a workspace are meant to depend on each other. If each crate had its own target directory, each crate would have to recompile each of the other crates in the workspace to have the artifacts in its own target directory. By sharing one target directory, the crates can avoid unnecessary rebuilding.
22. Notice that the workspace has only one Cargo.lock file at the top level of the workspace rather than having a Cargo.lock in each crate’s directory. This ensures that all crates are using the same version of all dependencies. If we add the rand package to the adder/Cargo.toml and add-one/Cargo.toml files, Cargo will resolve both of those to one version of rand and record that in the one Cargo.lock. Making all crates in the workspace use the same dependencies means the crates in the workspace will always be compatible with each other.
23. Running cargo test in a workspace structured like this one will run the tests for all the crates in the workspace. We can also run tests for one particular crate in a workspace from the top-level directory by using the `-p` flag and specifying the name of the crate we want to test.
24. If you publish the crates in the workspace to crates.io, each crate in the workspace will need to be published separately. The cargo publish command does not have an --all flag or a `-p` flag, so you must change to each crate’s directory and run cargo publish on each crate in the workspace to publish the crates.
25. The cargo install command allows you to install and use binary crates locally. This isn’t intended to replace system packages; it’s meant to be a convenient way for Rust developers to install tools that others have shared on crates.io. Note that you can only install packages that have binary targets. A binary target is the runnable program that is created if the crate has a src/main.rs file or another file specified as a binary, as opposed to a library target that isn’t runnable on its own but is suitable for including within other programs. Usually, crates have information in the README file about whether a crate is a library, has a binary target, or both.
26. Cargo is designed so you can extend it with new subcommands without having to modify Cargo. If a binary in your $PATH is named cargo-something, you can run it as if it was a Cargo subcommand by running cargo something. Custom commands like this are also listed when you run `cargo --list`. Being able to use cargo install to install extensions and then run them just like the built-in Cargo tools is a super convenient benefit of Cargo’s design!

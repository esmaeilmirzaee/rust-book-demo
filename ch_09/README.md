1. Rust groups errors into two major categories: **recoverable** and **unrecoverable** errors. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.
2. Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as _exceptions_. **Rust doesn’t have exceptions**. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.
3. Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these cases, Rust has the `panic!` macro. When the `panic!` macro executes, **your program will print a failure message, unwind and clean up the stack, and then quit**. This most commonly occurs when a bug of some kind has been detected and it’s not clear to the programmer how to handle the error.
4. By default, when a panic occurs, the program starts **unwinding**, which means **Rust walks back up the stack and cleans up the data from each function it encounters**. But this walking back and cleanup is a lot of work. _The alternative is to immediately abort_, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. If in your project you _need to make the resulting binary as small as possible_, you can switch from _unwinding_ to _aborting_ upon a panic by adding `panic = 'abort'` to the appropriate `[profile]` sections in your `Cargo.toml` file. For example, if you want to abort on panic in release mode, add this:

```toml
[profile.release]
panic = 'abort'
```

5. In C, attempting to read beyond the end of a data structure is undefined behavior. You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure. This is called a **buffer overread** and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.
6. A **backtrace** is a list of all the functions that have been called to get to this point. **Backtraces** in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

7. What you need to know right now is that `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant. Because `Result` has these generic type parameters, we can use the `Result` type and the functions that the standard library has defined on it in many different situations where the successful value and error value we want to return may differ.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(eror) => panic!("Problem opening the file {:?}", error),
    };
}
```

8. If we give `f` a type annotation that we know is not the return type of the function and then try to compile the code, the compiler will tell us that the types don’t match. **The error message will then tell us what the type of `f` is**.
9. Note that, like the `Option` `enum`, the `Result` `enum` and its variants have been brought into scope by the prelude, so we don’t need to specify `Result::` before the `Ok` and `Err` variants in the `match` arms.
10. What we want to do instead is take different actions for different failure reasons: if `File::open` failed because the file doesn’t exist, we want to create the file and return the handle to the new file. If `File::open` failed for any other reason—for example, because we didn’t have permission to open the file—we still want the code to `panic!` in the same way as it did.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Couldn't create the file {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        }
    };
}
```

11. The type of the value that `File::open` returns inside the `Err` variant is `io::Error`, which is a struct provided by the standard library. This struct has a method `kind` that we can call to get an `io::ErrorKind` value.
12. The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of errors that might result from an `io` operation. The variant we want to use is `ErrorKind::NotFound`, which indicates the file we’re trying to open doesn’t exist yet. So we match on `f`, but we also have an inner match on `error.kind()`.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Couldn't create the file {:?}", error);
            }) // this expression returns new created file. DO NOT ADD SEMICOLON.
        } else {
            panic!("Couldn't open the file {:?}", error);
        }
    });
}
```

13. Come back to this example after you’ve read Chapter 13, and look up the `unwrap_or_else` method in the standard library documentation. Many more of these methods can clean up huge nested match expressions when you’re dealing with errors.
14. Using `match` works well enough, but it can be a bit verbose and doesn’t always communicate intent well. The `Result<T, E>` type has many helper methods defined on it to do various tasks. One of those methods, called `unwrap`, is a shortcut method that is implemented just like the `match` expression we wrote in Listing 9-4. If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, unwrap will call the `panic!` macro for us.
15. Another method, `expect`, which is similar to `unwrap`, lets us also choose the `panic!` error message. Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

16. We use `expect` in the same way as `unwrap`: to return the file handle or call the `panic!` macro. The error message used by `expect` in its call to `panic!` will be the parameter that we pass to `expect`, rather than the default `panic!` message that `unwrap` uses.
17. When you’re writing a function whose implementation calls something that might fail, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do. This is known as **propagating the error** and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

18. Let’s look at the return type of the function first: `Result<String, io::Error>`. This means the function is returning a value of the type `Result<T, E>` where the generic parameter `T` has been filled in with the concrete type `String` and the generic type `E` has been filled in with the concrete type `io::Error`. If this function succeeds without any problems, the code that calls this function will receive an `Ok` value that holds a `String`—the username that this function read from the file. If this function encounters any problems, the code that calls this function will receive an `Err` value that holds an instance of `io::Error` that contains more information about what the problems were. We chose `io::Error` as the return type of this function because that happens to be the type of the error value returned from both of the operations we’re calling in this function’s body that might fail: the `File::open` function and the `read_to_string` method.
19. This pattern of **propagating errors** is so common in Rust that Rust provides the question mark operator `?` to make this easier.
20. There is a difference between what the match expression from Listing 9-6 does and what the ? operator does: error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as there’s an impl From<OtherError> for ReturnedError to define the conversion in the trait’s from function, the ? operator takes care of calling the from function automatically.
21. The `?` operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the `?`.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s);
}
```

22. Reading a file into a string is a fairly common operation, so _Rust_ provides the convenient `fs::read_to_string` function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it.

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

23. The `?` operator can only be used in functions that have a return type compatible with the value the `?` is used on.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

24. This code opens a file, which might fail. The `?` operator follows the `Result` value returned by `File::open`, but this main function has the return type of `()`, not `Result`. This code will compile to error. This error points out that we’re only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual`. **To fix this error, you have two choices**. One technique is to change the return type of your function to be `Result<T, E>` if you have no restrictions preventing that. The other technique is to use a match or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is appropriate.
25. The error message also mentioned that `?` can be used with `Option<T>` values as well. As with using `?` on `Result`, you can only use `?` on `Option` in a function that returns an `Option`. The behavior of the `?` operator when called on an `Option<T>` is similar to its behavior when called on a `Result<T, E>`: if the value is `None`, the `None` will be returned early from the function at that point. If the value is `Some`, the value inside the `Some` is the resulting value of the expression and the function continues.

```rust
fn last_char_at_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

26. This function returns `Option<char>` because it might find a character at this position, or there might be no character there. This code takes the text string slice argument and calls the lines method on it, which returns an iterator over the lines in the string. Because this function wants to examine the first line, it calls next on the iterator to get the first value from the iterator. If text is the empty string, this call to next will return `None`, and here we can use `?` to stop and return `None` from `last_char_of_first_line` if that is the case. If text is not the empty string, next will return a `Some` value containing a string slice of the first line in text.
27. Note that you can use the `?` operator on a `Result` in a function that returns `Result`, and you can use the `?` operator on an `Option` in a function that returns `Option`, but you can’t mix and match. The `?` operator won’t automatically convert a `Result` to an `Option` or vice versa; in those cases, there are methods like the `ok` method on `Result` or the `ok_or` method on `Option` that will do the conversion explicitly.
28. Executables written in C return integers when they exit, and _Rust_ executables follow this convention as well: programs that exit successfully return the integer `0`, and programs that error return some integer other than `0`.
29. Using `?` on a `Result` value in a main function with this return type is allowed, because now an `Err` value can be returned early. When a main function returns a `Result<(), E>`, the executable will exit with a value of `0` if main returns `Ok(())` and will exit with a nonzero value if main returns an `Err` value.
30. Returning Result is a good default choice when you’re defining a function that might fail.
31. Similarly, the `unwrap` and `expect` methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust.

```rust
use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}
```

32. We’re creating an `IpAddr` instance by parsing a hardcoded string. We can see that `127.0.0.1` is a valid IP address, so it’s acceptable to use `unwrap` here. However, having a hardcoded, valid string doesn’t change the return type of the parse method: we still get a `Result` value, and the compiler will still make us handle the `Result` as if the `Err` variant is a possibility because the compiler isn’t smart enough to see that this string is always a valid IP address. If the IP address string came from a user rather than being hardcoded into the program and therefore did have a possibility of failure, we’d definitely want to handle the `Result` in a more robust way instead.
33. If someone calls your code and passes in values that don’t make sense, the best choice might be to call `panic!` and alert the person using your library to the bug in their code so they can fix it during development. Similarly, `panic!` is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

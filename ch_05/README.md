1. Structs are similar to tuples in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.
2. To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields.
3. To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing `key: value` pairs, where the keys are the names of the fields and the values are the data we want to store in those fields. We don’t have to specify the fields in the same order in which we declared them in the struct. In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("john.doe@example.com"),
        username: String::from("John DOE"),
        active: false,
        sign_in_count: 1,
    }
}
```

4. Because the parameter names and the struct field names are exactly the same in Listing 5-4, we can use the field init shorthand syntax to rewrite build_user so that it behaves exactly the same but doesn’t have the repetition of email and username, as shown in Listing 5-5.

```rust
fn build_a_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: false,
        sign_in_count: 0,
    }
}
```

5. Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7. The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance. The `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in `user1`, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
};

fn main() {
    let user1 = User {
        username: String::from("JohnDOE"),
        email: String::from("JohnDOE@example.com"),
        active: false,
        sign_in_count: 0,
    };
    let user2 = User {
        username: String::from("JaneDOE"),
        email: String::from("JaneDOE@example.com"),
        ..user1,
    };
}
```

5. > In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid. It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like the following; this won’t work:

```rust
struct User {
    active: bool,
    sign_in_count: u32,
    username: &str,
    email: &str,
};

fn main() {
    let user1 = User {
        email: "JohnDOE@example.com",
        username: "John DOE",
        active: true,
        sign_in_count: 0,
    };
}
```

5. We’ll discuss how to fix these errors so you can store references in structs, but for now, we’ll fix errors like these using owned types like `String` instead of references like `&str`.

6. Note that the struct update syntax uses `=` like an assignment; this is because _it moves the data_, just as we saw in the “Ways Variables and Data Interact: Move” section. In this example, we can no longer use `user1` after creating `user2` because the **String** in the `username` field of user1 was moved into `user2`. If we had given `user2` new **String** values for both `email` and `username`, and thus only used the `active` and `sign_in_count` values from `user1`, then `user1` would still be valid after creating `user2`. The types of `active` and `sign_in_count` are types that implement the _Copy_ trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.
7. Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
8. To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple. For example, here we define and use two tuple structs named Color and Point:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(100, 200, 100);
}
```

9. Note that the `black` and `origin` values are different types, because they’re instances of different _tuple_ structs. **Each struct you define is its own type**, even though the fields within the struct have the same types. For example, a function that takes a parameter of type _Color_ cannot take a _Point_ as an argument, _even though both types are made up of three i32 values_. Otherwise, _tuple_ struct instances behave like tuples: you can destructure them into their individual pieces, you can use a `.` followed by the index to access an individual value, and so on.
10. You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section. Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. To define AlwaysEqual, we use the struct keyword, the name we want, then a semicolon. No need for curly brackets or parentheses! Then we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses. Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior! You’ll see in Chapter 10 how to define traits and implement them on any type, including unit-like structs.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

11. The `println!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting known as _Display_: output intended for direct end user consumption. The primitive types we’ve seen so far implement _Display_ by default, because there’s only one way you’d want to show a _1_ or any other primitive type to a user. But with **structs**, the way `println!` should format the output is less clear because there are more display possibilities: _Do you want commas or not?_ _Do you want to print the curly brackets?_ _Should all the fields be shown?_ Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of _Display_ to use with `println!` and the `{}` placeholder.
12. The `println!` macro call will now look like `println!("rect1 is {:?}", rect1);`. Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called _Debug_. The _Debug_ trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.
13. Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute `#[derive(Debug)]` just before the struct definition,

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {}
```

14. When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.
15. Another way to print out a value using the `Debug` format is to use the `dbg!` macro, which takes ownership of an expression, prints the file and line number of where that `dbg!` macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value. Calling the `dbg!` macro prints to the standard error console stream (`stderr`), as opposed to `println!` which prints to the standard output console stream (`stdout`).
16. _Methods_ are similar to functions: we declare them with the `fn` keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a `struct` (or an `enum` or a `trait object`, which we cover in Chapters 6 and 17, respectively), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

// method
impl Rectangle {
    fn area(&self) -> u64 {
        (self.width * self.height).into()
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let r = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of a rectangle {}x{} equals {}.", r.width, r.height, r.area());
}
```

17. To define the function within the context of `Rectangle`, we start an `impl` (implementation) block for Rectangle. Everything within this `impl` block will be associated with the `Rectangle` type. Then we move the area function within the `impl` curly brackets and change the first (and in this case, only) parameter to be `self` in the signature and everywhere within the body. In main, where we called the area function and passed `rect1` as an argument, we can instead use method syntax to call the area method on our `Rectangle` instance. The method syntax goes after an instance: we add a _dot_ followed by the method name, parentheses, and any arguments.
18. The `&self` is actually short for `self: &Self`. Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for. _Methods_ must have a parameter named `self` of type `Self` for their first parameter, so `Rust` lets you abbreviate this with only the name self in the first parameter spot. Note that we still need to use the `&` in front of the self shorthand to indicate this method borrows the `Self` instance, just as we did in `rectangle: &Rectangle`. Methods can take ownership of `self`, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.
19. We’ve chosen `&self` here for the same reason we used `&Rectangle` in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use `&mut self` as the first parameter. Having a method that takes ownership of the instance by using just `self` as the first parameter is rare; **this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation**.
20. Often, but not always, when we give methods with the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called _getters_, and Rust does not implement them automatically for struct fields as some other languages do.
21. Rust doesn’t have an equivalent to the `->` operator (i.e. C/C++); instead, Rust has a feature called **automatic referencing** and **dereferencing**. Calling methods is one of the few places in Rust that has this behavior.
22. Here’s how it works: when you call a method with `object.something()`, Rust automatically adds in `&,` `&mut`, or `*` so object matches the signature of the method. In other words, the following are the same:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

22. The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of `self`. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (`&self`), mutating (`&mut self`), or consuming (`self`). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
23. All functions defined within an `impl` block are called **associated functions** because they’re _associated_ with the type named after the `impl`. We can define **associated functions** that don’t have `self` as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the `String::from` function that’s defined on the `String` type.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated function
    fn square_rectangle(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    //
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height < other.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 20,
        height: 10,
    };
    let r2 = Rectangle::square_rectangle(4);
    println!("The area of a rectangle {}x{} equals {}.", r1.width, r1.height, r1.area());
    println!("The area of a rectangle {}x{} equals {}.", r2.width, r2.height, r2.area());
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
}
```

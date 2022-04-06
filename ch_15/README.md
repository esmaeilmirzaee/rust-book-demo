1. A `pointer` is a general concept for _a variable that contains an address in memory_. This address refers to, or “points at,” some other data. The most common kind of `pointer` in _Rust_ is a reference. **References** are indicated by the `&` symbol and borrow the value they point to. They don’t have any special capabilities other than referring to data. Also, they don’t have any overhead and are the kind of `pointer` we use most often.
2. **Smart pointers**, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities. One example that we’ll explore in this chapter is the reference counting smart pointer type. This pointer enables you to have multiple owners of data by keeping track of the number of owners and, when no owners remain, cleaning up the data.
3. In _Rust_, which uses the concept of ownership and borrowing, an additional difference between references and **smart pointers** is that references are pointers that only borrow data; in contrast, in many cases, **smart pointers** own the data they point to.
4. Both these types (i.e. `String`, `Vec<T>`) count as **smart pointers** because they own some memory and allow you to manipulate it. They also have metadata (such as their capacity) and extra capabilities or guarantees (such as with String ensuring its data will always be valid UTF-8).
5. **Smart pointers** are usually implemented using **structs**. The characteristic that distinguishes a smart pointer from an ordinary struct is that **smart pointers** implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or **smart pointers**. The `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope. Some important **smart pointers** in the standard library:

    - `Box<T>` for allocating values on the _heap_
    - `Rc<T>`, a **reference counting** type that enables multiple ownership
    - `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

6. The most straightforward smart pointer is a box, whose type is written `Box<T>`. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.
7. **Boxes don’t have performance overhead**, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

    - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

```rust
fn main() {
    let b = Box::new(5);
    println!("{}", b);
}
```

8. We define the variable `b` to have the value of a Box that points to the value 5, which is allocated on the heap. This program will print `b = 5;` in this case, we can access the data in the box similar to how we would if this data were on the stack. Just like any owned value, when a box goes out of scope, as `b` does at the end of main, it will be deallocated. The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).
9. At compile time, _Rust_ needs to know how much space a type takes up. One type whose size can’t be known at compile time is a recursive type, where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, _Rust_ doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.
10. At compile time, _Rust_ needs to know how much space a type takes up. One type whose size can’t be known at compile time is a recursive type, where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, _Rust_ doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types. The cons function concept has made its way into more general functional programming jargon: “to cons x onto y” informally means to construct a new container instance by putting the element x at the start of this new container, followed by the container y. Although functional programming languages use _cons lists_ frequently, the cons list isn’t a commonly used data structure in _Rust_. Most of the time when you have a list of items in _Rust_, `Vec<T>` is a better choice to use. Other, more complex recursive data types are useful in various situations, but by starting with the cons list, we can explore how boxes let us define a recursive data type without much distraction.

```rust
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

11. First, let’s look at how _Rust_ decides how much space it needs to store a value of a non-recursive type. To determine how much space to allocate for a `Message` value, _Rust_ goes through each of the variants to see which variant needs the most space. _Rust_ sees that `Message::Quit` doesn’t need any space, `Message::Move` needs enough space to store two `i32` values, and so forth. Because only one variant will be used, the most space a `Message` value will need is the space it would take to store the largest of its variants.
12. In this suggestion (i.e. an error message), **“indirection”** means that instead of storing a value directly, we’ll change the data structure to store the value indirectly by storing a pointer to the value instead.
13. Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other **smart pointer** types. They also don’t have any performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need.
14. The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation.
15. Implementing the `Deref` trait allows you to customize the behavior of the dereference operator, `*` (as opposed to the multiplication or glob operator). By implementing `Deref` in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with **smart pointers** too.
16. A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.
17. The `Deref` trait, provided by the standard library, requires us to implement one method named `deref` that borrows self and returns a reference to the inner data.

```rust
use std::ops::Deref;
enum MyBox<T> (T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

18. The `type Target = T;` syntax defines an associated type for the `Deref` trait to use. Associated types are a slightly different way of declaring a generic parameter.
19. Without the `Deref` trait, the compiler can only dereference `&` references. The deref method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get a `&` reference that it knows how to dereference.
20. When we entered `*y`, behind the scenes _Rust_ actually ran this code:

```rust
*(y.deref());
```

21. _Rust_ substitutes the `*` operator with a call to the `deref` method and then a plain dereference so we don’t have to think about whether or not we need to call the `deref` method. This _Rust_ feature lets us write code that functions identically whether we have a regular reference or a type that implements `Deref`.
22. `Deref` coercion is a convenience that _Rust_ performs on arguments to functions and methods. `Deref` coercion works only on types that implement the `Deref` trait. `Deref` coercion converts such a type into a reference to another type. For example, `deref` coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `&str`. `Deref` coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the `deref` method converts the type we provided into the type the parameter needs.
23. When the `Deref` trait is defined for the types involved, _Rust_ will analyze the types and use `Deref::deref` as many times as necessary to get a reference to match the parameter’s type. The number of times that `Deref::deref` needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!
24. Similar to how you use the `Deref` trait to override the operator on immutable references, you can use the `DerefMut` trait to override the operator on mutable references.
25. `Rust` does `deref` coercion when it finds types and trait implementations in three cases:

    From `&T` to `&U` when `T: Deref<Target=U>`
    From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
    From `&mut T` to `&U` when `T: Deref<Target=U>`

26. The first two cases are the same except for mutability. The first case states that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can get a `&U` transparently. The second case states that the same `deref` coercion happens for mutable references. The third case is trickier: _Rust_ will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile). Converting one mutable reference to one immutable reference will never break the borrowing rules. Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that. Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.
27. In some languages, the programmer must call code to free memory or resources every time they finish using an instance of a smart pointer. If they forget, the system might become overloaded and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically. As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources!

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping a custom smart pointer. {}", self.data);
    }
}
```

28. Specify the code to run when a value goes out of scope by implementing the `Drop` trait. The `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to `self`.
29. Unfortunately, it’s not straightforward to disable the automatic `drop` functionality. Disabling `drop` isn’t usually necessary; the whole point of the `Drop` trait is that it’s taken care of automatically. Occasionally, however, you might want to clean up a value early. One example is when using smart pointers that manage _locks_: you might want to force the `drop` method that releases the _lock_ so that other code in the same scope can acquire the _lock_. _Rust_ doesn’t let you call the `Drop` trait’s `drop` method manually; instead you have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope.
30. A **destructor** (i.e. is the general programming term for a function that cleans up an instance) is analogous to a **constructor**, which creates an instance.
31. _Rust_ doesn’t let us call `drop` explicitly because _Rust_ would still automatically call `drop` on the value at the end of main. This would be a double free error because _Rust_ would be trying to clean up the same value twice.

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("A string"),
    };
    drop(c);
    println!("This can't be done. {}", c.data);
}
```

32. The `std::mem::drop` function is different from the `drop` method in the `Drop` trait. We call it by passing the value we want to force to be dropped early as an argument. The function is in the prelude.
33. You can use code specified in a `Drop` trait implementation in many ways to make cleanup convenient and safe: for instance, you could use it to create your own memory `allocator!` With the `Drop` trait and _Rust_’s ownership system, you don’t have to remember to clean up because _Rust_ does it automatically. You also don’t have to worry about problems resulting from accidentally cleaning up values still in use: the ownership system that makes sure references are always valid also ensures that `drop` gets called only once when the value is no longer being used.
34. To enable multiple ownership, _Rust_ has a type called `Rc<T>`, which is an abbreviation for reference counting. The `Rc<T>` type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
35. We use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil
}

fn main() {
    let a = Rc::new(Cons(3, Rc::new(4, Nil)));
    let b = Rc::new(Cons(2, Rc::clone(&a)));
    let c = Rc::new(Cons(1, Rc::new(Cons(2, Rc::clone(&a)))));
}
```

36. The implementation of `Rc::clone` doesn’t make a deep copy of all the data like most types’ implementations of clone do. The call to `Rc::clone` only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot of time. By using `Rc::clone` for reference counting, we can visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count. When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to `Rc::clone`.
37. Using `Rc<T>` allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist.

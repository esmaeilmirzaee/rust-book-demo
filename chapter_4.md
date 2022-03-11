1. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no-longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.
2. Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.
3. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. **Data with an unknown size at compile time or a size that might change must be stored on the heap instead.**
4. The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating. Pushing values onto the stack is not considered allocating. Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
5. Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; _that location is always at the top of the stack_.
6. Allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
7. Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory.
8. Allocating a large amount of space on the heap can also take time.
9. When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
10. Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.
11. First, let’s take a look at the ownership rules.
    11.1. Each value in Rust has a variable that’s called its owner.
    11.2. There can only be one owner at a time.
    11.3. When the owner goes out of scope, the value will be dropped.
12. A scope is the range within a program for which an item is valid. The variable is valid from the point at which it’s declared until the end of the current scope.
13. The types covered previously are all a known size, can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope. But we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the String type is a great example.
14. We’ve already seen string literals, where a string value is hardcoded into our program. String literals are convenient, but they aren’t suitable for every situation in which we may want to use text. One reason is that they’re immutable. Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? For these situations, Rust has a second string type, String. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the from function, like so:

```rust
fn main() {
    let s = String::from("Hello");
    s.push_str(", World");
    println!("{}", s)
}
```

15. In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.
16. With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
    16.1. The memory must be requested from the memory allocator at runtime.
    16.2. We need a way of returning this memory to the allocator when we’re done with our String.
17. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and call code to explicitly return it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
18. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.
19. **In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.**

```rust
fn main() {
    let s1 = String::from("Hello, World");
    let s2 = s1;
}
```

20. Take a look at Figure 4-1 to see what is happening to String under the covers. A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
21. _The length is how much memory, in bytes, the contents of the String is currently using._ **The capacity is the total amount of memory, in bytes, that the String has received from the allocator.**
22. Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. But Figure 4-2 shows both data pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
23. To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work:

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s1);
}
```

24. If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of calling it a shallow copy, it’s known as a move.
25. If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of calling it a shallow copy, it’s known as a move.
26. If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("S1: {}, S2: {}", s1, s2);
}
```

27. Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying and we can leave it out.
28. Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.
29. So what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:

    -   All the integer types, such as u32.
    -   The Boolean type, bool, with values true and false.
    -   All the floating point types, such as f64.
    -   The character type, char.
    -   Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

30. The semantics for passing a value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.
31. Returning values can also transfer ownership. But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for using a value without transferring ownership, called references.
32. The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
33. We can provide a reference to the String value. A reference is like a pointer in that it’s an address we can follow to access data stored at that address that is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type.
34. These ampersands represent references, and they allow you to refer to some value without taking ownership of it. The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, \*.
35. The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because it does not own it, the value it points to will not be dropped when the reference stops being used. Likewise, the signature of the function uses & to indicate that the type of the parameter s is a reference.
36. Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
37. First, we change s to be mut. Then we create a mutable reference with &mut s where we call the change function, and update the function signature to accept a mutable reference with some_string: &mut String. This makes it very clear that the change function will mutate the value it borrows.
38. **Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time.**

```rust
fn main() {
    let mut s = String::from("Lorem ipsum");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{} {}", r1, r2);
}
```

39. This error says that this code is invalid because we cannot borrow s as mutable more than once at a time. The first mutable borrow is in r1 and must last until it’s used in the println!, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.
40. The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:
    -   Two or more pointers access the same data at the same time.
    -   At least one of the pointers is being used to write to the data.
    -   There’s no mechanism being used to synchronize access to the data.
41. We also cannot have a mutable reference while we have an immutable one to the same value. Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
42. The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed. The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short)
43. In languages with pointers, it’s easy to erroneously create a dangling pointer--a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory. **In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.**
44. > At any given time, you can have either one mutable reference or any number of immutable references. Also, references must always be valid.
45. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
46. Because we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method: `let bytes = string_variable.as_bytes();`
47. Next, we create an iterator over the array of bytes using the iter method: `for (i, &item) in bytes.iter().enumerate();`
48. For now, know that iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead. The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element.
49. We search for the byte that represents the space by using the byte literal syntax. `if item == b' ' {}`

```rust
fn main() {
    let s = String::from("Hello World");
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!(i);
        }
    }
}
```

50. A string slice is a reference to part of a String, and it looks like this:

```rust
fn main() {
    let msg = String::from("lorem ipsum");
    let lorem = &msg[0..4]; // &msg[..4];
    let ipsum = &msg[5..10]; // &msg[5..];
}
```

50. Rather than a reference to the entire String, `lorem` is a reference to a portion of the String, specified in the extra [0..4] bit. We create slices using a range within brackets by specifying [starting_index..ending_index], where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in the slice.
51. String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. For the purposes of introducing string slices, we are assuming ASCII only in this section.
52. Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. The println! after the call to clear uses the reference in word, so the immutable reference must still be active at that point. Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails. Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!
53. Recall that we talked about string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals:

```rust
fn main() {
    let s = "Hello, Rust!";
}
```

53. The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.
54. A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.

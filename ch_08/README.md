1. _Rust_’s standard library includes a number of very useful data structures called **collections**. Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in `array` and `tuple` types, the data these collections point to is stored on the `heap`, which means **the amount of data does not need to be known at compile time and can grow or shrink as the program runs**. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. In this chapter, we’ll discuss three collections that are used very often in Rust programs:

    - A `vector` allows you to store a variable number of values next to each other.
    - A `string` is a collection of characters. We’ve mentioned the `String` type previously, but in this chapter we’ll talk about it in depth.
    - A `hash map` allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a `map`.

2. The first collection type we’ll look at is `Vec<T>`, also known as a _vector_. Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

```rust
let v: Vec<i32> = Vec::new();
```

3. Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store. This is an important point. Vectors are implemented using generics. For now, know that the `Vec<T>` type provided by the standard library can hold any type, and when a specific vector holds a specific type, the type is specified within angle brackets.
4. In more realistic code, Rust can often infer the type of value you want to store once you insert values, so you rarely need to do this type annotation. It’s more common to create a `Vec<T>` that has initial values, and Rust provides the vec! macro for convenience. The macro will create a new vector that holds the values you give it. As with any variable, if we want to be able to change its value, we need to make it mutable using the `mut` keyword.

```rust
let v = vec![1, 2, 3];
v.push(4);
```

5. Note two details here. First, we use the index value of 2 to get the third element: `vector`s are indexed by number, starting at zero. Second, the two ways to get the third element are by using `&` and `[]`, which gives us a reference, or by using the get method with the index passed as an argument, which gives us an `Option<&T>`.
6. When the `get` method is passed _an index that is outside the_ `vector`, it returns `None` without panicking. You would use this method if accessing an element beyond the range of the vector happens occasionally under normal circumstances. Your code will then have logic to handle having either `Some(&element)` or `None`.

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let first = &v[0]; // immutable borrow occurs here
    v.push(5); // mutable borrow occurs here
    println!("The first element in vector is {}.", first);
}
```

7. Why should a reference to the first element care about what changes at the end of the vector? This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.
    > Recall the rule that states you can’t have mutable and immutable references in the same scope.
8. At the beginning of this chapter, we said that vectors can only store values that are the same type. This can be inconvenient; there are definitely use cases for needing to store a list of items of different types. Fortunately, the variants of an enum are defined under the same enum type, so when we need to store elements of a different type in a vector, we can define and use an enum!
9. Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. A secondary advantage is that we can be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the `vector`. Using an `enum` plus a match expression means that Rust will ensure at compile time that every possible case is handled.
10. New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8. These factors combine in a way that can seem difficult when you’re coming from other programming languages.
11. We’ll first define what we mean by the term string. Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`. In Chapter 4, we talked about _string slices_, which are references to some `UTF-8` encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices. The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
12. When Rustaceans refer to “strings” in _Rust_, they usually mean the `String` and the string slice `&str` types, not just one of those types.
13. Rust’s standard library also includes a number of other string types, such as `OsString`, `OsStr`, `CString`, and `CStr`. Library crates can provide even more options for storing string data. See how those names all end in `String` or `Str`? They refer to **owned** and **borrowed variants**, just like the `String` and `str` types you’ve seen previously. These string types can store text in different encodings or be represented in memory in a different way, for example.
14. Many of the same operations available with `Vec<T>` are available with `String` as well, starting with the new function to create a string, `let s = String::new();`.
15. This line creates a new empty string called `s`, which we can then load data into. Often, we’ll have some initial data that we want to start the string with. For that, we use the `to_string` method, which is available on any type that implements the `Display` trait, as `string` literals do.

```rust
fn main() {
    let s = "This is a literal string.".to_string();
    println!("{}", s);
}
```

16. A `String` is a wrapper over a `Vec<u8>`. `let hello = String::from("Hola"); ` In this case, `len` will be `4`, which means the `vector` storing the string `“Hola”` is 4 bytes long. Each of these letters takes 1 byte when encoded in `UTF-8`.
17. To avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile `let answer = &hello[0];` at all and prevents misunderstandings early in the development process.
18. Another point about `UTF-8` is that there are actually three relevant ways to look at strings from Rust’s perspective: as **bytes**, **scalar values**, and **grapheme clusters** (the closest thing to what we would call letters). Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.
19. A final reason Rust doesn’t allow us to index into a `String` to get a character is that indexing operations are expected to always take constant time `(O(1))`. But it isn’t possible to guarantee that performance with a `String`, **because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were**.
20. Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. Therefore, Rust asks you to be more specific if you really need to use indices to create string slices. To be more specific in your indexing and indicate that you want a string slice, rather than indexing using `[]` with a single number, you can use `[]` with a range to create a string slice containing particular bytes:

```rust
fn main() {
    let hello = String::from("Halo");
    let h = &hello[1..4];
}
```

20. Here, `h` will be a `&str` that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters was 2 bytes, which means `h` will be `Зд`.
21. If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the `chars` method. Calling chars on `“नमस्ते”` separates out and returns six values of type `char`, and you can iterate over the result to access each element:

```rust
fn main() {
    let str = String::from("नमस्ते");
    for char in str.chars() {
        println!("{}", char);
    }
}
```

22. To summarize, strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the correct handling of `String` data the default behavior for all Rust programs, which means programmers have to put more thought into handling `UTF-8` data upfront. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving `non-ASCII` characters later in your development life cycle.
23. The `push_str` method takes a string slice because we don’t necessarily want to take ownership of the parameter. The push method takes a single character as a parameter and adds it to the String.
24. The `format!` macro works in the same way as `println!`, but instead of printing the output to the screen, _it returns a String with the contents_. The version of the code using `format!` is much easier to read, and the code generated by the `format!` macro uses references so that this call doesn’t take ownership of any of its parameters.
25. The last of our common collections is the hash map. The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`. It does this via a hashing function, which determines how it places these keys and values into memory. Many programming languages support this kind of data structure, but they often use a different name, such as `hash`, `map`, `object`, `hash table`, `dictionary`, or `associative array`, just to name a few

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert("Yellow".to_string(), 50);
}
```

26. Note that we need to first use the `HashMap` from the collections portion of the standard library. Of our three common collections, this one is the least often used, so it’s not included in the features brought into scope automatically in the prelude. **Hash maps also have less support from the standard library**; _there’s no built-in macro to construct them_, for example.
27. **Just like vectors, hash maps store their data on the heap**. Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.
28. For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

```rust
use std::collections::HashMap;
fn main() {
    let scores: HashMap<String, i32> = HashMap::new();
    let team_name = String::from("Blue");
    scores.insert(&team_name, 10);
    let team_score = match scores.get(&team_name) {
        Some(&team_score) => team_score,
        None => println!("Score for the team is not provided."),
    };
}
```

29. Here, score will have the value that’s associated with the Blue team, and the result will be `Some(&10)`. The result is wrapped in Some because get returns an `Option<&V>`; if there’s no value for that key in the hash map, get will return `None`.
30. If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
}
```

31. It’s common to check whether a particular key has a value and, if it doesn’t, insert a value for it. Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter. The return value of the `entry` method is an `enum` called `Entry` that represents a value that might or might not exist. Let’s say we want to check whether the key for the Yellow team has a value associated with it. If it doesn’t, we want to insert the value 50, and the same for the Blue team.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
```

32. The `or_insert` method on `Entry` is defined to return a _mutable_ reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a _mutable_ reference to the new value. This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.
33. Another common use case for hash maps is to look up a key’s value and then update it based on the old value.

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    ley mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{}", map);
}
```

34. By default, `HashMap` uses a hashing function called **SipHash** that can provide resistance to _Denial of Service (DoS)_ attacks involving hash tables1. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A hasher is a type that implements the `BuildHasher` trait.
35. Here are some exercises you should now be equipped to solve:

    -   Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    -   Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    -   Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

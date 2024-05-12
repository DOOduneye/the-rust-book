# Ownership

Memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

## Stack and Heap

### Stack

The stack stores memory in **last in, first out**. The stack is fast because of the way it stores and accesses data. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

### Heap

The heap is less organized: when you put data on the heap, you request a certain amount of space. The operating system finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called **allocating on the heap** and is sometimes abbreviated as just **allocating**. Pushing values onto the stack is not considered allocating.

## Ownership Rules

1. Each value in Rust has an *owner*.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Mutable References

You can have only one mutable reference to a particular piece of data in a particular scope. This code will not compile:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

### Rules of References

1. You can have as many immutable references as you want.
2. You can have exactly one mutable reference. 
3. You cannot combine mutable and immutable references. Albiet this is possible:
    ```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point as they are no longer referenced

    let r3 = &mut s; // no problem
    println!("{}", r3);
    ```
4. You can only multiple mutable references if they are in different scopes.

## Data Races

A data race is similar to a race condition and happens when these three behaviors occur:

1. Two or more pointers access the same data at the same time.
2. At least one of the pointers is being used to write to the data.
3. There’s no mechanism being used to synchronize access to the data.

## Dangling References

A dangling reference is a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory. Rust ensures that references will never be dangling references.

**References must always be valid.**

## Slices

A slice is a reference to a contiguous sequence of elements in a collection. Slices are a more general form of a reference.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

## Citation

This content was taken from the [Rust Book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) and is licensed under the Creative Commons Attribution 4.0 International License. To view a copy of this license, visit [https://creativecommons.org/licenses/by/4.0/](https://creativecommons.org/licenses/by/4.0/).

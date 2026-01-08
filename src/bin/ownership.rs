fn main() {
    /*
    Ownership is the "secret sauce" that makes Rust unique.
    It is the language's way of managing memory without needing a Garbage Collector
    (like Java or Python) and without making the programmer manage memory manually (like C or C++).

    1. What is Ownership exactly?
    Ownership is a set of rules the Rust compiler checks at compile time. If the rules are broken, the program won't compile.
    There are three main rules:
        - Each value in Rust has a variable that’s called its owner.
        - There can only be one owner at a time.
        - When the owner goes out of scope, the value will be dropped (deleted).

    2. Why do we need it? (The Problem it Solves)
    Computers have two types of memory: the Stack and the Heap.
        - The Stack is fast but requires data to have a fixed, known size (like int, bool).
        - The Heap is for data that grows (like a String), but it’s harder to manage.
    The Problem: In other languages, you either have a Garbage Collector (GC) that pauses your
    program to clean up the heap (slow), or you have to manually free memory.
    If you forget to free it, you get a Memory Leak. If you free it twice, your program crashes.

    The Solution: Ownership ensures that memory is cleaned up automatically and predictably the
    moment it is no longer needed, with zero performance cost at runtime.
    */

    /*
        Example A The "Move" (One Owner Only)
        When you assign an "owned" type (like a String) to another variable,
        Rust moves the data rather than copying it.
    */
    let s1: String = String::from("hello");
    let s2: String = s1; // The ownership of "hello" MOVES to s2

    // println!("{}", s1); // ❌ ERROR! s1 no longer owns the data.
    println!("{}", s2); // ✅ This works.

    /*
        Example B: Scope and Dropping
        Rust automatically cleans up memory the instant a variable hits a closing curly brace }.
    */
    {
        let s: String = String::from("world"); // s is valid from this point forward
        println!("{}", s);
        // do more stuff with s
    } // 🗑️ This scope is over. Rust calls 'drop' and the memory is freed!
    // println!("{}", s); // ❌ ERROR! s is gone.

    /*
        Example C: Passing to Functions
        Passing a variable to a function also moves ownership.
    */
    let s: String = String::from("hello");
    take_ownership(s); // ownership moves into the function

    // println!("{}", s); // ❌ ERROR! The function "ate" s and then dropped it.

    /*
        Example D: The "Copy" Types (Stack Only)
        For types like i32 or bool, Rust implements the Copy trait.
        When you assign x = y, it simply duplicates the bits. Both variables are still valid.
    */
    let x = 5;
    let y = x; // x is copied to y
    println!("{}, {}", x, y); // ✅ Works! (Stack data is cheap to duplicate)

    /*
        Example E The Clone (Explicit)
        For "owned" types that involve the Heap (like String or Vec), copying is "expensive."
        Rust makes you do it manually so you are aware of the performance cost.
    */
    let s1: String = String::from("hello");
    let s2: String = s1.clone(); // Deep copy created on the heap

    println!("s1: {}, s2: {}", s1, s2); // ✅ BOTH work!
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
} // 🗑️ some_string is dropped here.

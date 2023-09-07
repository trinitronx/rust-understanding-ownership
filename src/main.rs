fn main() {
    // Scope example
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("variable `s` is: {s}");
    } // this scope is now over, and s is no longer valid

    // The String Type
    strings();

    // Variables and Data Interacting with Move
    stack_vs_heap();

    // Variables and Data Interacting with Clone
    clone_example();

    // Stack-Only Data: Copy
    stack_only_data_copy();

    // Ownership and Functions
    ownership_and_functions();
}

// Example: The String Type
fn strings() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

// Example: Variables and Data Interacting with Move
fn stack_vs_heap() {
    // Size is known at compile-time, so stored on the Stack
    let x = 5;
    let y = x;
    println!("`y` is stack-allocated: {y}");

    // String type is growable
    // Size is unknown at compile-time, so must be Heap-allocated
    let s1 = String::from("hello");
    let s2 = s1; // s1 is "moved": s2 copied s1's internal values: ptr, len, capacity

    // now s1 is considered invalid, because it was "moved"
    // println!("{}, world!", s1); // borrow of moved value: `s1`
    println!("{}, world!", s2);
} // s2 goes out of scope, and Heap memory is freed / drop()

// Example: Variables and Data Interacting with Clone
fn clone_example() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

// Example: Stack-Only Data: Copy
fn stack_only_data_copy() {
    // Simple scalar types can be copied directly on the stack
    let x = 5;
    let y = x; // This value is copied on the stack

    println!("x = {}, y = {}", x, y);

    // only simple scalar / integer types... Can be Copied
    let s1 = String::from("hello");
    let tup = (1, 2, 3); //  (i32, i32) implements Copy
    let tup2 = (1, 2, 3, s1); // This mixed-type tuple includes a String
    let tup3 = tup2; // This cannot be copied directly on the stack
                     // (String does not and cannot implement the Copy trait)
    println!("tup is: {:?}", tup); // (i32, i32, i32, String) does not implement Copy due to including String
    println!("tup3 is: {:?}", tup3); // (i32, i32, i32, String) does not implement Copy due to including String
                                     // println!("tup2 is: {:?}", tup2); // Compiler error borrow of moved value: `tup2`
}

// Example: Ownership and Functions
fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        // println!("`s` is {s}"); // Compiler Error: borrow of moved value: `s`
    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("`x` is still valid: {x}");
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("some_string: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("some_integer: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

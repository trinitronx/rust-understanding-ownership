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

    // Return Values and Scope
    return_values_and_scope();

    // Return Values and Scope - Tuples
    return_values_and_scope_tuples();

    // References and Borrowing
    references_and_borrowing();

    // Attempting to modify a borrowed value
    attempted_modify_borrowed_value();

    // Mutable References
    mutable_references();

    // Attempting Two Mutable References
    attempted_two_mutable_references();

    // Multiple Mutable References
    multiple_mutable_references();

    // Attempting Mixed Mutable/Immutable References
    attempted_mixed_mutable_immutable_references();

    // Fixed Dangling Pointers
    no_dangling_pointers();

    // The Slice Type
    naiive_slice_implementation_mutable_string_offset();

    // String Slices
    string_slices();
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

// Example: Return Values and Scope
fn return_values_and_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// Example: Return Values and Scope - Tuples
fn return_values_and_scope_tuples() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    // println!("`s1` is '{}'", s1); // Compiler Error: borrow of moved value: `s1`
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// Example: References and Borrowing
fn references_and_borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length2(&s1); // passing a reference to s1: &s1

    println!("The length of '{}' is {}.", s1, len); // Note: s1 still valid here b/c it wasn't moved
}

fn calculate_length2(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// Example: Attempting to modify a borrowed value
fn attempted_modify_borrowed_value() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    println!("change(): `some_string` is {some_string}");
    // some_string.push_str(", world"); // Compile Error: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
}

// Example: Mutable References
fn mutable_references() {
    let mut s = String::from("hello");

    println!("mutable_references(): `s` initialized to {s}");
    change2(&mut s);
    println!("mutable_references(): `s` is {s}");
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
    println!("change2(): `some_string` is {some_string}");
}

// Example: Attempting Two Mutable References
fn attempted_two_mutable_references() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // Compile Error: cannot borrow `s` as mutable more than once at a time
    // Clone of a dereferenced string ref desn't work
    // let r2 = *r1.clone(); // Compile Error: the size for values of type `str` cannot be known at compilation time
    let r2 = r1.clone(); // Clone of a reference works b/c size of reference / pointer is known @ compile time
    println!(
        "attempted_two_mutable_references(): `r1`, `r2` = {}, {}",
        r1, r2
    );
}

// Example: Multiple Mutable References
fn multiple_mutable_references() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

// Example: Attempting Mixed Mutable/Immutable References
fn attempted_mixed_mutable_immutable_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // Compiler Error: cannot borrow `s` as mutable because it is also borrowed as immutable
                 // let r3 = &mut s; // BIG PROBLEM
    let r3 = &mut s.clone(); // Clone is OK
    r3.push_str("?"); // Mutating r3 reference modifies the cloned value ONLY

    println!(
        "attempted_mixed_mutable_immutable_references(): {}, {}, and {}",
        r1, r2, r3
    );

    // Note: ref scope starts from where it is introduced
    // and continues through the last time that reference is used
    let mut s2 = String::from("hello");

    let r3 = &s2; // no problem
    let r4 = &s2; // no problem
    println!("{} and {}", r3, r4);
    // variables r3 and r4 will not be used after this point

    // Now r3 and r4 are out of scope, so new mutable ref is OK
    let r3 = &mut s2; // no problem
    println!("{}", r3);
    // These scopes don’t overlap, so this code is allowed:
    // the compiler can tell that the reference is no longer being used at a
    // point before the end of the scope.
}

// Example: Dangling Pointers
// fn dangling_pointers() {
//     // Since `s` variable lifetime lives only inside the function below,
//     // this does not compile
//     let reference_to_nothing = dangle();
// }

// // Compile Error: missing lifetime specifier
// // help: this function's return type contains a borrowed value, but there is no
// // value for it to be borrowed from
// fn dangle() -> &String {  // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     // we return a reference to the String, s
//     &s // returns ref to `s`, but `s` has local scope!
// }// Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// Example: Fixed Dangling Pointers
fn no_dangling_pointers() {
    // Since `s` variable lifetime lives only inside the function below,
    // this does not compile
    let reference_to_moved_ownership_value = no_dangle();
    println!("no_dangling_pointers(): reference_to_moved_ownership_value: {reference_to_moved_ownership_value}");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s // Ownership of `s` is moved out of the function
}

// Example: The Slice Type
// A function to return the first word of a string
/// first_word function returns a byte index value into the String parameter
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// usize integer offset is similar to slice &str, but
// does not share lifetime with the String
// So, mutating the String causes the offset to lose meaning & context
fn naiive_slice_implementation_mutable_string_offset() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // The following will cause a runtime error:
    //   thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 5', src/main.rs:349:9
    // println!(
    //     "naiive_slice_implementation_mutable_string_offset(): `s[word]` is: {}",
    //     s.as_bytes()[word]
    // );
}

// Example: String Slices
fn string_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("string_slices(): {} {}", hello, world);

    // Equivalent syntax
    let slice1 = &s[0..2]; // "he"
    let slice2 = &s[..2]; // "he"
    println!(
        "string_slices(): `slice1`:  {}, `slice2`: {}",
        slice1, slice2
    );

    // End of string equivalent syntax
    let len = s.len();
    // These are both the same
    let slice3 = &s[3..len]; // "lo world"
    let slice4 = &s[3..]; // "lo world"
    println!(
        "string_slices(): `slice3`:  {}, `slice4`: {}",
        slice3, slice4
    );

    // Entire String as slice
    let len = s.len();
    // Equivalent syntax
    let slice5 = &s[0..len]; // "hello world"
    let slice6 = &s[..]; // "hello world"
    println!(
        "string_slices(): `slice5`:  {}, `slice6`: {}",
        slice5, slice6
    );
}

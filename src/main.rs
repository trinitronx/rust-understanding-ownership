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
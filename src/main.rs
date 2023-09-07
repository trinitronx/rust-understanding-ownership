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
}

// Example: The String Type
fn strings() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

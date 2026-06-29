fn main() {
    let mut s = String::from("Hello"); // Create a new String with content "Hello"
    s.push_str(", world!"); // Append the string ", world!" to the original String
    println!("{}", s); // Output: "Hello, world!"

    let s: &str = "Hello"; // Create a new &str with content "Hello"
    // The following line would produce a compile-time error since &str is immutable:
    // s.push_str(", world!");
    println!("{}", s); // Output: "Hello"

    let s = String::from("Hello"); // Create a new String with content "Hello"
    let s_slice: &str = &s; // Create a &str slice that borrows from the String

    println!("String: {}", s); // Output: "Hello"
    println!("&str: {}", s_slice); // Output: "Hello"
}

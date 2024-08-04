use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Example 1: Confusion Around the `let` Keyword
    let mut x = 10;  // Declare x as mutable
    x = 20;  // Now reassignment is allowed
    println!("Example 1 - Confusion Around the `let` Keyword: {}", x);

    // Example 2: Misunderstanding Ownership and Borrowing
    let s = String::from("Hello, Rust!");
    let s2 = &s;  // Borrowing s, s is still valid
    println!("Example 2 - Misunderstanding Ownership and Borrowing:");
    println!("s: {}", s);
    println!("s2: {}", s2);

    // Example 3: Ignoring Error Handling - Using match for error handling
    match read_file() {
        Ok(contents) => println!("Example 3 - Ignoring Error Handling:\n{}", contents),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read_file() -> io::Result<String> {
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

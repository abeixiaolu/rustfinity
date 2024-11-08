use std::{fs::File, io::Write};

use sha256::digest;

fn main() {
    greet();
    add(5, 3);
    curly_braces_block();
    println!("add2: {}", add2(5, 3));
    println!("generate_sha_256_hash: {}", generate_sha_256_hash("Hello, world!"));
    write_to_file("Hello, world!", "hello.txt");
}

fn greet() {
    println!("Hello, Function!")
}

fn add(a: i32, b: i32) {
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);
}

fn curly_braces_block() {
    let x = {
        let y = 5;
        y * 2
    };
    println!("The value of x is: {}", x);
}

// it's preferred for functions to return a value rather than mutating outside state.
fn add2(a: i32, b: i32) -> i32 {
    //! We can remove the return keyword and the semicolon
    // return a + b;
    a + b
}

// Functions that do not modify external state and always produce the same output
// for the same input are called pure function.

fn generate_sha_256_hash(input: &str) -> String {
    let hash = digest(input.as_bytes());
    hash
}

fn write_to_file(data:&str, filename: &str) {
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(data.as_bytes()).expect("Failed to write to file");
}
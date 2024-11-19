fn main() {
    let a = 1;
    let b = a;
    println!("a: {}", a); // No compile error, because the value of a is copied, not moved
    println!("b: {}", b);
    println!("complex type");
    let a = String::from("hello");
    let b = a;
    let c = b.clone();
    // println!("a: {}", a); // Compile error, because a is moved to b
    println!("b: {}", b);
    println!("c: {}", c);
}

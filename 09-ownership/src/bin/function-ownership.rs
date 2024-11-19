fn main() {
    let s = String::from("Hello");
    // takes_ownership(s);
    // println!("s is {}", s); // Compile error, because s is moved to the function

    let s = takes_ownership(s);
    println!("Gives back the ownership of s is {}", s);
}

fn takes_ownership(s: String) -> String {
    println!("s is {}", s);
    let s = s.repeat(10);
    s
}

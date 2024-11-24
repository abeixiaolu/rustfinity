fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("r1 is {}", r1);
    println!("r2 is {}", r2);
    //  if we use the immutable references first and then mutating the value,
    // the compiler can still guarantee that the immutable references
    // will not change and point to the data that we are expecting.
    let r3 = &mut s;
    println!("r3 is {}", r3);

    let mut ss = String::from("hello");
    let ss1 = &mut ss;
    ss1.push_str(", world");
    let ss2 = &ss;
    println!("ss2 is {}", ss2);
}

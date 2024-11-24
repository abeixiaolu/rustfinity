fn main() {
    // let s = String::from("Hello");
    // // borrow the value
    // print_str(&s);
    // println!("s is {}", s);
    // let mut str = String::from("Hello");
    // change_str(&mut str);
    // println!("str is mutated to {}", str);

    // let mut value = 5;
    // let ref1 = &mut value;
    // // let ref2 = &mut value; // cannot borrow `value` as mutable more than once at a time
    // println!("ref1 is {}", ref1);
    // println!("ref2 is {}", ref2);

    mutable_reference_limitations();
}

fn print_str(s: &String) {
    println!("print_str is {}", s);
}

// the value that is borrowed can not be changed.
// fn change_str(s: &String) {
//     s.push_str(" World");
// }

// the value that is borrowed can be changed.
fn change_str(s: &mut String) {
    s.push_str(" World");
}

fn mutable_reference_limitations() {
    let mut s = String::from("Hello");
    let r1 = &mut s;
    let r2 = &mut s;

    let result = concatenate(r1, r2);
    println!("result is {}", result);
}

fn concatenate(s1: &mut String, s2: &mut String) -> String {
    s1.to_string() + s2
}

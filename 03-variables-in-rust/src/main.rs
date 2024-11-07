// fn main() {
//     let x = 5;

//     // x = 10;
//     println!("The value of x is {}", x);
// }

// fn main() {
//     let mut x = 5;
//     println!("The value of x is {}", x);
//     x = 10;
//     println!("The value of new x is {}", x);
// }

// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is {}", x);
// }

// let x = 10;
// you must always annotate the type of the constant.
// const MAX_POINTS: u32 = 100_000;
// const PI: f32 = 3.14;
// const AUTHOR: &str = "John Doe";
// fn main() {
//     println!("The value of MAX_POINTS = {}", MAX_POINTS);
//     println!("The value of PI is: {}", PI);
//     println!("The author is: {}", AUTHOR);
// }

fn main() {
    let x = 5;
    {
        let y = 10;
        println!("The value of y is: {}", y);
    } // y goes out of scope here

    println!("The value of x is: {}", x);
    // println!("The value of y is: {}", y);
}
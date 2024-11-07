// fn main() {
//     // annotate the type explicitly
//     let z:i32 = "999".parse().unwrap();
//     println!("The value of z is: {}", z);
// }

// scaler types: i32, f64, char, bool
// compound types: tuple, array

// fn main() {
//     let sum = 5 + 10;
//     let difference = 95.5 - 4.3;
//     let product = 4 * 30;
//     let quotient = 56.7 / 32.2;
//     let remainder = 43 % 5;

//     println!("Sum: {}", sum);
//     println!("Difference: {}", difference);
//     println!("Product: {}", product);
//     println!("Quotient: {}", quotient);
//     println!("Remainder: {}", remainder);

//     let product2 = 4 * 30.4 as i32;
//     println!("Product2: {}", product2);

//     let quotient = 11/10; // The result will be 1, not 1.1
//     let quotient2 = 11.0/10.0;  // The result will be 1.1
//     let quotient3:f32 = 11 as f32 / 10 as f32; // The result will be 1.1
 
//     println!("The quotient is: {}", quotient);
//     println!("The quotient2 is: {}", quotient2);
//     println!("The quotient3 is: {}", quotient3);
// }


// fn main() {
//     let is_true = true;
//     let is_false: bool = false;
//     println!("The value of is_true is: {}", is_true);
//     println!("The value of is_false is: {}", is_false);
// }

// fn main() {
//     let c = 'a';
//     let heart_eyed_cat = '😻';
//     let chinese_yin_yang = '☯';
//     let chinese = '中';
//     let japanese = '日';
//     println!("The value of c is: {}", c);
//     println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);
//     println!("The value of chinese_yin_yang is: {}", chinese_yin_yang);
//     println!("The value of chinese is: {}", chinese);
//     println!("The value of japanese is: {}", japanese);
// }


// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
//     let seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
//     println!("The value of seasons is: {:?}", seasons);
//     println!("The value of arr is: {:?}", arr);
//     println!("The value of months is: {:?}", months);
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
 
//     println!("The sixth element of a is: {}", a[5]); // No compile error, but will cause a runtime error when running the program
// }


fn main() {
    let tup = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

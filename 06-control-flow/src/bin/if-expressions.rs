fn main() {
    if_basic();
    if_let();
}

fn if_basic() {
  let number = 5;

  if number < 5 {
    println!("The number is less than 5")
  } else if number == 5 {
    println!("The number is equal to 5")
  } else {
    println!("The number is greater than 5")
  }
}

// Use if in a let statement
fn if_let() {
  let condition = true;
  let number = if condition { 5 } else { 6 }; // It must a same type
  println!("The number is: {}", number);
}

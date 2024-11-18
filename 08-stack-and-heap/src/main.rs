fn main() {
    let num = Box::new(42);
    println!("The heap number is {}", num);
} // The scope ends, memory is automatically released.

fn main() {
    loop_basic();
    println!("--------------------------------");
    loop_return_value();
    println!("--------------------------------");
    skipping_an_iteration();
    println!("--------------------------------");
    loop_labels();
    println!("--------------------------------");
    while_loop();
    println!("--------------------------------");
    for_loop();
    println!("--------------------------------");
    for_range();
}

fn loop_basic() {
    let mut counter = 0;
    loop {
        println!("The counter is {}", counter + 1);
        counter += 1;
        if counter == 5 {
            break;
        }
    }
}

fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The loop_return_value result is {}", result);
}

fn skipping_an_iteration() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter % 2 == 0 {
            continue;
        }
        if counter > 9 {
            break;
        }
        println!("The skipping_an_iteration counter is {}", counter);
    }
}

fn loop_labels() {
    let mut outer_counter = 0;
    'outer: loop {
        let mut inner_counter = 0;
        'inner: loop {
            println!(
                "Outer loop: {}, Inner loop: {}",
                outer_counter, inner_counter
            );
            if inner_counter == 5 {
                break 'inner;
            }
            if outer_counter == 2 {
                break 'outer;
            }
            inner_counter += 1;
        }
        outer_counter += 1;
    }
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
}

fn for_loop() {
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers {
        println!("The number is: {}", number);
    }
}

fn for_range() {
    // for number in 1..5 { // 1, 2, 3, 4
    for number in 1..=5 {
        // 1, 2, 3, 4, 5
        println!("The number is: {}", number);
    }
}

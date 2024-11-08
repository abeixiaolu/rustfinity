fn main() {
    pattern_matching();
    guard_pattern();
    let state = State::Running;
    if_let(state);
}

fn pattern_matching() {
    let x = 5;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
}

fn guard_pattern() {
    let x = 30;

    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        n if n > 3 => println!("Greater than three"),
        _ => println!("Other"),
    }
}

enum State {
    Idle,
    Running,
    Waiting,
}

fn if_let(state: State) {
    if let State::Idle = state {
        println!("The system is idle");
    }
}

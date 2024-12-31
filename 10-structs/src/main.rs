struct User {
    id: i32,
    username: String,
    email: String,
}

fn main() {
    /* let user = User {
        id: 1,
        username: String::from("John"),
        email: String::from("john@example.com"),
    };

    println!("User ID: {}", user.id);
    println!("User Username: {}", user.username);
    println!("User Email: {}", user.email); */

    same_field();

    struct_update_syntax();

    debug_print();
}

fn same_field() {
    let username = String::from("John");
    let email = String::from("john@example.com");

    let user = User {
        id: 1,
        username,
        email,
    };

    println!("User ID: {}", user.id);
    println!("User Username: {}", user.username);
    println!("User Email: {}", user.email);

    // println!("email: {}", email);
}

fn struct_update_syntax() {
    let user = User {
        id: 1,
        username: String::from("John"),
        email: String::from("john@example.com"),
    };

    let user2 = User {
        username: String::from("Jane"),
        ..user
    };

    println!(
        "user2: {}, {}, {}",
        user2.username, user2.email, user.username
    );
}

#[derive(Debug)]
struct Dog {
    name: String,
    breed: String,
}

fn debug_print() {
    let dog = Dog {
        name: String::from("Buddy"),
        breed: String::from("Golden Retriever"),
    };

    println!("{:?}", dog);
}

// fn main() {
//     let my_string = String::from("Hello, World!");
//     let hello = &my_string[0..5];
//     let world = &my_string[7..12];
//     println!("hello is {}", hello);
//     println!("world is {}", world);
// }

// fn main() {
//     let mut text = String::from("good morning!");
//     let prefix = get_prefix(&text);

//     // text = format!("Hello {}", text);

//     println!("The prefix is {}", prefix);
//     println!("The text is {}", text);
// }

// fn get_prefix(t: &String) -> String {
//     let characters = t.chars();
//     for (idx, char) in characters.enumerate() {
//         if char == ' ' {
//             return t[..idx].to_string();
//         }
//     }
//     return t[..].to_string();
// }

// borrowed string
// fn get_prefix(t: &String) -> &str {
//     let characters = t.chars();
//     for (idx, char) in characters.enumerate() {
//         if char == ' ' {
//             return &t[..idx];
//         }
//     }
//     return &t[..];
// }

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..3];

    let complete_slice = &numbers[..];
    let from_start_slice = &numbers[..3];
    let to_end_slice = &numbers[2..];

    println!("The slice is {:?}", slice);
    println!("The complete slice is {:?}", complete_slice);
    println!("The from start slice is {:?}", from_start_slice);
    println!("The to end slice is {:?}", to_end_slice);
}

fn main() {
    tuple_struct();

    unit_struct();
}

struct Color(i32, i32, i32);
fn tuple_struct() {
    let c = Color(255, 0, 0);
    println!("The color is RGB({}, {}, {})", c.0, c.1, c.2);
}

struct UnitStruct;
trait Describable {
    fn describe(&self) -> String;
}
impl Describable for UnitStruct {
    fn describe(&self) -> String {
        "This is a unit struct implementing the Describable trait".to_string()
    }
}

fn unit_struct() {
    let unit = UnitStruct;
    println!("{}", unit.describe());
}

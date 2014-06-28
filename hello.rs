extern crate input;

fn main() {
    let name = input::get_str("name");
    let a    = input::get_int("a");
    let b    = input::get_int("b");
    println!("Hello, {}.", name);
    println!("{} + {} = {}", a, b, a+b);
}
use std::io;
use std::str;
fn output(name: ~str, a: int, b: int) {
    println!("Hello, {}.", name);
    println!("{} + {} = {}", a, b, a+b);
}

fn main() {
    print!("Enter your name: ");
    let name_raw = io::stdin().read_line().ok().unwrap();
    let name = str::replace(name_raw, "\n", "");

    print!("Enter a: ");
    let a_raw = io::stdin().read_line().ok().unwrap();
    let a_fixed = str::replace(a_raw, "\n", "");
    let a: int = from_str(a_fixed).unwrap();

    print!("Enter b: ");
    let b_raw = io::stdin().read_line().ok().unwrap();
    let b_fixed = str::replace(b_raw, "\n", "");
    let b: int = from_str(b_fixed).unwrap();

    output(name, a, b);
}
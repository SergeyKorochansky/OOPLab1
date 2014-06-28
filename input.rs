use std::io;
use std::str;

pub fn get_str(name: &str) -> ~str {
    print!("Enter {}: ", name);
    let name_raw = io::stdin().read_line().ok().unwrap();
    str::replace(name_raw, "\n", "")
}

pub fn get_int(name: &str) -> int {
	print!("Enter {}: ", name);
    let int_raw = io::stdin().read_line().ok().unwrap();
    let int_fixed = str::replace(int_raw, "\n", "");
    from_str(int_fixed).unwrap()
}
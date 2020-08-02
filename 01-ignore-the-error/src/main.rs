use std::fs;

fn main() {
    // Try changing the file name
    let content = fs::read_to_string("./Cargo.toml").unwrap();
    println!("{}", content)
}

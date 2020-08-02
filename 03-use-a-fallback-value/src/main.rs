use std::env;

fn main() {
    // Try running with a different env var - `PORT=4000 cargo run`
    let port = env::var("PORT").unwrap_or("3000".to_string());
    println!("{}", port);
}

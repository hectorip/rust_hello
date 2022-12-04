// import env
use std::env;

// Starting to test rust
fn main() {
    // reading command line arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("Me gusta Rust");
}
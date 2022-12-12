// import env
use std::env;

// Starting to test rust
fn main() {
    // reading command line arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?} - How this works?", args);
    println!("Me gusta Rust");
    if args.len() > 1 {
        println!("Hello, {}!", args[1]);
    } else {
        println!("Hello, World!");
    }
}
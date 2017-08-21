use std::io;

fn main() {
    let mut input = String::new();
    println!("Hello, world!");
    io::stdin().read_line(&mut input);
    println!("{}", input);
}
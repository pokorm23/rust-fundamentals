#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let name = it.next().unwrap().unwrap();
    let age: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();

    println!("Hi, {}! You are {} years old.", name, age);
}

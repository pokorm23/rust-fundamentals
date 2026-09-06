#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let s = it.next().unwrap().unwrap();
    //let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();

    println!("{}", count(&s));
}

fn count(s: &str) -> usize {
    s.len()
}
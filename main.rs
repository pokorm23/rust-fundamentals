#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();

    println!("{}", s(n));
}

fn s(n: i32) -> i64 {
    n as i64 * n as i64
}
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();

    let mut t: i64 = 0;
    for i in 1..=n {
        t += i as i64;
    }

    println!("{}", t);
}
 
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();

    let ns = n.to_string();
    let s = match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz",
        (0, _) => "Fizz",
        (_, 0) => "Buzz",
        _ => &ns
    };

    println!("{}", s);
}
 
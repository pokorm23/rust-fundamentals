#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let s = it.next().unwrap().unwrap();
    //let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<i64> = s.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let mut max = nums.get(0).unwrap();

    for n in &nums {
        if n > max {
            max = n;
        }
    }

    println!("{}", max);
}

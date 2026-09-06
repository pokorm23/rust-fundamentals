#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let s = it.next().unwrap().unwrap();
    //let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<i64> = s.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let max = nums.iter()
    .filter(|n| *n % 2 == 0)
    .map(|n| n * n)
    .sum::<i64>();

    println!("{}", max);
}

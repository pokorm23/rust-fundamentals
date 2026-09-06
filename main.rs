#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    //let s = it.next().unwrap().unwrap();
    //let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<_> = it.map(|s| s.unwrap().trim().parse::<i32>().unwrap())
        .collect();
    
    let a = Point { x: nums[0], y: nums[1] };
    let b = Point { x: nums[2], y: nums[3] };

    println!("{}", a.dist(&b));
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, other: &Point) -> i32 {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2)
    }
} 
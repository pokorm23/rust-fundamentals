#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};
use std::collections::{HashSet};

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let s = it.next().unwrap().unwrap();
    //let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();
    let mut seen: HashSet<&str> = HashSet::new();
    
    for n in s.split_whitespace() {
        seen.insert(n);
    }

    let d = seen.len();

    println!("{}", d);
}

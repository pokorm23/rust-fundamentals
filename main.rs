#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let s = it.next().unwrap().unwrap();
    //let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();
    //let nums: Vec<_> = it.map(|s| s.unwrap().trim().parse::<i32>().unwrap())
    //    .collect();

    match parse_int(&s) {
        Ok(n) => println!("ok: {}", n),
        Err(e) => println!("error: {}", e),
    }
}


fn parse_int(s: &str) -> Result<i32, String> {
    let n = s.trim().parse::<i32>();

    match n {
        Ok(nn) => Ok(nn),
        Err(e) => Err("not a number".to_string())
    }
}
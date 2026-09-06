#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::io::{BufRead};
use std::collections::HashMap;

use crate::Light::{Green, Red, Yellow};

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let s = it.next().unwrap().unwrap();
    //let n: i32 = it.next().unwrap().unwrap().trim().parse().unwrap();
    //let nums: Vec<_> = it.map(|s| s.unwrap().trim().parse::<i32>().unwrap())
    //    .collect();

    let c = match s.trim() {
        "red" => Red,
        "yellow" => Yellow,
        _ => Green
    };
    
    println!("{}", name(&next(c)));
}

enum Light {
    Red, Yellow, Green
}

fn next(l: Light) -> Light {
    match l {
        Light::Red => Light::Green,
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}
fn name(l: &Light) -> &str {
    match l {
        Light::Red => "red",
        Light::Yellow => "yellow",
        Light::Green => "green",
    }
}
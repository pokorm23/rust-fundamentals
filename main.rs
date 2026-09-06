use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let w: i64 = it.next().unwrap().parse().unwrap();
    let h: i64 = it.next().unwrap().parse().unwrap();
    println!("{}", w * h);
}

use std::io::{self, BufRead};
fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let crabs = line.split(',').map(|v| v.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
    let max = crabs.iter().cloned().max().unwrap();
    let psum_abs = |n: i64| -> i64 { (0i64 ..= n.abs()).sum() };
    let r = (0..max).map(|w| crabs.iter().map(|c| psum_abs(c - w)).sum::<i64>()).min();
    println!("{}", r.unwrap());
}

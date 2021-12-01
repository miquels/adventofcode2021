// cargo run < ../../input/input.txt

use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .map(|s| u32::from_str(&s.unwrap()).unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|slice| slice.iter().fold(0u32, |accum, x| accum + x))
        .enumerate()
        .fold((0u32, 0u32), |(prev, tot), (idx, cur)| (cur, (tot + (idx > 0 && cur > prev) as u32)));
    println!("{}", result.1);
}

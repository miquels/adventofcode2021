// cargo run < ../../input/input.txt

use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let result = io::stdin()
        .lock()
        .lines()
        .map(|s| u32::from_str(&s.unwrap()).unwrap())
        .fold((0u32, 0u32), |(prev, tot), cur| (cur, (tot + (prev > 0 && cur > prev) as u32)));
    println!("{}", result.1);
}

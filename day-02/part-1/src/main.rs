// cargo run < ../../input/input.txt

use std::io::{self, BufRead};

fn main() {
    let pos = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .fold((0u32, 0u32), |s, line| {
            let mut i = line.split_whitespace();
            let dir = i.next().unwrap();
            let val: u32 = i.next().unwrap().parse().unwrap();
            match dir {
                "forward" => (s.0 + val, s.1),
                "down" => (s.0, s.1 + val),
                "up" => (s.0, s.1 - val),
                _ => unreachable!(),
            }
        });
    println!("{}", pos.0 * pos.1);
}

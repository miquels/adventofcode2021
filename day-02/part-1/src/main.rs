// cargo run < ../../input/input.txt

use std::io::{self, BufRead};

fn main() {
    let pos = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .fold((0u32, 0u32), |mut a, line| {
            let mut i = line.split_whitespace();
            let dir = i.next().unwrap();
            let val: u32 = i.next().unwrap().parse().unwrap();
            match dir {
                "forward" => a.0 += val,
                "down" => a.1 += val,
                "up" => a.1 -= val,
                _ => unreachable!(),
            };
            a
        });
    println!("{}", pos.0 * pos.1);
}

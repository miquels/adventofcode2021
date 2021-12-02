// cargo run < ../../input/input.txt

use std::io::{self, BufRead};

#[derive(Default)]
struct SubMarine {
    hpos: i32,
    depth: i32,
    aim: i32,
}

fn main() {
    let pos = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .fold(SubMarine::default(), |mut s, line| {
            let mut i = line.split_whitespace();
            let dir = i.next().unwrap();
            let val: i32 = i.next().unwrap().parse().unwrap();
            match dir {
                "forward" => {
                    s.hpos += val;
                    s.depth += s.aim  * val;
                },
                "down" => s.aim += val,
                "up" => s.aim -= val,
                _ => unreachable!(),
            };
            s
        });
    println!("{}", pos.hpos * pos.depth);
}

// cargo run < ../../input/input.txt

use std::io::{self, BufRead};

fn main() {
    let (res, cnt) = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .fold((Vec::new(), 0u32), |(state, cnt), line| {
            let s = line
                .chars()
                .enumerate()
                .map(|(i, c)| if i < state.len() { state[i] } else { 0u32 } + (c == '1') as u32)
                .collect();
            (s, cnt + 1)
        });
    assert!(res.len() <= 32);
    let gamma = res
        .iter()
        .fold(0u64, |a, &v| a << 1 | (v >= cnt / 2) as u64);
    let epsilon = gamma ^ (2u64.pow(res.len() as u32) - 1);
    println!("gamma {}, epsilon {}, power {}", gamma, epsilon, gamma * epsilon);
}

use std::io::{self, BufRead};

fn main() {
    let x = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap()
                        .split_whitespace()
                        .skip(11)
                        .map(|d| d.len())
                        .filter(|&l| l == 2 || l == 4 || l == 3 || l == 7).count())
        .sum::<usize>();
    println!("{}", x);
}

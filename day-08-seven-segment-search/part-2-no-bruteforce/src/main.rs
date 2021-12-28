use std::io::{self, BufRead};

fn decode_digits(patterns: &[u8], digits: &[u8]) -> u16 {
    let mut map = [0u8; 10];
    let mut patterns = patterns.to_vec();
    patterns.sort_by(|a, b| a.count_ones().partial_cmp(&b.count_ones()).unwrap());
    for p in patterns.iter() {
        let n = match p.count_ones() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 if (*p & map[4]).count_ones() == 2 => 2,
            5 if (*p & map[7]).count_ones() == 3 => 3,
            5 => 5,
            6 if (*p & map[4]).count_ones() == 4 => 9,
            6 if (*p & map[7]).count_ones() == 3 => 0,
            6 => 6,
            7 => 8,
            _ => continue,
        };
        map[n as usize] = *p;
    }
    digits.iter().fold(0u16, |n, &d| n * 10 + map.iter().position(|&v| v == d).unwrap() as u16)
}

fn main() {
    let result = io::stdin().lock().lines().flatten().map(|line| {
        let b = line
            .split_whitespace()
            .filter(|&s| s != "|")
            .map(|s| s.chars().fold(0u8, |a, x| a + 2u8.pow((x as u8 - b'a') as u32) as u8))
            .collect::<Vec<_>>();
        decode_digits(&b[0..10], &b[10..14]) as u64
    }).sum::<u64>();
    println!("{}", result);
}

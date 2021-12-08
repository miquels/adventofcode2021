use std::io::{self, BufRead};

fn decode_digits(patterns: &[u8], digits: &[u8]) -> u16 {
    let mut map = [0u8; 10];
    let mut rmap = std::collections::HashMap::<u8, u8>::new();
    for p in patterns.iter() {
        let n = match p.count_ones() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            _ => continue,
        };
        rmap.insert(*p, n);
        map[n as usize] = *p;
    }
    for p in patterns.iter() {
        let n = match p.count_ones() {
            5 if (*p & map[4]).count_ones() == 2 => 2,
            5 if (*p & map[7]).count_ones() == 3 => 3,
            5 => 5,
            6 if (*p & map[4]).count_ones() == 4 => 9,
            6 if (*p & map[7]).count_ones() == 3 => 0,
            6 => 6,
            _ => continue,
        };
        rmap.insert(*p, n);
    }
    digits.iter().fold(0u16, |n, d| n * 10 + rmap[d] as u16)
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

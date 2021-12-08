use std::io::{self, BufRead};
use itertools::Itertools;

fn map_digit(digit: &[u8], mapping: &[u8]) -> i64 {
    let mut buf = [0u8; 7];
    let len = digit.len();
    for i in 0 .. len {
        buf[i] = b'a' + mapping[(digit[i] - b'a') as usize];
    }
    buf[..len].sort();
    match &buf[..len] {
        b"abcefg" => 0,
        b"cf" => 1, 
        b"acdeg" => 2, 
        b"acdfg" => 3,
        b"bcdf" => 4, 
        b"abdfg" => 5,
        b"abdefg" => 6,
        b"acf" => 7,
        b"abcdefg" => 8,
        b"abcdfg" => 9,
        _ => -1,
    }
}

fn decode_digits(samples: &[&[u8]], digits: &[&[u8]], mappings: &[Vec<u8>]) -> u64 {
    for mapping in mappings {
        if !samples.iter().all(|d| map_digit(&d, mapping) >= 0) {
            continue;
        }
        return digits.iter().fold(0, |n, d| n * 10 + map_digit(d, mapping) as u64);
    }
    unreachable!()
}

fn main() {
    let mappings = (0u8 .. 7).permutations(7).collect::<Vec<_>>();

    let result = io::stdin().lock().lines().map(|line| {
        let line = line.unwrap();
        let w = line.split_whitespace().map(|s| s.as_bytes()).collect_vec();
        decode_digits(&w[0..10], &w[11..15], &mappings) as u64
    }).sum::<u64>();
    println!("{}", result);
}

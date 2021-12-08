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

fn decode_digits(samples: &[Vec<u8>], digits: &[Vec<u8>], mappings: &[Vec<u8>]) -> u64 {

'LOOP:
    for mapping in mappings {
        for digit in samples {
            if map_digit(&digit, mapping) < 0 {
                continue 'LOOP;
            }
        }
        let mut n = 0;
        for digit in digits.iter() {
            n = n * 10 + map_digit(digit, mapping) as u64;
        }
        return n;
    }
    unreachable!()
}

fn main() {
    let mappings = (0u8 .. 7).permutations(7).collect::<Vec<_>>();

    let result = io::stdin().lock().lines().map(|line| {
        let line = line.unwrap();
        let w = line
            .split_whitespace()
            .map(|s| {
                let mut v = s.as_bytes().to_vec();
                v.sort();
                v
            })
            .collect_vec();
        decode_digits(&w[0..10], &w[11..15], &mappings) as u64
    }).sum::<u64>();
    println!("{}", result);
}

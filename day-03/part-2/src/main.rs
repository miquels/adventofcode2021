// cargo run < ../../input/input.txt

use std::io::{self, BufRead};

fn calc<'a>(bitset: &Vec<&'a Vec<bool>>, oxy: bool) -> &'a Vec<bool> {
    let mut bitset = bitset.clone();
    for pos in 0 .. bitset[0].len() {
        let set = bitset.iter().filter(|b| b[pos]).count() as u32;
        let b = 2 * set < bitset.len() as u32;
        bitset.retain(|bitset| bitset[pos] == oxy ^ b);
        if bitset.len() == 1 {
            return bitset[0];
        }
    }
    unreachable!();
}

fn main() {
    let bitsets = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c == '1').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let bitsets = bitsets.iter().collect::<Vec<_>>();

    let to_u32 = |v: &Vec<bool>| v.iter().fold(0u32, |accum, &x| (accum << 1) | (x as u32));
    let oxy = to_u32(calc(&bitsets, true));
    let co2 = to_u32(calc(&bitsets, false));
    println!("oxygen: {}, co2: {}, rating {}", oxy, co2, oxy * co2);
}

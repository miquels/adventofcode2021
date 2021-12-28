// cargo run < ../../input/input.txt

use std::io::{self, Read};

fn main() {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();
    let mut boards = data
        .split("\n\n")
        .map(|d| d
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let numbers = boards.remove(0);

    fn check(v: &[u8]) -> bool {
        let check_one = |v: &[u8], n, i| (0u8..5).filter(|x| v[(x*i+n) as usize] == 0).count() == 5;
        (0..5).find(|&n| check_one(v, n, 5) || check_one(v, n*5, 1)).is_some()
    }

    for n in numbers.iter().cloned() {
        let mut idx = 0;
        while idx < boards.len() {
            if let Some(i) = boards[idx].iter().position(|&x| x == n) { boards[idx][i] = 0; }
            if check(&boards[idx]) {
                if boards.len() > 1 {
                    boards.remove(idx);
                    continue;
                }
                println!("{}", boards[idx].iter().fold(0u32, |a, &x| a + x as u32) * n as u32);
                return;
            }
            idx += 1;
        }
    }
    println!("NO BINGO");
}

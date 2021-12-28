use std::io::{self, BufRead};

fn main() {
    let m = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let max_x = m[0].len() - 1;
    let max_y = m.len() - 1;
    let n = (0 ..= max_y)
        .map(|y| (0 ..= max_x)
            .filter_map(|x| {
                let n = m[y][x];
                ((y == 0 || m[y-1][x] > n) &&
                 (x == 0 || m[y][x-1] > n) &&
                 (x == max_x || m[y][x+1] > n) &&
                 (y == max_y || m[y+1][x] > n)).then(|| n as u64 + 1)
            })
            .sum::<u64>())
        .sum::<u64>();
    println!("{}", n);
}

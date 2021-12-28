use std::io::{self, BufRead};

fn around(x: usize, y: usize, max_x: usize, max_y: usize) -> impl Iterator<Item=(usize, usize)> {
    [(0, -1), (-1, 0), (1, 0), (0, 1)]
        .iter()
        .map(move |(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(move |&(x, y)| x >= 0 && x <= max_x as i32 && y >= 0 && y <= max_y as i32)
        .map(|(x, y)| (x as usize, y as usize))
}

fn climb(f: &mut Vec<Vec<u8>>, x: usize, y: usize, max_x: usize, max_y: usize) -> u32 {
    let t = around(x, y, max_x, max_y)
        .filter_map(|(xa, ya)| {
            (f[ya][xa] > f[y][x] && f[ya][xa] != 9)
                .then(|| climb(f, xa, ya, max_x, max_y))
        })
        .sum::<u32>();
    f[y][x] = 9;
    t + 1
}

fn main() {
    let mut f = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_x = f[0].len() - 1;
    let max_y = f.len() - 1;
    let mut nums = (0 ..= max_y)
        .map(|y| (0 ..= max_x).map(move |x| (x, y)))
        .flatten()
        .filter_map(|(x, y)| {
            around(x, y, max_x, max_y)
                .all(|(xa, ya)| f[ya][xa] > f[y][x])
                .then(|| climb(&mut f, x, y, max_x, max_y))
        })
        .collect::<Vec<_>>();
    nums.sort_by(|a, b| b.cmp(a));

    println!("{}", nums.into_iter().take(3).product::<u32>());
}

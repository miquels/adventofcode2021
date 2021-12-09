use std::io::{self, BufRead};

fn climb(f: &mut Vec<Vec<u8>>, x: i32, y: i32, max_x: i32, max_y: i32) -> u32 {
    let n = f[y as usize][x as usize];
    let mut t = 1;
    for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
        if x + dx >= 0 && x + dx <= max_x && y + dy >= 0 && y + dy <= max_y {
            let m = f[(y + dy) as usize][(x + dx) as usize];
            if m > n && m != 9 {
                t += climb(f, x + dx, y + dy, max_x, max_y);
            }
        }
    }
    f[y as usize][x as usize] = 9;
    t
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
        .map(|y|
            (0 ..= max_x).filter_map(|x| {
                let n = f[y][x];
                let b = (y == 0 || f[y-1][x] > n) && (y == max_y || f[y+1][x] > n) &&
                        (x == 0 || f[y][x-1] > n) && (x == max_x || f[y][x+1] > n);
                b.then(|| climb(&mut f, x as i32, y as i32, max_x as i32, max_y as i32))
            })
            .collect::<Vec<_>>().into_iter()
        )
        .flatten()
        .collect::<Vec<_>>();
    nums.sort_by(|a, b| b.cmp(a));
    println!("{}", nums.into_iter().take(3).product::<u32>());
}

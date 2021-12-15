use std::time::Instant;
use std::io::{self, BufRead};
use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: u16,
    y: u16,
}

type Map = Vec<Vec<u16>>;

fn map_from_stdin() -> Map {
    io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.bytes().map(|b| (b - b'0') as u16).collect::<Vec<_>>())
        .collect()
}

fn larger_map(map: &Map) -> Map {
    let len_x = map[0].len();
    let len_y = map.len();
    let mut m = map.clone();
    for y in 0..len_y*5 {
        if m.len() <= y {
            m.push(Vec::new());
        }
        m[y].resize(len_x * 5, 0);
    }

    for y in 0..5 {
        for x in y..5 {
            if x == 0 && y == 0 {
                continue;
            }
            for y1 in 0..len_y {
                for x1 in 0..len_x {
                    let n = (m[y*len_y + y1][(x-1)*len_x + x1] % 9) + 1;
                    m[y*len_y + y1][x*len_x + x1] = n;
                    m[x*len_x + y1][y*len_y + x1] = n;
                }
            }
        }
    }
    m
}

#[inline]
fn around(map: &Map, p: Pos, max_x: usize, max_y: usize) -> impl Iterator<Item=(Pos, u16)> +'_ {
    [(0, -1), (-1, 0), (1, 0), (0, 1)]
        .iter()
        .map(move |(dx, dy)| (p.x as i32 + dx, p.y as i32 + dy))
        .filter(move |&(x, y)| x >= 0 && x <= max_x as i32 && y >= 0 && y <= max_y as i32)
        .map(|(x, y)| (Pos { x: x as u16, y: y as u16 }, map[y as usize][x as usize]))
}

struct Visit {
    pos: Pos,
    risk: u32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.risk.eq(&other.risk)
    }
}

impl Eq for Visit {}

fn my_dijkstra(map: &Map) -> u16 {
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let mut risks = (0 .. map.len()).map(|_| {
        let mut row = Vec::new();
        row.resize(map[0].len(), i32::MAX as u32);
        row
    }).collect::<Vec<_>>();
    risks[0][0] = 0;

    let mut to_visit = std::collections::BinaryHeap::new();
    to_visit.push(Visit{ pos: Pos{ x: 0, y: 0 }, risk: 0 });

    while let Some(Visit{ pos, risk}) = to_visit.pop() {
        if (risks[pos.y as usize][pos.x as usize] & 0x80000000) > 0 {
            continue;
        }
        risks[pos.y as usize][pos.x as usize] |= 0x80000000;

        for (npos, height) in around(map, pos, max_x, max_y) {
            let n = risk + height as u32;
            if n < (risks[npos.y as usize][npos.x as usize] & 0x7fffffff) {
                risks[npos.y as usize][npos.x as usize] &= 0x80000000;
                risks[npos.y as usize][npos.x as usize] |= n;
                to_visit.push(Visit{ pos: npos, risk: n });
            }
        }
    }
    (risks[max_y][max_x] & 0x7fffffff) as u16
}

fn shortest_path_pathfinding(map: &Map) -> u16 {
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let entrance = Pos { x: 0, y: 0 };
    let exit = Pos { x: max_x as u16, y: max_y as u16 };

    dijkstra(&entrance, |p| around(map, *p, max_x, max_y), |p| *p == exit).unwrap().1
}

fn main() {
    let map = map_from_stdin();
    let map5 = larger_map(&map);

    let now = Instant::now();
    let res = shortest_path_pathfinding(&map);
    let elapsed = now.elapsed();
    println!("part1: {} (crate 'pathfinding', {:?})", res, elapsed);

    let now = Instant::now();
    let res = my_dijkstra(&map);
    let elapsed = now.elapsed();
    println!("part1 {} (my_dijkstra, {:?})", res, elapsed);

    let now = Instant::now();
    let res = shortest_path_pathfinding(&map5);
    let elapsed = now.elapsed();
    println!("part2: {} (crate 'pathfinding', {:?}))", res, elapsed);

    let now = Instant::now();
    let p2 = my_dijkstra(&map5);
    let elapsed = now.elapsed();
    println!("part2: {} (my_dijkstra, {:?})", p2, elapsed);
}

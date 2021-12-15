use std::io::{self, BufRead};
use std::time::Instant;

type Map = Vec<Vec<u16>>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

struct PositionRisk {
    pos: Pos,
    risk: u32,
}

impl Ord for PositionRisk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for PositionRisk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PositionRisk {
    fn eq(&self, other: &Self) -> bool {
        self.risk.eq(&other.risk)
    }
}

impl Eq for PositionRisk {}

#[inline]
fn neighbors(map: &Map, p: Pos, max_x: usize, max_y: usize) -> impl Iterator<Item = (Pos, u16)> + '_ {
    let Pos { x, y } = p;
    [
        (y > 0).then(|| (x, y - 1)),
        (x > 0).then(|| (x - 1, y)),
        (x < max_x).then(|| (x + 1, y)),
        (y < max_y).then(|| (x, y + 1)),
    ]
    .into_iter()
    .flatten()
    .map(|(x, y)| (Pos { x, y }, map[y][x]))
}

fn dijkstra(map: &Map) -> u16 {
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let mut risks = (0..map.len())
        .map(|_| {
            let mut row = Vec::new();
            row.resize(map[0].len(), i32::MAX as u32);
            row
        })
        .collect::<Vec<_>>();
    risks[0][0] = 0;

    let mut to_visit = std::collections::BinaryHeap::new();
    to_visit.push(PositionRisk {
        pos: Pos { x: 0, y: 0 },
        risk: 0,
    });

    while let Some(PositionRisk { pos, risk }) = to_visit.pop() {
        if (risks[pos.y][pos.x] & 0x80000000) > 0 {
            continue;
        }
        risks[pos.y][pos.x] |= 0x80000000;

        for (npos, height) in neighbors(map, pos, max_x, max_y) {
            let n = risk + height as u32;
            if n < (risks[npos.y][npos.x] & 0x7fffffff) {
                risks[npos.y][npos.x] &= 0x80000000;
                risks[npos.y][npos.x] |= n;
                to_visit.push(PositionRisk { pos: npos, risk: n });
            }
        }
    }
    (risks[max_y][max_x] & 0x7fffffff) as u16
}

fn multiply_map(map: &Map, factor: usize) -> Map {
    let (len_x, len_y) = (map[0].len(), map.len());
    let mut m = map.clone();
    m.resize(len_y * factor, Vec::new());
    m.iter_mut().for_each(|x| x.resize(len_x * factor, 0));

    for y in 0..factor * len_y {
        for x in 0..factor * len_x {
            m[y][x] = (map[y % len_y][x % len_x] - 1 + (y / len_y + x / len_x) as u16) % 9 + 1;
        }
    }
    m
}

fn map_from_stdin() -> Map {
    io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.bytes().map(|b| (b - b'0') as u16).collect::<Vec<_>>())
        .collect()
}

fn main() {
    let map = map_from_stdin();
    let map5 = multiply_map(&map, 5);

    let now = Instant::now();
    let res = dijkstra(&map);
    let elapsed = now.elapsed();
    println!("part1: {} ({:?})", res, elapsed);

    let now = Instant::now();
    let res = dijkstra(&map5);
    let elapsed = now.elapsed();
    println!("part2: {} ({:?})", res, elapsed);
}

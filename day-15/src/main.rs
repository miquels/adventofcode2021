use std::io::{self, BufRead};
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: u16,
    y: u16,
}

type Map = Vec<Vec<u32>>;

fn map_from_stdin() -> Map {
    io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.bytes().map(|b| (b - b'0') as u32).collect::<Vec<_>>())
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

fn shortest_path(map: &Map) -> u32 {
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let entrance = Pos { x: 0, y: 0 };
    let exit = Pos { x: max_x as u16, y: max_y as u16 };

    let neighbors = |p: &Pos| {
        [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .iter()
            .map(move |(dx, dy)| (p.x as i32 + dx, p.y as i32 + dy))
            .filter(move |&(x, y)| x >= 0 && x <= max_x as i32 && y >= 0 && y <= max_y as i32)
            .map(|(x, y)| (Pos { x: x as u16, y: y as u16 }, map[y as usize][x as usize]))
            .collect::<Vec<_>>()
    };
    dijkstra(&entrance, neighbors, |p| *p == exit).unwrap().1
}

fn main() {
    let map = map_from_stdin();
    println!("part1: {}", shortest_path(&map));

    let map5 = larger_map(&map);
    println!("part2: {}", shortest_path(&map5));
}

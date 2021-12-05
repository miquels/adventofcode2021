// cargo run < ../input/input.txt

use std::io::{self, BufRead};

struct Floor([[i16; 1000]; 1000]);

impl Floor {
    fn new() -> Floor {
        Floor([[0i16; 1000]; 1000])
    }

    fn clear(&mut self) {
        self.0.iter_mut().map(|y| y.iter_mut()).flatten().for_each(|e| *e = 0);
    }

    fn draw(&mut self, line: &Line) {
        let dir = |i, j| if i > j { -1 } else { std::cmp::min(1, j - i) };
        let xd = dir(line.x1, line.x2);
        let yd = dir(line.y1, line.y2);
        let (mut x, mut y) = (line.x1, line.y1);
        loop {
            self.0[y as usize][x as usize] += 1;
            if x == line.x2 && y == line.y2 {
                break;
            }
            x += xd;
            y += yd;
        }
    }

    fn ndanger(&self) -> usize {
        self.0.iter().map(|y| y.iter()).flatten().filter(|&&n| n >= 2).count()
    }
}

struct Line {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16,
}

impl Line {
    fn parse(line: &str) -> Line {
        let c = line
            .split(" -> ")
            .map(|n| n.split(','))
            .flatten()
            .map(|n| n.parse::<i16>().unwrap())
            .collect::<Vec<i16>>();
        Line{ x1: c[0], y1: c[1], x2: c[2], y2: c[3] }
    }
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|s| Line::parse(&s.unwrap()))
        .collect::<Vec<_>>();

    let mut floor = Floor::new();
    lines.iter().filter(|l| l.x1 == l.x2 || l.y1 == l.y2).for_each(|l| floor.draw(&l));
    println!("part 1: {}", floor.ndanger());

    floor.clear();
    lines.iter().for_each(|l| floor.draw(&l));
    println!("part 2: {}", floor.ndanger());
}

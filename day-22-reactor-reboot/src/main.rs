use std::cmp;
use scan_fmt::*;

#[derive(Default, Clone)]
struct Cuboid {
    x1:  i64,
    x2:  i64,
    y1:  i64,
    y2:  i64,
    z1:  i64,
    z2:  i64,
    on: bool,
    removed: Vec<Cuboid>,
}

impl Cuboid {
    fn from_stdin() -> Vec<Cuboid> {
        let mut c = Vec::new();
        while let Ok((on, x1, x2, y1, y2, z1, z2)) = scanln_fmt!("{} x={}..{},y={}..{},z={}..{}", String, i64, i64, i64, i64, i64, i64) {
            c.push(Cuboid {
                x1: cmp::min(x1, x2),
                x2: cmp::max(x1, x2),
                y1: cmp::min(y1, y2),
                y2: cmp::max(y1, y2),
                z1: cmp::min(z1, z2),
                z2: cmp::max(z1, z2),
                on: on == "on",
                ..Cuboid::default()
            });
        }
        c
    }

    fn overlap(&self, other: &Cuboid) -> Option<Cuboid> {
        if self.x2 < other.x1 || self.y2 < other.y1 || self.z2 < other.z1 ||
            self.x1 > other.x2 || self.y1 > other.y2 || self.z1 > other.z2 {
                return None;
        }
        Some(Cuboid {
            x1: cmp::max(self.x1, other.x1),
            x2: cmp::min(self.x2, other.x2),
            y1: cmp::max(self.y1, other.y1),
            y2: cmp::min(self.y2, other.y2),
            z1: cmp::max(self.z1, other.z1),
            z2: cmp::min(self.z2, other.z2),
            on: self.on,
            ..Cuboid::default()
        })
    }

    fn remove(&mut self, other: &Cuboid) {
        if let Some(other) = self.overlap(other) {
            for r in &mut self.removed {
                r.remove(&other);
            }
            self.removed.push(other);
        }
    }

    fn size(&self) -> i64 {
        let sz = (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1);
        sz - self.removed.iter().map(|r| r.size()).sum::<i64>()
    }
}

fn part2(cuboids: &[Cuboid]) {
    let mut nc = Vec::<Cuboid>::new();
    for c in cuboids {
        for x in &mut nc {
            x.remove(c);
        }
        if c.on {
            nc.push(c.clone());
        }
    }
    println!("part2: cubes ON: {}", nc.iter().map(|c| c.size()).sum::<i64>());
}

fn part1(cuboids: &[Cuboid]) {
    let mut cube = [[[false; 101]; 101]; 101];
    let init = Cuboid { x1: -50, x2: 50, y1: -50, y2: 50, z1: -50, z2: 50, ..Cuboid::default() };
    for c1 in cuboids {
        if let Some(c) = c1.overlap(&init) {
            for x in c.x1 ..= c.x2 {
                for y in c.y1 ..= c.y2 {
                    let z = &mut cube[(x+50) as usize][(y+50) as usize];
                    z[(c.z1 + 50) as usize ..= (c.z2 + 50) as usize].fill(c1.on);
                }
            }
        }
    }
    let num_on = cube
        .iter()
        .map(|x| x.iter().map(|y| y.iter().filter(|&&z| z).count()).sum::<usize>())
        .sum::<usize>();
    println!("part1: cubes in initialization area ON: {}", num_on);
}

fn main() {
    let cuboids = Cuboid::from_stdin();
    part1(&cuboids);
    part2(&cuboids);
}


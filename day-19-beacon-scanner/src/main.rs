use scan_fmt::scan_fmt;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead};
use std::ops::DerefMut;

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn rotate(&self, n: usize) -> Pos {
        let (x, y, z) = (self.x, self.y, self.z);
        let (x, y, z) = match n {
            0 => (x, y, z),
            1 => (-y, x, z),
            2 => (-x, -y, z),
            3 => (y, -x, z),
            4 => (x, z, -y),
            5 => (-z, x, -y),
            6 => (-x, -z, -y),
            7 => (z, -x, -y),
            8 => (y, x, -z),
            9 => (-x, y, -z),
            10 => (-y, -x, -z),
            11 => (x, -y, -z),
            12 => (z, x, y),
            13 => (-x, z, y),
            14 => (-z, -x, y),
            15 => (x, -z, y),
            16 => (z, y, -x),
            17 => (-y, z, -x),
            18 => (-z, -y, -x),
            19 => (y, -z, -x),
            20 => (y, z, x),
            21 => (-z, y, x),
            22 => (-y, -z, x),
            23 => (z, -y, x),
            _ => unreachable!(),
        };
        Pos { x, y, z }
    }

    fn sub(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn add(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn manhattan(&self, other: &Pos) -> u32 {
        let d = (other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs();
        d as u32
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Pos>,
    id: usize,
    position: Pos,
}

impl Scanner {
    fn new(id: usize) -> Scanner {
        Scanner {
            id,
            beacons: Vec::new(),
            position: Pos::default(),
        }
    }

    fn normalize(&self, other: &mut Scanner) -> Vec<usize> {
        let mut m = HashMap::<Pos, (usize, u32)>::new();

        for b1 in 0..other.beacons.len() {
            for r1 in 0..24 {
                let rbeacon = other.beacons[b1].rotate(r1);
                for b in 0..self.beacons.len() {
                    let distance = self.beacons[b].sub(&rbeacon);
                    m.entry(distance).and_modify(|e| e.1 += 1).or_insert((r1, 1u32));
                }
            }
        }

        let mut neighbors = Vec::new();
        for (pos, (rotation, count)) in &m {
            if *count >= 12 {
                // Found a match. Make this rotation permanent.
                for beacon in &mut other.beacons {
                    *beacon = beacon.rotate(*rotation);
                }
                // And remember position.
                other.position = self.position.add(pos);
                neighbors.push(other.id);
            }
        }

        neighbors
    }
}

struct Scanners {
    scanners: Vec<RefCell<Scanner>>,
}

impl Scanners {
    fn from_stdin() -> Scanners {
        let mut idx = 0;
        let mut scanners = Vec::new();
        let mut scanner = Scanner::new(0);

        for line in io::stdin().lock().lines().flatten() {
            if line.starts_with("---") {
                if idx > 0 {
                    scanners.push(RefCell::new(scanner));
                    scanner = Scanner::new(idx);
                }
                idx += 1;
                continue;
            }
            if let Ok((x, y, z)) = scan_fmt!(&line, "{},{},{}", i32, i32, i32) {
                scanner.beacons.push(Pos { x, y, z });
            }
        }
        scanners.push(RefCell::new(scanner));

        Scanners { scanners }
    }

    fn normalize1(&self, start_id: usize, done: &mut HashSet<usize>) {
        done.insert(start_id);

        for scanner_id in 0..self.scanners.len() {
            if done.contains(&scanner_id) {
                continue;
            }

            let mut scanner = self.scanners[scanner_id].borrow_mut();
            let start_scanner = self.scanners[start_id].borrow();
            let next_ids = start_scanner.normalize(scanner.deref_mut());
            drop(start_scanner);
            drop(scanner);

            for &next_id in &next_ids {
                if !done.contains(&next_id) {
                    self.normalize1(next_id, done);
                }
            }
        }
    }

    fn normalize(&self) {
        let mut done = HashSet::new();
        self.normalize1(0, &mut done);
    }

    #[allow(dead_code)]
    fn print(&self) {
        for s in &self.scanners {
            let scanner = s.borrow();
            println!("scanner {} position {}", scanner.id, scanner.position);
        }
    }

    fn count_beacons(&self) -> usize {
        let mut beacons = HashSet::new();
        for s in &self.scanners {
            let scanner = s.borrow();
            for beacon in &scanner.beacons {
                beacons.insert(beacon.add(&scanner.position));
            }
        }
        beacons.len()
    }

    fn max_manhattan(&self) -> u32 {
        let mut max = 0;
        for s1 in 0..self.scanners.len() {
            for s2 in s1 + 1..self.scanners.len() {
                let p1 = self.scanners[s1].borrow().position;
                let p2 = self.scanners[s2].borrow().position;
                let m = p1.manhattan(&p2);
                if m > max {
                    max = m;
                }
            }
        }
        max
    }
}

fn main() {
    let scanners = Scanners::from_stdin();
    scanners.normalize();
    println!("number of beacons: {}", scanners.count_beacons());
    println!("max manhattan distance between scanners: {}", scanners.max_manhattan());
}

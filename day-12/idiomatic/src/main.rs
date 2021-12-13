use std::collections::{HashMap, HashSet};
use scan_fmt::*;

struct Cave {
    small:  bool,
    next:   Vec<String>,
}

impl Cave {
    fn new(name: &str) -> Cave {
        Cave {
            small: name.chars().next().map(|c| c.is_lowercase()).unwrap(),
            next: Vec::new(),
        }
    }
}

#[derive(Default)]
struct Map {
    caves:  HashMap::<String, Cave>,
}

impl Map {
    fn read_from_stdin() -> Map {
        let mut map = Map::default();
        while let Ok((c1, c2)) = scanln_fmt!("{}-{}", String, String) {
            if c2 != "start" {
                map.caves.entry(c1.clone()).or_insert(Cave::new(&c1)).next.push(c2.clone());
            }
            if c1 != "start" {
                map.caves.entry(c2.clone()).or_insert(Cave::new(&c2)).next.push(c1.clone());
            }
        }
        map
    }

    fn explore(&self, name: &str, twice_ok: bool, mut visited: HashSet<String>) -> u32 {
        if name == "end" {
            return 1;
        }
        let cave = &self.caves[name];
        if cave.small {
            visited.insert(name.to_string());
        }
        let mut paths = 0;
        for next in &cave.next {
            let beenthere = visited.contains(next);
            if !beenthere || twice_ok {
                paths += self.explore(next, twice_ok && !beenthere, visited.clone());
            }
        }
        paths
    }
}

fn main() {
    let map = Map::read_from_stdin();

    let p1 = map.explore("start", false, HashSet::new());
    println!("part1 # of paths: {}", p1);

    let p2 = map.explore("start", true, HashSet::new());
    println!("part2 # of paths: {}", p2);
}

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

#[derive(Clone, Default)]
struct Path {
    once:   HashSet<String>,
    twice:  bool,
    ends:   bool,
}

struct Paths(Vec<Path>);

impl Paths {
    fn new(twice: bool) -> Paths {
        Paths(vec![ Path { twice, ..Path::default() }])
    }

    fn prune(&mut self) {
        self.0.retain(|p| p.ends);
    }

    fn branch(&mut self, cur_path: usize, twice: bool) -> usize {
        let mut branch = self.0[cur_path].clone();
        branch.twice = twice;
        self.0.push(branch);
        self.0.len() - 1
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
        map.caves.values_mut().for_each(|c| c.next.sort());
        map
    }

    fn explore(&self, paths: &mut Paths, cur_path: usize, name: &str) {
        if name == "end" {
            paths.0[cur_path].ends = true;
            return;
        }
        let cave = &self.caves[name];
        if cave.small {
            paths.0[cur_path].once.insert(name.to_string());
        }
        for next in &cave.next {
            let mut twice = paths.0[cur_path].twice;
            if paths.0[cur_path].once.contains(next) {
                if !twice {
                    continue;
                }
                twice = false;
            }
            let branch = paths.branch(cur_path, twice);
            self.explore(paths, branch, next);
        }
    }
}

fn main() {
    let map = Map::read_from_stdin();

    let mut paths = Paths::new(false);
    map.explore(&mut paths, 0, "start");
    paths.prune();
    println!("part1 # of paths: {}", paths.0.len());

    let mut paths = Paths::new(true);
    map.explore(&mut paths, 0, "start");
    paths.prune();
    println!("part2 # of paths: {}", paths.0.len());
}

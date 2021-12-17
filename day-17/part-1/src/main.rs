use std::collections::{HashMap, HashSet};
use scan_fmt::*;

#[derive(Default, Debug)]
struct Horizontal {
    min:    i32,
    max:    i32,
    steps:  HashSet<i32>,
    max_steps: i32,
}

impl Horizontal {
    fn new(min: i32, max: i32) -> Horizontal {
        Horizontal { min, max, ..Horizontal::default() }
    }


    // xspeed   step   pos
    // 10      0       0
    // 10      1      10
    // 9       2      19
    // 8       3      27
    // 7       4      34
    // 6       5      40
    // 5       6      45
    // 4       7      49
    // 3       8      52
    // 2       9      54
    // 1      10      55
    // 0      11      55
    //
    // xpos = (($xspeed + ($xspeed + 1 - $step)) * $step) / 2
    // xpos = ((2 * $xspeed + 1 - $step) * $step) / 2

    // Find all possible steps that end up between (min_x ..= max_x).
    // There is a maximum number of steps after which we always
    // end up there.
    fn calc(&mut self) {
        let mut maxset = HashSet::new();
        for speed in 1 ..=  self.max {
            for step in 1 ..= speed {
                let pos = ((2 * speed + 1 - step) * step) / 2;
                if pos < self.min {
                    continue;
                }
                if pos > self.max {
                    break;
                }
                if step == speed {
                    maxset.insert(step);
                } else {
                    self.steps.insert(step);
                }
            }
        }
        self.max_steps = maxset.drain().min().unwrap_or(i32::MAX);
    }
}

#[derive(Default, Debug)]
struct Vertical {
    min:      i32,
    max:      i32,
    steps:   HashMap<i32, i32>,
    max_speed: i32,
}

impl Vertical {
    fn new(min: i32, max: i32) -> Vertical {
        Vertical { min, max, max_speed: -min, ..Vertical::default() }
    }
    
    // Find all possible steps that go through (min_y ..= max_y).
    fn calc(&mut self) {
        for speed in 1 ..= self.max_speed {
            let mut max = 0;
            for step in 1 .. i32::MAX {
                let pos = ((2 * speed + 1 - step) * step) / 2;
                max = std::cmp::max(pos, max);
                if pos > self.max {
                    continue;
                }
                if pos < self.min {
                    break;
                }
                let e = self.steps.entry(step).or_insert(max);
                if max > *e {
                    *e = max;
                }
            }
        }
    }
}

fn main() {
    let (min_x, max_x, min_y, max_y) = scanln_fmt!("target area: x={}..{}, y={}..{}",
        i32, i32, i32, i32).unwrap();

    // find possible steps on the x axis
    let mut h  = Horizontal::new(min_x, max_x);
    h.calc();

    // find possible steps on the y axis
    let mut v  = Vertical::new(min_y, max_y);
    v.calc();

    // find overlap with max. height reached.
    let r = v.steps
        .iter()
        .filter(|(&step, _)| h.steps.contains(&step) || step >= h.max_steps)
        .max_by(|(_, h1),(_, h2)| h1.cmp(h2))
        .unwrap();

    println!("{}", r.1);
}


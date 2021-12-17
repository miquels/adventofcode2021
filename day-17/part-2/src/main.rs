use std::collections::HashMap;
use scan_fmt::*;

#[derive(Default, Debug)]
struct Horizontal {
    min:    i32,
    max:    i32,
    steps:  HashMap<i32, Vec<i32>>,
    max_steps:  HashMap<i32, Vec<i32>>,
}

impl Horizontal {
    fn new(min: i32, max: i32) -> Horizontal {
        Horizontal { min, max, ..Horizontal::default() }
    }

    // Find all possible steps that end up between (min_x ..= max_x).
    // There is a maximum number of steps after which we always end up there.
    fn calc(&mut self) {
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
                    self.max_steps.entry(step).or_insert(Vec::new()).push(speed);
                } else {
                    self.steps.entry(step).or_insert(Vec::new()).push(speed);
                }
            }
        }
    }
}

#[derive(Default, Debug)]
struct Vertical {
    min:      i32,
    max:      i32,
    steps:   HashMap<i32, Vec<i32>>,
}

impl Vertical {
    fn new(min: i32, max: i32) -> Vertical {
        Vertical { min, max, ..Vertical::default() }
    }
    
    // Find all possible steps that go through (min_y ..= max_y).
    fn calc(&mut self) {
        for speed in self.min ..= -self.min {
            for step in 1 .. i32::MAX {
                let pos = ((2 * speed + 1 - step) * step) / 2;
                if pos > self.max {
                    continue;
                }
                if pos < self.min {
                    break;
                }
                self.steps.entry(step).or_insert(Vec::new()).push(speed);
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

    // find overlaps.
    let mut xyspeeds = Vec::<(i32, i32)>::new();
    for (step, yspeeds) in v.steps.drain() {

        // first find exact matches.
        if let Some(xspeeds) = h.steps.get(&step) {
            for &yspeed in &yspeeds {
                for &xspeed in xspeeds {
                    xyspeeds.push((xspeed, yspeed));
                }
            }
        }

        // now find saturated matches.
        for (&max_steps, xspeeds) in h.max_steps.iter() {
            if step >= max_steps {
                for &yspeed in &yspeeds {
                    for &xspeed in xspeeds {
                        xyspeeds.push((xspeed, yspeed));
                    }
                }
            }
        }
    }
    xyspeeds.sort();
    xyspeeds.dedup();

    println!("{}", xyspeeds.len());
}


use std::collections::HashSet;
use std::io::{self, BufRead};
use scan_fmt::*;

#[derive(Default, Clone)]
struct Paper {
    dots: HashSet<[i16; 2]>,
}

impl Paper {
    fn read_dots() -> Paper {
        let mut dots = HashSet::new();
        for line in io::stdin().lock().lines().flatten() {
            if line == "" {
                break;
            }
            dots.insert(scan_fmt!(&line, "{},{}", i16, i16).map(|c| [c.0, c.1]).unwrap());
        }
        Paper { dots }
    }

    fn max_xy(&self) -> [i16; 2] {
        let max_x = self.dots.iter().map(|c| c[0]).max().unwrap();
        let max_y = self.dots.iter().map(|c| c[1]).max().unwrap();
        [max_x, max_y]
    }

    fn fold(&mut self, axis: char, apos: i16) {
        let axis = (axis == 'y') as usize;
        let max = self.max_xy()[axis];
        let off = std::cmp::max(max - apos, apos) - 1;
        self.dots = self.dots
            .drain()
            .filter_map(|mut c| {
                if c[axis] < apos {
                    c[axis] = c[axis] - apos + 1 + off;
                    Some(c)
                } else if c[axis] > apos {
                    c[axis] = apos + 1 - c[axis] + off;
                    Some(c)
                } else {
                    None
                }
            })
            .collect();
    }

    fn show(&self) {
        let max_xy = self.max_xy();
        for y in 0..=max_xy[1] {
            let mut s = String::new();
            for x in 0 ..=max_xy[0] {
                s.push(self.dots.contains(&[x, y]).then(|| '#').unwrap_or(' '));
            }
            println!("{}", s);
        }
    }
}

fn read_commands() -> Vec<(char, i16)> {
    let mut cmds = Vec::new();
    for line in io::stdin().lock().lines().flatten() {
        cmds.push(scan_fmt!(&line, "fold along {}={}", char, i16).unwrap());
    }
    cmds
}

fn main() {
    let mut paper = Paper::read_dots();
    let cmds = read_commands();

    for (i, cmd) in cmds.iter().enumerate() {
        paper.fold(cmd.0, cmd.1);
        if i == 0 {
            println!("# of dots after first instruction: {}\n", paper.dots.len());
        }
    }
    paper.show();
}

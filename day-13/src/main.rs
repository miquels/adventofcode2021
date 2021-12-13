use std::collections::HashSet;
use std::io::{self, BufRead};
use scan_fmt::*;

struct Paper {
    dots: HashSet<[i16; 2]>,
}
type Cmds = Vec<(char, i16)>;

impl Paper {
    fn max_xy(&self) -> [i16; 2] {
        let max_x = self.dots.iter().map(|c| c[0]).max().unwrap();
        let max_y = self.dots.iter().map(|c| c[1]).max().unwrap();
        [max_x, max_y]
    }

    fn fold(&mut self, axis: char, apos: i16) {
        let axis = (axis == 'y') as usize;
        let max = self.max_xy()[axis];
        let off = std::cmp::max(max - apos, apos);
        self.dots = self.dots
            .drain()
            .filter(|&[x, y]| x != y)
            .map(|mut c| {
                match c[axis] < apos {
                    true => c[axis] = c[axis] - apos + off,
                    false => c[axis] = apos - c[axis] + off,
                }
                c
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

fn read() -> (Paper, Cmds) {
    let (mut dots, mut cmds) = (HashSet::new(), Vec::new());
    for line in io::stdin().lock().lines().flatten() {
        let _ = scan_fmt!(&line, "{},{}", i16, i16).map(|c| dots.insert([c.0, c.1]));
        let _ = scan_fmt!(&line, "fold along {}={}", char, i16).map(|c| cmds.push(c));
    }
    (Paper { dots }, cmds)
}

fn main() {
    let (mut paper, cmds) = read();
    for (i, cmd) in cmds.iter().enumerate() {
        paper.fold(cmd.0, cmd.1);
        if i == 0 {
            println!("# of dots after first instruction: {}\n", paper.dots.len());
        }
    }
    paper.show();
}

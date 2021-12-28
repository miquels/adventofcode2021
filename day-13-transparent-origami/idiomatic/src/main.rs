use std::collections::HashSet;
use std::io::{self, BufRead};
use scan_fmt::*;

struct Paper {
    dots: HashSet<[i16; 2]>,
}
type Cmds = Vec<(char, i16)>;

impl Paper {
    fn fold(&mut self, axis: char, apos: i16) {
        let axis = (axis == 'y') as usize;
        self.dots = self.dots
            .drain()
            .map(|mut c| { if c[axis] > apos { c[axis] = 2 * apos - c[axis] } c })
            .collect();
    }

    fn show(&self) {
        let max = |axis| self.dots.iter().map(|c| c[axis]).max().unwrap();
        for y in 0..=max(1) {
            let mut s = String::new();
            for x in 0 ..=max(0) {
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

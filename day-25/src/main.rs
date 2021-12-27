use std::io::{self, BufRead};

struct SeaFloor {
    garden: Vec<Vec<u8>>,
    size_x: usize,
    size_y: usize,
    buf: Vec<u8>,
}

impl SeaFloor {
    fn from_stdin() -> SeaFloor {
        let garden = io::stdin()
            .lock()
            .lines()
            .map(|b| b.unwrap().into_bytes())
            .collect::<Vec<_>>();
        let (size_x, size_y) = (garden[0].len(), garden.len());
        let mut buf = Vec::new();
        buf.resize(std::cmp::max(size_x, size_y), 0);
        SeaFloor { garden, size_x, size_y, buf }
    }

    fn step_x(&mut self) -> bool {
        let mut moved = false;
        let sx = self.size_x;
        for y in 0 .. self.size_y {
            self.buf[0 .. sx].copy_from_slice(&self.garden[y]);
            for x in (0 .. self.size_x).rev() {
                if self.buf[x] == b'>' && self.buf[(x+1) % sx] == b'.' {
                    self.garden[y][x] = b'.';
                    self.garden[y][(x+1) % sx] = b'>';
                    //println!("moved {}, {} -> {},{}", y, x, y, (x+1) % sx);
                    moved = true;
                }
            }
        }
        moved
    }

    fn step_y(&mut self) -> bool {
        let mut moved = false;
        let sy = self.size_y;
        for x in 0 .. self.size_x {
            for y in 0 .. self.size_y {
                self.buf[y] = self.garden[y][x];
            }
            for y in 0 .. self.size_y {
                //println!("y {}, {}{}", y, self.buf[y] as char, self.buf[(y + 1) % sy] as char);
                if self.buf[y] == b'v' && self.buf[(y + 1) % sy] == b'.' {
                    self.garden[y][x] = b'.';
                    self.garden[(y+1) % sy][x] = b'v';
                    moved = true;
                }
            }
        }
        moved
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0 .. self.size_y {
            println!("{}", self.garden[y].iter().map(|&c| c as char).collect::<String>());
        }
        println!("");
    }
}

fn main() {
    let mut sf = SeaFloor::from_stdin();
    let mut steps = 0;
    while steps < 100000 {
        steps += 1;
        let mut moved = false;
        moved |= sf.step_x();
        moved |= sf.step_y();
        if !moved {
            break;
        }
    }
    println!("part1: {} cucumber marches", steps);
}

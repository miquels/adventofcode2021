use std::io::{self, BufRead};

struct Cave {
    dumbos: Vec<Vec<u8>>,
    max_x: usize,
    max_y: usize,
    num_dumbos: u32,
}

impl Cave {
    fn read_from_stdin() -> io::Result<Cave> {
        let dumbos = io::stdin()
            .lock()
            .lines()
            .map(|line| line.map(|l| l.bytes().map(|c| c - b'0').collect::<Vec<_>>()))
            .collect::<io::Result<Vec<_>>>()?;
        Ok(Cave {
            max_x: dumbos[0].len() - 1,
            max_y: dumbos.len() - 1,
            num_dumbos: (dumbos[0].len() * dumbos.len()) as u32,
            dumbos,
        })
    }

    fn around(&self, x: usize, y: usize) -> impl Iterator<Item=(usize, usize)> {
        let (x, y, max_x, max_y) = (x as i32, y as i32, self.max_x as i32, self.max_y as i32);
        (-1..=1).map(|dy| (-1..=1).map(move |dx| (dx, dy))).flatten()
            .filter(|(dx, dy)| *dx !=0 || *dy != 0)
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(move |(x, y)| (0..=max_x).contains(x) && (0..=max_y).contains(y))
            .map(|(x, y)| (x as usize, y as usize))
    }

    fn iter_xy(&self) -> impl Iterator<Item=(usize, usize)> {
        let (max_x, max_y) = (self.max_x, self.max_y);
        (0..=max_y).map(move |y| (0..=max_x).map(move |x| (x, y))).flatten()
    }

    fn flash(&mut self, x: usize, y: usize) -> u32 {
        self.dumbos[y][x] += 1;
        self.around(x, y)
            .filter_map(|(x, y)| {
                if self.dumbos[y][x] < 10 {
                    self.dumbos[y][x] += 1;
                }
                (self.dumbos[y][x] == 10).then(|| self.flash(x, y))
            })
            .sum::<u32>() + 1
    }

    fn step(&mut self) -> u32 {
        self.iter_xy().for_each(|(x, y)| {
            if self.dumbos[y][x] >= 10 {
                self.dumbos[y][x] = 0;
            }
            self.dumbos[y][x] += 1;
        });
        self.iter_xy()
            .filter_map(|(x, y)| {
                (self.dumbos[y][x] == 10).then(|| self.flash(x, y))
            })
            .sum::<u32>()
    }
}

fn main() -> io::Result<()> {
    let mut cave = Cave::read_from_stdin()?;
    let mut solved = 0;
    let mut flashes = 0;

    for step in 1 .. u32::MAX {
        let lit = cave.step();
        flashes += lit;
        if step == 100 {
            println!("number of flashes after step 100: {}", flashes);
            solved += 1;
        }
        if lit == cave.num_dumbos {
            println!("all dumbo's lit after step {}", step);
            solved += 1;
        }
        if solved == 2 {
            break;
        }
    }

    Ok(())
}

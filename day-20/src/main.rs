use std::io::{self, BufRead};

struct Enhancer {
    algo:   Vec<bool>
}

impl Enhancer {
    fn from_stdin() -> Enhancer {
        let line = io::stdin().lock().lines().next().unwrap().unwrap();
        let algo = line.chars().map(|c| c == '#').collect::<Vec<_>>();
        Enhancer { algo }
    }

    fn enhance1(&self, img: &Image, step: u32) -> Image {
        let (mut px, len_x, len_y) = (Vec::new(), img.len_x + 2, img.len_y + 2);
        px.resize_with(len_y as usize, || {
            let mut x = Vec::new(); x.resize((len_x) as usize, false); x
        });
        let inf = if self.algo[0] { (step & 1) != 0 } else { false };
        for y in 0 .. len_y {
            for x in 0 .. len_x {
                let index =
                    ((img.pixel_at(x - 2, y - 2, inf) as u16) << 8) |
                    ((img.pixel_at(x - 1, y - 2, inf) as u16) << 7) |
                    ((img.pixel_at(x - 0, y - 2, inf) as u16) << 6) |
                    ((img.pixel_at(x - 2, y - 1, inf) as u16) << 5) |
                    ((img.pixel_at(x - 1, y - 1, inf) as u16) << 4) |
                    ((img.pixel_at(x - 0, y - 1, inf) as u16) << 3) |
                    ((img.pixel_at(x - 2, y - 0, inf) as u16) << 2) |
                    ((img.pixel_at(x - 1, y - 0, inf) as u16) << 1) |
                    ((img.pixel_at(x - 0, y - 0, inf) as u16));
                px[y as usize][x as usize] = self.algo[index as usize];
            }
        }
        Image { pixels: px, len_x, len_y }
    }

    fn enhance(&self, mut img: Image, steps: u32) -> Image {
        for step in 0 .. steps {
            img = self.enhance1(&img, step);
        }
        img
    }
}

struct Image {
    pixels: Vec<Vec<bool>>,
    len_x: i32,
    len_y: i32,
}

impl Image {
    fn from_stdin() -> Image {
        let pixels = io::stdin()
            .lock()
            .lines()
            .flatten()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (len_x, len_y) = (pixels[0].len() as i32, pixels.len() as i32);
        Image { pixels, len_x, len_y }
    }

    #[inline]
    fn pixel_at(&self, x: i32, y: i32, inf: bool) -> bool {
        if x >= 0 && x < self.len_x && y >= 0 && y < self.len_y {
            self.pixels[y as usize][x as usize]
        } else {
            inf
        }
    }

    fn pixels_lit(&self) -> usize {
        self.pixels.iter().map(|x| x.iter().filter(|p| **p).count()).sum()
    }
}

fn main() {
    let enhancer = Enhancer::from_stdin();
    let _skip = io::stdin().lock().lines().next();
    let mut image = Image::from_stdin();
    image = enhancer.enhance(image, 2);
    println!("part1: enhanced 2 times:  pixels lit: {}", image.pixels_lit());
    image = enhancer.enhance(image, 48);
    println!("part2: enhanced 50 times: pixels lit: {}", image.pixels_lit());
}

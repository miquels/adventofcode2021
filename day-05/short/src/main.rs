use std::collections::HashMap;
use scan_fmt::*;

fn main() {
    let mut floor = HashMap::<(i16, i16), i16>::default();
    while let Ok((mut x1, mut y1, x2, y2)) = scanln_fmt!("{},{} -> {},{}", i16, i16, i16, i16) {
        let (xd, yd) = ((x2 - x1).signum(), (y2 - y1).signum());
        loop {
            *floor.entry((x1, y1)).or_insert(0) += 1;
            if x1 == x2 && y1 == y2 {
                break;
            }
            x1 += xd;
            y1 += yd;
        }
    }
    println!("part 2: {}", floor.values().filter(|&&n| n >= 2).count());
}

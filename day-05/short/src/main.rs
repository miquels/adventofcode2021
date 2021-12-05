use std::collections::HashMap;
use scan_fmt::*;

fn main() {
    let mut floor = HashMap::<(i16, i16), i16>::default();
    while let Ok(mut c) = scanln_fmt!("{},{} -> {},{}", i16, i16, i16, i16) {
        let (xd, yd) = ((c.2 - c.0).signum(), (c.3 - c.1).signum());
        while {
            *floor.entry((c.0, c.1)).or_insert(0) += 1;
            c = (c.0 + xd, c.1 + yd, c.2, c.3);
            c.0 - xd != c.2 || c.1 - yd != c.3
        } {}
    }
    println!("part 2: {}", floor.values().filter(|&&n| n >= 2).count());
}

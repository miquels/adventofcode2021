fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut fish = line.trim().split(',').map(|v| v.parse::<u8>().unwrap()).collect::<Vec<_>>();
    for _ in 0 .. 80 {
        let n = fish.iter_mut().map(|f| if *f > 0 { *f -= 1; 0 } else { *f = 6; 1 }).sum();
        fish.extend(std::iter::repeat(8).take(n));
    }
    println!("{}", fish.len());
}

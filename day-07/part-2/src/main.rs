fn read_csv_line<T: std::str::FromStr>() -> Vec<T> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(',').flat_map(|v| v.parse::<T>()).collect()
}

fn main() {
    let crabs = read_csv_line::<i64>();
    let max = crabs.iter().cloned().max().unwrap();
    let psum_abs = |n: i64| -> i64 { (0i64 ..= n.abs()).sum() };
    let r = (0..max).map(|w| crabs.iter().map(|c| psum_abs(c - w)).sum::<i64>()).min();
    println!("{}", r.unwrap());
}

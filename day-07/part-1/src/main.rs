fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let numbers = line.trim().split(',').flat_map(|v| v.parse::<i64>()).collect::<Vec<_>>();
    let max = *numbers.iter().max().unwrap_or(&0);
    let r = (0 .. max).map(|w| numbers.iter().fold(0, |a,n| a + (n - w).abs())).min();
    println!("{}", r.unwrap());
}

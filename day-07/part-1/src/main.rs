fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let numbers = line.trim().split(',').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let max = *numbers.iter().max().unwrap_or(&0);
    let sum = numbers.iter().sum::<i64>();
    let mut data = Vec::new();
    data.resize(max as usize, 0i64);
    for n in numbers {
        for w in 0 .. max {
            data[w as usize] += (n - w).abs();
        }
    }
    println!("{}", data.iter().max().unwrap());
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let numbers = line.trim().split(',').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let max = *numbers.iter().max().unwrap_or(&0);
    let mut data = Vec::new();
    data.resize(max as usize, 0i64);
    for n in numbers {
        for w in 0 .. max {
            data[w as usize] += fib((n - w).abs());
        }
    }
    println!("{}", data.iter().min().unwrap());
}

fn fib(n: i64) -> i64 {
    (0 ..= n).fold(0i64, |accum, x| accum + x)
}


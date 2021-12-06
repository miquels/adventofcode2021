fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut state = [0u64; 9];
    line.trim().split(',').map(|v| v.parse::<usize>().unwrap()).for_each(|v| state[v] += 1);
    (1..=256).map(|i| state[(i+6)%9] += state[(i+8)%9]).last();
    println!("{}", state.iter().sum::<u64>());
}

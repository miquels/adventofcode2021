fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut state = [0u64; 9];
    line.trim().split(',').map(|v| v.parse::<usize>().unwrap()).for_each(|v| state[v] += 1);
    for _ in 0 .. 256 {
        state.rotate_left(1);
        state[6] += state[8];
    }
    println!("{}", state.iter().sum::<u64>());
}

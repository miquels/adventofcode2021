fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut state = [0u64; 9].to_vec();
    line.trim().split(',').map(|v| v.parse::<usize>().unwrap()).for_each(|v| state[v] += 1);
    for _ in 0 .. 256 {
        let n = state.remove(0);
        state[6] += n;
        state.push(n);
    }
    println!("{}", state.drain(..).sum::<u64>());
}

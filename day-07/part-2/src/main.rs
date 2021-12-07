fn main() -> aoclib::Result<()> {
    let crabs = aoclib::read_csv_line::<i64>()?;
    let max = crabs.iter().cloned().max().unwrap();
    let psum_abs = |n: i64| -> i64 { (0i64 ..= n.abs()).sum() };
    let r = (0..max).map(|w| crabs.iter().map(|c| psum_abs(c - w)).sum::<i64>()).min();
    println!("{}", r.unwrap());
    Ok(())
}

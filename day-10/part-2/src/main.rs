use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let points = HashMap::from([ ('(', 1), ('[', 2), ('{', 3), ('<', 4) ]);
    let pairs = HashMap::from([ ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>') ]);
    let mut scores = Vec::new();

'LINE:
    while let Some(Ok(line)) = io::stdin().lock().lines().next() {
        let mut v = Vec::new();
        for c in line.chars() {
            match c {
                '('|'['|'{'|'<' => v.push(c),
                ')'|']'|'}'|'>' => {
                    if pairs[&v.pop().unwrap()] != c {
                        continue 'LINE;
                    }
                },
                _ => panic!("unexpected char: {}", c),
            }
        }
        let s = v.iter().rev().fold(0u64, |a, x| 5*a + points[x]);
        scores.push(s);
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let points = HashMap::from([ (')', 3), (']', 57), ('}', 1197), ('>', 25137) ]);
    let pairs = HashMap::from([ ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>') ]);
    let mut score = 0;

    while let Some(Ok(line)) = io::stdin().lock().lines().next() {
        let mut v = Vec::new();
        for c in line.chars() {
            match c {
                '('|'['|'{'|'<' => v.push(c),
                ')'|']'|'}'|'>' => {
                    if pairs[&v.pop().unwrap()] != c {
                        score += points[&c];
                        break;
                    }
                 },
                 _ => panic!("unexpected char: {}", c),
            }
        }
    }
    println!("{}", score);
}

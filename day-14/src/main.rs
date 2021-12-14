use std::collections::HashMap;
use scan_fmt::*;

struct Rule {
    letter:     char,
    pairs:      Vec<Vec<char>>,
}

struct Poly {
    rules:      HashMap<Vec<char>, Rule>,
    letters:    HashMap<char, u64>,
    pairs:      HashMap<Vec<char>, u64>,
}

impl Poly {
    fn from_stdin() -> Poly {
        let line = scanln_fmt!("{}", String).unwrap();
        let _ = scanln_fmt!("{}", String);

        let mut pairs = HashMap::new();
        for p in line.chars().collect::<Vec<_>>().windows(2) {
            *pairs.entry(p.to_vec()).or_insert(0) += 1;
        }

        let mut letters = HashMap::new();
        for c in line.chars() {
            *letters.entry(c).or_insert(0) += 1;
        }

        let mut rules = HashMap::new();
        while let Ok((pair, letter)) = scanln_fmt!("{} -> {}", String, char) {
            let pair = pair.chars().collect::<Vec<_>>();
            rules.insert(pair.clone(), Rule {
                pairs:  vec![ vec![pair[0], letter], vec![letter, pair[1]] ],
                letter,
            });
        }

        Poly { rules, letters, pairs }
    }

    fn step(&mut self) {
        let mut pairs = std::mem::replace(&mut self.pairs, HashMap::new());
        for (pair, value) in pairs.drain() {
            let rule = &self.rules[&pair];
            *self.letters.entry(rule.letter).or_insert(0) += value;
            for p in &rule.pairs {
                *self.pairs.entry(p.clone()).or_insert(0) += value;
            }
        }
    }

    fn report(&self, step: usize) {
        let mut letters = self.letters.iter().map(|(c, n)| (*c, *n)).collect::<Vec<_>>();
        letters.sort_by(|a, b| a.1.cmp(&b.1));
        let (first, last) = (letters[0], letters.last().unwrap());
        println!("step {}: {:?} - {:?} = {}", step, last, first, last.1 - first.1);
    }
}

fn main() {
    let mut poly = Poly::from_stdin();

    for step in 1 ..=40 {
        poly.step();
        if step == 10 || step == 40 {
            poly.report(step);
        }
    }
}
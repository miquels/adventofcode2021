use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Default, Debug, Clone)]
struct Number {
    first: PairId,
    pairs: HashMap<PairId, Pair>,
}

impl Number {
    fn parse(input: &mut &str) -> Number {
        let mut number = Number::default();
        Pair::parse(input, &mut number, 0);
        number
    }

    fn print_item(&self, item: &Item, s: &mut String) {
        match item {
            Item::Regular(item) => s.push_str(&format!("{}", item)),
            Item::Pair(pair) => self.print_pair(*pair, s),
        }
    }

    fn print_pair(&self, pair_id: PairId, s: &mut String) {
        let pair = &self.pairs[&pair_id];
        s.push('[');
        self.print_item(&pair.items[0], s);
        s.push(',');
        self.print_item(&pair.items[1], s);
        s.push(']');
    }

    fn add_list(&self, list: &mut Vec<PairId>, pair_id: PairId) {
        let mut added = false;
        let pair = &self.pairs[&pair_id];
        match pair.items[0] {
            Item::Regular(_) => {
                list.push(pair_id);
                added = true;
            },
            Item::Pair(p) => self.add_list(list, p),
        }
        if !added {
            list.push(pair_id);
        }
        match pair.items[1] {
            Item::Regular(_) => {},
            Item::Pair(p) => self.add_list(list, p),
        }
    }

    fn make_list(&self) -> Vec<PairId> {
        let mut list = Vec::new();
        self.add_list(&mut list, self.first);
        list
    }

    fn split(&mut self) -> bool {
        let mut idx = 0;
        let list = self.make_list();
        while idx < list.len() {
            for i in 0 .. 2 {
                let pair = self.pairs.get_mut(&list[idx]).unwrap();
                match pair.items[i] {
                    Item::Regular(num) => {
                        if num >= 10 {
                            let mut np = Pair::new(pair.depth + 1);
                            np.items[0] = Item::Regular(num / 2);
                            np.items[1] = Item::Regular((num + 1) / 2);
                            np.parent = Some(pair.id);
                            pair.items[i] = Item::Pair(np.id);
                            self.pairs.insert(np.id, np);
                            return true;
                        }
                    },
                    Item::Pair(_) => {},
                }
            }
            idx += 1;
        }

        false
    }

    fn explode(&mut self) -> bool {

        let list = self.make_list();

        let mut idx = 0;
        while idx < list.len() {

            let pair_id = list[idx];
            let pair = &self.pairs[&pair_id];
            if pair.depth < 4 {
                idx += 1;
                continue;
            }

            let (left, right) = match &pair.items {
                [Item::Regular(left), Item::Regular(right)] => (*left, *right),
                _ => {
                    idx += 1;
                    continue;
                },
            };

            let mut li = idx;
            while li > 0 {
                li -= 1;
                let pair = self.pairs.get_mut(&list[li]).unwrap();
                if let Item::Regular(ref mut num) = pair.items[1] {
                    *num += left;
                    break;
                }
                if let Item::Regular(ref mut num) = pair.items[0] {
                    *num += left;
                    break;
                }
            }

            let mut ri = idx;
            while ri + 1 < list.len() {
                ri += 1;
                let pair = self.pairs.get_mut(&list[ri]).unwrap();
                if let Item::Regular(ref mut num) = pair.items[0] {
                    *num += right;
                    break;
                }
                if let Item::Regular(ref mut num) = pair.items[1] {
                    *num += right;
                    break;
                }
            }

            if let Some(parent_id) = self.pairs[&pair_id].parent {
                for i in 0 .. 2 {
                    match self.pairs[&parent_id].items[i] {
                        Item::Pair(p) => {
                            if p == pair_id {
                                self.pairs.get_mut(&parent_id).unwrap().items[i] = Item::Regular(0);
                                self.pairs.remove(&pair_id);
                                break;
                            }
                        },
                        Item::Regular(_) => {},
                    }
                }
            }

            return true;
        }

        false
    }

    fn reduce(&mut self) {
        loop {
            while self.explode() {
            }
            if !self.split() {
                break;
            }
        }
    }

    fn add(&mut self, other: &Number) {
        let mut pair = Pair::new(0);
        pair.items[0] = Item::Pair(self.first);
        pair.items[1] = Item::Pair(other.first);
        self.first = pair.id;
        self.pairs.extend(other.pairs.iter().map(|(k, v)| (k.clone(), v.clone())));
        for pair in self.pairs.values_mut() {
            if pair.parent.is_none() {
                pair.parent = Some(self.first);
            }
            pair.depth += 1;
        }
        self.pairs.insert(pair.id, pair);
        self.reduce();
    }

    fn item_magnitude(&self, item: &Item) -> i32 {
        match item {
            Item::Regular(r) => *r,
            Item::Pair(p) => self.pair_magnitude(*p),
        }
    }

    fn pair_magnitude(&self, pair_id: PairId) -> i32 {
        let pair = &self.pairs[&pair_id];
        3 * self.item_magnitude(&pair.items[0]) + 2 * self.item_magnitude(&pair.items[1])
    }

    fn magnitude(&self) -> i32 {
        self.pair_magnitude(self.first)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.print_pair(self.first, &mut s);
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
enum Item {
    Regular(i32),
    Pair(PairId),
}

impl Item {
    fn parse(input: &mut &str, number: &mut Number, parent_id: PairId, depth: u32) -> Item {
        if input.is_empty() {
            panic!("Unexpected EOL");
        }
        if &(*input)[..1] == "[" {
            let pair = Pair::parse(input, number, depth + 1);
            number.pairs.get_mut(&pair).unwrap().parent = Some(parent_id);
            return Item::Pair(pair);
        }
        let len = input.len();
        let mut num = 0;
        while let Some(c) = input.chars().next() {
            if !c.is_numeric() {
                break;
            }
            num = 10 * num + (c as i32 - 48);
            *input = &(*input)[1..];
        }
        if input.len() == len {
            panic!("parse error, expected regular number at {}", *input);
        }
        return Item::Regular(num);
    }
}

#[derive(Default, Debug, Hash, Clone, Eq, PartialEq, Copy)]
struct PairId {
    id: u32,
}

impl PairId {
    fn new() -> PairId {
        static ID: AtomicU32 = AtomicU32::new(1);
        PairId{ id: ID.fetch_add(1, Ordering::Relaxed) }
    }

    fn is_zero(&self) -> bool {
        self.id == 0
    }
}

#[derive(Debug, Clone)]
struct Pair {
    id:         PairId,
    parent:     Option<PairId>,
    items:      [Item; 2],
    depth:      u32,
}

impl Pair {
    fn parse(input: &mut &str, number: &mut Number, depth: u32) -> PairId {

        let mut pair = Pair::new(depth);
        if number.first.is_zero() {
            number.first = pair.id;
        }

        expect(input, "[");
        pair.items[0] = Item::parse(input, number, pair.id, depth);
        expect(input, ",");
        pair.items[1] = Item::parse(input, number, pair.id, depth);
        expect(input, "]");

        let pair_id = pair.id;
        number.pairs.insert(pair.id, pair);
        pair_id
    }

    fn new(depth: u32) -> Pair {
        Pair {
            id: PairId::new(),
            items: [Item::Regular(0), Item::Regular(0)],
            parent: None,
            depth,
        }
    }
}

fn expect(input: &mut &str, s: &str) {
    if input.len() == 0 {
        panic!("unexpected EOL");
    }
    if &(*input)[..s.len()] != s {
        panic!("parse error, expected '{}' at {}", s, *input);
    }
    *input = &(*input)[s.len()..];
}

fn main() {
    let numbers = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| Number::parse(&mut line.as_str()))
        .collect::<Vec<_>>();
    let mut first = numbers[0].clone();
    for num in &numbers[1..] {
        first.add(num);
    }
    println!("sum:        {}", first);
    println!("mangnitude: {}", first.magnitude());

    let mut max = 0;
    for x in 0 .. numbers.len() {
        for y in 0 .. numbers.len() {
            if x == y {
                continue;
            }
            let mut n = numbers[x].clone();
            n.add(&numbers[y]);
            let m = n.magnitude();
            if m > max {
                max = m;
            }
        }
    }
    println!("largest magnitude of any sum of two numbers: {}", max);
}


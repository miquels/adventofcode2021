use std::cmp::{self, Ordering};
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Operand {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone)]
enum Value {
    Const(i64),
    Range(Vec<(i64, i64)>),
    Reg(usize),
}

impl Default for Value {
    fn default() -> Value {
        Value::Const(0)
    }
}

impl Value {
    fn parse(v: &str) -> Value {
        if let Ok(n) = v.parse::<i64>() {
            return Value::Const(n);
        }
        Value::Reg((v.as_bytes()[0] - b'w') as usize)
    }

    fn squash(mut v: Vec<(i64, i64)>) -> Value {
        // make sure ranges increase.
        for r in v.iter_mut() {
            if r.0 > r.1 {
                std::mem::swap(&mut r.0, &mut r.1);
            }
        }
        // sort ranges by overlap.
        v.sort_by(|(a, b), (x, y)| {
            if a == x && b == y {
                Ordering::Equal
            } else if b < x {
                Ordering::Less
            } else if a >y {
                Ordering::Greater
            } else if a < x {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        let o = v.clone();
        // merge overlapping ranges.
        let mut idx = 1;
        while idx < v.len() {
            let (a, b) = v[idx - 1];
            let (x, y) = v[idx];
            if b >= (x - 1) && a <= (y + 1) {
                v.remove(idx);
                v[idx-1] = (cmp::min(a, x), cmp::max(b, y));
            } else {
                idx += 1;
            }
        }

        for idx in 1 .. v.len() {
            if v[idx-1].1 + 1 == v[idx].0 {
                println!("{:?}", o);
                println!("");
                println!("{:?}", v);
                println!("");
                panic!("failed to merge ranges {:?} {:?}", v[idx-1], v[idx]);
            }
        }

        // This is not very precise, but otherwise things get out of hand quick.
        if v.len() > 150 {
            let last = v[v.len() - 1].1;
            v.truncate(150);
            v[149].1 = last;
        }

        if is_const(&v) {
            Value::Const(v[0].0)
        } else {
            Value::Range(v)
        }
    }

    fn exec(&self, other: &Value, operand: Operand, regs: &Regs) -> Value {
        // First, reduce all values down to ranges.
        let (mut r1, mut r2) = match self {
            Value::Const(a) => return Value::Range(vec![(*a, *a)]).exec(other, operand, regs),
            Value::Reg(r) => return self.exec(&regs[*r], operand, regs),
            Value::Range(r1) => {
                match other {
                    Value::Const(a) => {
                        return self.exec(&Value::Range(vec![(*a, *a)]), operand, regs);
                    },
                    Value::Reg(r) => return self.exec(&regs[*r], operand, regs),
                    Value::Range(r2) => (r1, r2),
                }
            }
        };

        let v: Vec<(i64, i64)> = match operand {
            Operand::Add => {
                r1
                   .iter()
                   .map(|&(a, b)| r2.iter().map(move |&(x, y)| {
                       (a.saturating_add(x), b.saturating_add(y))
                    }))
                   .flatten()
                   .collect()
            },
            Operand::Div => {
                if r2.len() != 1 || r2[0].0 != r2[0].1 {
                    panic!("div: cannot divide by a range");
                }
                let x = r2[0].0;
                r1.iter().map(|&(a, b)| (a / x, b / x)).collect()
            },
            Operand::Mod => {
                if r2.len() != 1 || r2[0].0 != r2[0].1 {
                    panic!("mod: cannot mod by a range");
                }
                let x = r2[0].0;
                r1.iter().map(|&(a, b)| {
                    if b - a >= x {
                        vec![(0, x-1)]
                    } else {
                        let ma = a % x;
                        let mb = b % x;
                        if mb < ma {
                            vec![(0, ma), (ma, x - 1)]
                        } else {
                            vec![(ma, mb)]
                        }
                    }
                })
                .flatten()
                .collect()
            },
            Operand::Mul => {
                // Make sure first range is the smallest.
                let s1 = r1.iter().map(|&(a, b)| b - a + 1).sum::<i64>();
                let s2 = r2.iter().map(|&(a, b)| b - a + 1).sum::<i64>();
                /*
                if s1 * s2 > 1000 {
                    println!("r1 is {}, r2 = {}", r1.len(), r2.len());
                    println!("s1 is {}, s2 = {}", s1, s2);
                }*/
                if s1 > s2 {
                    std::mem::swap(&mut r1, &mut r2);
                    /*
                    if s1 * s2 > 1000 {
                        println!("swapped");
                    }*/
                }
                /*
                if r1.len() != 1 && r2.len() != 1 {
                    println!("{:?}\n", r1);
                    println!("{:?}\n", r2);
                    panic!("mul: cannot multiply a multi-range by a multi-range");
                }*/
                let mut v = Vec::new();
                for &(a, b) in r1 {
                    match (a, b) {
                        (0, 0) => v.push((0, 0)),
                        (0, 1) => {
                            v.push((0, 1));
                            v.extend(r2);
                        },
                        (1, 1) => v.extend(r2),
                        (a, b) => {
                            let n = r2.iter().map(|&(x, y)| y - x + 1).sum::<i64>();
                            if (b - a + 1) * n > 50 {
                                v.push((a * r2[0].0, b * r2[r2.len()-1].1));
                            } else {
                                for &(x, y) in r2 {
                                    for a in a ..= b {
                                        for x in x ..= y {
                                            v.push((a * x, a * x));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                v
            },
            Operand::Eql => {
                let overlap = r1.iter().any(|&(a, b)| r2.iter().any(|&(x, y)| a <= y && b >= x));
                if is_const(r1) && is_const(r2) {
                    vec![(overlap as i64, overlap as i64)]
                } else {
                    if !overlap {
                        vec![(0, 0)]
                    } else {
                        vec![(0, 1)]
                    }
                }
            },
            Operand::Inp => unreachable!(),
        };
        Value::squash(v)
    }

    fn maybe_zero(&self) -> bool {
        match self {
            Value::Const(0) => true,
            Value::Range(r) => r.iter().any(|&(x, y)| x <= 0 && y >= 0),
            _ => false,
        }
    }
}

fn is_const(v: &Vec<(i64, i64)>) -> bool {
    v.len() == 1 && v[0].0 == v[0].1
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Const(x) => write!(f, "{}", x),
            Value::Range(r) => {
                if r.len() == 1 {
                    if is_const(r) {
                        write!(f, "{}", r[0].0)
                    } else {
                        write!(f, "{}..={}", r[0].0, r[0].1)
                    }
                } else {
                    let mut s = String::new();
                    for i in 0 .. r.len() {
                        let (x, y) = r[i];
                        s.push_str(&format!("{}..{}", x, y));
                        if i < r.len() - 1 {
                            s += ",";
                        }
                    }
                    write!(f, "[{}]", s)
                }
            },
            Value::Reg(r) => write!(f, "{}", (*r as u8 + b'w') as char),
        }
    }
}

type Regs = [Value; 4];

#[derive(Debug)]
struct Instruction {
    reg:    usize,
    op:     Operand,
    val:    Value,
}

impl Instruction {
    fn read() -> Option<Instruction> {
        let mut s = String::new();
        if io::stdin().lock().read_line(&mut s).unwrap() == 0 {
            return None;
        }
        let w = s.split_whitespace().collect::<Vec<_>>();
        let reg = (w[1].as_bytes()[0] - b'w') as usize;
        let (op, val) = match w[0] {
            "inp" => (Operand::Inp, Value::Range(vec![(1, 9)])),
            "add" => (Operand::Add, Value::parse(w[2])),
            "mul" => (Operand::Mul, Value::parse(w[2])),
            "div" => (Operand::Div, Value::parse(w[2])),
            "mod" => (Operand::Mod, Value::parse(w[2])),
            "eql" => (Operand::Eql, Value::parse(w[2])),
            other => panic!("unknown instruction \"{}\"", other),
        };
        Some(Instruction{ reg, op, val })
    }

    fn exec(&self, regs: &mut Regs, input: &mut &[i64]) {
        let regs2 = regs.clone();
        match self.op {
            Operand::Inp => {
                regs[self.reg] = match input[0] {
                    1 ..= 9 => Value::Const(input[0]),
                    _ => Value::Range(vec![(1, 9)]),
                };
                *input = &(*input)[1..];
            },
            op => regs[self.reg] = regs2[self.reg].exec(&self.val, op, regs),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reg = (self.reg as u8 + b'w') as char;
        match self.op {
            Operand::Inp => write!(f, "inp {}", reg),
            Operand::Add => write!(f, "add {} {}", reg, self.val),
            Operand::Mul => write!(f, "mul {} {}", reg, self.val),
            Operand::Div => write!(f, "div {} {}", reg, self.val),
            Operand::Mod => write!(f, "mod {} {}", reg, self.val),
            Operand::Eql => write!(f, "eql {} {}", reg, self.val),
        }
    }
}

#[derive(Debug, Default)]
struct ALU {
    regs:   Regs,
    prog:   Vec<Instruction>,
    input:  [i64; 14],
}

impl ALU {
    fn load(&mut self) {
        while let Some(insn) = Instruction::read() {
            self.prog.push(insn);
        }
    }

    fn run(&mut self, dbg: bool) {
        self.regs.fill(Value::Const(0));
        let mut input = &self.input[..];
        if dbg {
            println!("{:15} -> [{}, {}, {}, {}]", "", self.regs[0], self.regs[1], self.regs[2], self.regs[3]);
        }
        for i in 0 .. self.prog.len() {
            self.prog[i].exec(&mut self.regs, &mut input);
            if dbg {
                println!("{:15} -> [{}, {}, {}, {}]", self.prog[i], self.regs[0], self.regs[1], self.regs[2], self.regs[3]);
            }
        }
    }

    fn input(&self) -> String {
        self.input.iter().map(|&c| (c as u8 + b'0') as char).collect::<String>()
    }
}

fn main() {
    let mut alu = ALU::default();
    alu.load();
    solve1(&mut alu);
    //solve2(&mut alu);
}

fn solve2(alu: &mut ALU) {
    let mut idx = 0;
    alu.input.fill(0);
'LOOP:
    while idx < 14 {
        for n in 1 ..=9 {
            alu.input[idx] = n;
            alu.run(true);
            if alu.regs[3].maybe_zero() {
                println!("OK  {} z={}", alu.input(), alu.regs[3]);
                idx += 1;
                continue 'LOOP;
            }
            println!("BAD {} z={}", alu.input(), alu.regs[3]);
        }
        println!("FAIL");
        break;
    }
}

fn solve1(alu: &mut ALU) {
    let mut idx = 0;
    alu.input.fill(0);
'LOOP:
    while idx < 14 {
        for n in (1 ..=9).rev() {
            alu.input[idx] = n;
            alu.run(false);
            if alu.regs[3].maybe_zero() {
                //println!("OK  {} z={}", alu.input(), alu.regs[3]);
                idx += 1;
                continue 'LOOP;
            }
            //println!("BAD {} z={}", alu.input(), alu.regs[3]);
        }
        println!("FAIL");
        return;
    }
    println!("part1: {}", alu.input());
}


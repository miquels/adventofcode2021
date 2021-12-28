use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Value {
    Const(i64),
    Reg(usize),
}

impl Value {
    fn parse(v: &str) -> Value {
        if let Ok(n) = v.parse::<i64>() {
            return Value::Const(n);
        }
        Value::Reg((v.as_bytes()[0] - b'w') as usize)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Const(x) => write!(f, "{}", x),
            Value::Reg(r) => write!(f, "{}", (*r as u8 + b'w') as char),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operand {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone, Copy, Default)]
struct Regs {
    reg: [i64; 4],
}

impl fmt::Display for Regs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[w: {:7}, x: {:7}, y: {:7}, z: {:7}]",
            self.reg[0], self.reg[1], self.reg[2], self.reg[3]
        )
    }
}

#[derive(Debug)]
struct Instruction {
    reg: usize,
    op: Operand,
    val: Value,
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
            "inp" => (Operand::Inp, Value::Const(0)),
            "add" => (Operand::Add, Value::parse(w[2])),
            "mul" => (Operand::Mul, Value::parse(w[2])),
            "div" => (Operand::Div, Value::parse(w[2])),
            "mod" => (Operand::Mod, Value::parse(w[2])),
            "eql" => (Operand::Eql, Value::parse(w[2])),
            other => panic!("unknown instruction \"{}\"", other),
        };
        Some(Instruction { reg, op, val })
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
    regs: Regs,
    prog: Vec<Instruction>,
    pc: usize,
    input: [i64; 14],
    inp: usize,
    dbg: bool,
}

#[derive(Clone, Copy)]
struct AluState {
    regs: Regs,
    pc: usize,
    input: [i64; 14],
    inp: usize,
}

impl ALU {
    fn load_program(&mut self) {
        while let Some(insn) = Instruction::read() {
            self.prog.push(insn);
        }
    }

    fn checkpoint(&self) -> AluState {
        AluState { regs: self.regs, pc: self.pc, input: self.input, inp: self.inp }
    }

    fn restore(&mut self, state: &AluState) {
        self.regs = state.regs;
        self.pc = state.pc;
        self.input = state.input;
        self.inp = state.inp;
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.pc = 0;
        self.inp = 0;
        self.regs.reg.fill(0);
    }

    fn end(&self) -> bool {
        self.pc == self.prog.len()
    }

    fn get(&self, val: &Value) -> i64 {
        match val {
            Value::Const(v) => *v,
            Value::Reg(r) => self.regs.reg[*r],
        }
    }

    fn step(&mut self) {
        let insn = &self.prog[self.pc];
        let regs = self.regs;
        let r = insn.reg;
        match insn.op {
            Operand::Inp => {
                self.regs.reg[r] = self.input[self.inp];
                self.inp += 1;
            }
            Operand::Add => self.regs.reg[r] += self.get(&insn.val),
            Operand::Mul => self.regs.reg[r] *= self.get(&insn.val),
            Operand::Div => self.regs.reg[r] /= self.get(&insn.val),
            Operand::Mod => self.regs.reg[r] %= self.get(&insn.val),
            Operand::Eql => self.regs.reg[r] = (self.regs.reg[r] == self.get(&insn.val)) as i64,
        }
        if self.dbg {
            println!("{:3} {}   {} -> {}", self.pc, regs, &self.prog[self.pc], self.regs.reg[r]);
        }
        self.pc += 1;
    }

    #[allow(dead_code)]
    fn run(&mut self) {
        while self.pc < self.prog.len() {
            self.step();
        }
    }

    fn input(&self) -> String {
        self.input
            .iter()
            .map(|&c| (c as u8 + b'0') as char)
            .collect::<String>()
    }
}

fn run_block(alu: &mut ALU) {
    loop {
        alu.step();
        if alu.end() || alu.prog[alu.pc].op == Operand::Inp {
            break;
        }
    }
}

fn solve(alu: &mut ALU, depth: usize) -> bool {

    // Check if we're done.
    if depth == 14 {
        return alu.regs.reg[3] == 0;
    }

    // Find the next magic constant.
    let mut magic = 0;
    for pc in alu.pc .. alu.prog.len() {
        if alu.prog[pc].op == Operand::Add && alu.prog[pc].reg == 1 {
            if let Value::Const(v) = alu.prog[pc].val {
                magic = v;
                break;
            }
        }
    }

    // If we cannot predict a number, go forward.
    if magic <= -25 || magic >= 10 {
        let state = alu.checkpoint();
        for i in 1 ..= 9 {
            alu.input[depth] = i;
            run_block(alu);
            if solve(alu, depth + 1) == true {
                return true;
            }
            alu.restore(&state);
        }
        return false;
    }

    let x = (alu.regs.reg[3] % 26) + magic;
    if x < 1 || x > 9 {
        // If we cannot calculate a valid digit, return false so that
        // we try again with different input leading up to here.
        return false;
    }
    alu.input[depth] = x;
    run_block(alu);
    solve(alu, depth + 1)
}

fn main() {
    let mut alu = ALU::default();
    alu.load_program();
    alu.input.fill(1);
    if solve(&mut alu, 0) {
        println!("part2: lowest valid model number: {}", alu.input());
    } else {
        println!("part2: FAILED");
    }
}

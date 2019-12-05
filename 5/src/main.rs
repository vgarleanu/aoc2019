use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;

type Operand = (i64, Mode);

#[derive(Debug, PartialEq)]
enum Mode {
    Pos,
    Imm,
}

impl Mode {
    fn get_abs(&self, mem: &Vec<i64>, val: i64) -> i64 {
        match self {
            Mode::Pos => mem[val as usize],
            Mode::Imm => val,
        }
    }
}

#[derive(Debug)]
enum Operation {
    // Params: r1, r2, r3,
    Add(Operand, Operand, Operand),
    Mul(Operand, Operand, Operand),
    Inp(Operand),
    Out(Operand),
    Jit(Operand, Operand),
    Jif(Operand, Operand),
    Lst(Operand, Operand, Operand),
    Equ(Operand, Operand, Operand),
    Hlt,
}

impl Operation {
    fn from(mem: &Vec<i64>, pc: usize) -> (usize, Self) {
        let inst = mem[pc]
            .to_string()
            .chars()
            .rev()
            .map(|x| x.to_digit(10).unwrap())
            .map(|x| x as i64)
            .collect::<Vec<i64>>();

        let mut inst_iter = inst.iter();
        let i1 = inst_iter.next().unwrap();

        let inst = inst_iter
            .next()
            .map_or_else(|| *i1, |x| concat(*x as u64, *i1 as u64) as i64);

        let r1_m = inst_iter.next().map_or_else(
            || Mode::Pos,
            |x| match x {
                0 => Mode::Pos,
                1 => Mode::Imm,
                _ => panic!("Weird ass mode: {}, pc: {}", x, pc),
            },
        );

        let r2_m = inst_iter.next().map_or_else(
            || Mode::Pos,
            |x| match x {
                0 => Mode::Pos,
                1 => Mode::Imm,
                _ => panic!(),
            },
        );

        let r3_m = inst_iter.next().map_or_else(
            || Mode::Pos,
            |x| match x {
                0 => Mode::Pos,
                1 => Mode::Imm,
                _ => panic!(),
            },
        );

        let r1 = mem.get(pc + 1).map(|x| *x).unwrap_or(0);
        let r2 = mem.get(pc + 2).map(|x| *x).unwrap_or(0);
        let r3 = mem.get(pc + 3).map(|x| *x).unwrap_or(0);

        match inst {
            1 => (pc + 4, Operation::Add((r1, r1_m), (r2, r2_m), (r3, r3_m))),
            2 => (pc + 4, Operation::Mul((r1, r1_m), (r2, r2_m), (r3, r3_m))),
            3 => (pc + 2, Operation::Inp((r1, r1_m))),
            4 => (pc + 2, Operation::Out((r1, r1_m))),
            5 => (pc + 3, Operation::Jit((r1, r1_m), (r2, r2_m))),
            6 => (pc + 3, Operation::Jif((r1, r1_m), (r2, r2_m))),
            7 => (pc + 4, Operation::Lst((r1, r1_m), (r2, r2_m), (r3, r3_m))),
            8 => (pc + 4, Operation::Equ((r1, r1_m), (r2, r2_m), (r3, r3_m))),
            99 => (pc + 1, Operation::Hlt),
            _ => panic!("Weird inst: {}, pc: {}", inst, pc),
        }
    }
}

struct VM {
    mem: Vec<i64>,
    pc: usize,
}

impl VM {
    fn new(vm: Vec<i64>) -> Self {
        Self { mem: vm, pc: 0 }
    }

    fn start(&mut self) {
        'vm: loop {
            let (new_pc, inst) = Operation::from(&self.mem, self.pc);
            self.pc = new_pc;
            match inst {
                Operation::Add(r1, r2, r3) => self.add(r1, r2, r3),
                Operation::Mul(r1, r2, r3) => self.mul(r1, r2, r3),
                Operation::Inp(r1) => self.input(r1),
                Operation::Out(r1) => self.output(r1),
                Operation::Jit(r1, r2) => self.jump_if_true(r1, r2),
                Operation::Jif(r1, r2) => self.jump_if_false(r1, r2),
                Operation::Lst(r1, r2, r3) => self.less_than(r1, r2, r3),
                Operation::Equ(r1, r2, r3) => self.equals(r1, r2, r3),
                Operation::Hlt => break 'vm,
            }
        }
    }

    fn add(&mut self, r1: Operand, r2: Operand, r3: Operand) {
        let r1 = r1.1.get_abs(&self.mem, r1.0);
        let r2 = r2.1.get_abs(&self.mem, r2.0);

        assert_eq!(r3.1, Mode::Pos);

        self.mem[r3.0 as usize] = r1 + r2;
    }

    fn mul(&mut self, r1: Operand, r2: Operand, r3: Operand) {
        let r1 = r1.1.get_abs(&self.mem, r1.0);
        let r2 = r2.1.get_abs(&self.mem, r2.0);

        assert_eq!(r3.1, Mode::Pos);

        self.mem[r3.0 as usize] = r1 * r2;
    }

    fn input(&mut self, r1: Operand) {
        assert_eq!(r1.1, Mode::Pos);

        self.mem[r1.0 as usize] = get_input(">")
            .parse::<i64>()
            .expect("Expected a signed 64 bit int");
    }

    fn output(&self, r1: Operand) {
        let r1 = r1.1.get_abs(&self.mem, r1.0);
        println!("R1 is: {}", r1);
    }

    fn jump_if_true(&mut self, r1: Operand, r2: Operand) {
        let r1 = r1.1.get_abs(&self.mem, r1.0);
        let r2 = r2.1.get_abs(&self.mem, r2.0);
        if r1 != 0 {
            self.pc = r2 as usize;
        }
    }

    fn jump_if_false(&mut self, r1: Operand, r2: Operand) {
        let r1 = r1.1.get_abs(&self.mem, r1.0);
        let r2 = r2.1.get_abs(&self.mem, r2.0);

        if r1 == 0 {
            self.pc = r2 as usize;
        }
    }

    fn less_than(&mut self, r1: Operand, r2: Operand, r3: Operand) {
        let r1 = r1.1.get_abs(&self.mem, r1.0);
        let r2 = r2.1.get_abs(&self.mem, r2.0);
        let r3 = r3.1.get_abs(&self.mem, r3.0);

        self.mem[r3 as usize] = if r1 < r2 { 1 } else { 0 }
    }

    fn equals(&mut self, r1: Operand, r2: Operand, r3: Operand) {
        let r1 = r1.1.get_abs(&self.mem, r1.0);
        let r2 = r2.1.get_abs(&self.mem, r2.0);
        let r3 = r3.1.get_abs(&self.mem, r3.0);

        self.mem[r3 as usize] = if r1 == r2 { 1 } else { 0 }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Please enter a valid number");

    input.trim().to_string()
}

fn concat(a: u64, b: u64) -> u64 {
    let mut pow = 10;
    while b >= pow {
        pow *= 10;
    }

    a * pow + b
}

fn main() -> std::io::Result<()> {
    /*
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    */

    let contents = get_input("Program:");

    let program = contents
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let mut vm = VM::new(program);
    vm.start();

    Ok(())
}

use std::fs::File;
use std::io::prelude::*;

fn vm(p1: u32, p2: u32, mut program: Vec<u32>) -> Vec<u32> {
    program[1] = p1;
    program[2] = p2;

    let mut pc = 0;
    loop {
        let opcode = program[pc];
        let v1 = program[pc + 1] as usize;
        let v2 = program[pc + 2] as usize;
        let out = program[pc + 3] as usize;
        match opcode {
            1 => program[out] = program[v1] + program[v2],
            2 => program[out] = program[v1] * program[v2],
            99 => break,
            _ => {
                println!("Unknown opcode");
                break;
            }
        }
        pc += 4;
    }

    program
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let program = contents
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<_>>();

    for noun in 1..100 {
        for verb in 1..100 {
            if vm(noun, verb, program.clone())[0] == 19690720 {
                println!("Found: {}", 100 * noun + verb);
            }
        }
    }
    Ok(())
}

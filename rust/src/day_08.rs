// Although brute forcing your way in seems like an easy solution, its better to use
//  a hash to check if the second number exists instead of iterating through the whole
//  set of numbers twice. The second part is pretty similar to the first one and can follow the same
//  principle.

use anyhow::*;
use std::collections::HashSet;

#[aoc_generator(day8)]
pub fn input_gen(input: &str) -> Vec<Instruction> {
    parse_input(input)
}

#[aoc(day8, part1)]
pub fn solve_part1(data: &Vec<Instruction>) -> Result<i32> {
    Ok(run_until_loop(data)?.1)
}

#[aoc(day8, part2)]
pub fn solve_part2(data: &Vec<Instruction>) -> Result<i32> {
    solve_2(data)
}

#[derive(Clone,Debug, PartialEq, Eq)]
pub enum Instruction {
    Acc(i32), // Increments acc by a certain value
    Jmp(i32), // Jumps a certain value
    Nop(i32), // Does nothing
}

impl Instruction {
    fn from_strs(buf: &str, num: &str) -> Self {
        //        dbg!(buf);
        match buf {
            "acc" => Instruction::Acc(num.parse().unwrap()),
            "jmp" => Instruction::Jmp(num.parse().unwrap()),
            _ => Instruction::Nop(num.parse().unwrap()),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Instruction::Acc(x) => format!("acc {}", x),
            Instruction::Jmp(x) => format!("jmp {}", x),
            Instruction::Nop(x) => format!("nop {}", x),
        }
    }

    fn run(&self, curr_i: usize, acc: &mut i32) -> usize {
        let mut next_i = curr_i;
        match self {
            Instruction::Acc(v) => {
                *acc += v;
                next_i += 1;
            }
            Instruction::Jmp(v) => {
                next_i += *v as usize;
            }
            Instruction::Nop(_) => next_i += 1,
        };
        next_i
    }

    fn is_jmp_or_nop(&self) -> bool {
        match self {
            Instruction::Jmp(_) => true,
            Instruction::Nop(_) => true,
            _ => false,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|v| (Instruction::from_strs(*v.get(0).unwrap(), *v.get(1).unwrap())))
        .collect::<Vec<Instruction>>()
}

// Returns (false, <acc>) if there program finished naturally or (true, <acc>) if it encountered a loop
pub fn run_until_loop(data: &Vec<Instruction>) -> Result<(bool, i32)> {
    let mut acc_total: i32 = 0;
    let mut visited_instr = HashSet::<usize>::new();
    let mut instr_i = 0;
    let mut loop_found = false;

    while let Some(instr) = data.get(instr_i) {
        if visited_instr.contains(&instr_i) == true {
            loop_found = true;
            break;
        }
        visited_instr.insert(instr_i);
        instr_i = instr.run(instr_i, &mut acc_total);
    }

    Ok((loop_found, acc_total))
}

pub fn flip_instr(intr: &Instruction) -> Result<Instruction> {
    match intr {
        Instruction::Jmp(x) => Ok(Instruction::Nop(*x)),
        Instruction::Nop(x) => Ok(Instruction::Jmp(*x)),
        Instruction::Acc(_) => bail!("Tried to flip an add"),
    }
}

// gets the next jmp or nop in a vector after an offset and returns a copy of the data and the
// index of the instruction that was changed
// This could be improved by simply scanning the data while running the program
pub fn flip_jmp_or_nop(data: &Vec<Instruction>, offset: usize) -> Result<(usize, Vec<Instruction>)> {
    // Find the index of the next instruction to flip
    let index = data[offset..]
        .iter()
        .enumerate()
        .find(|(i, x)| x.is_jmp_or_nop())
        .unwrap()
        .0;

    // Makes a copy of the data, flipping an instruction
    let mut new_data = data.clone();
    new_data[offset+index] = flip_instr(&data[offset+index])?;
    Ok((offset+index, new_data.to_vec()))
}

pub fn solve_2(data: &Vec<Instruction>) -> Result<i32> {
    let mut offset = 0;
    let mut stop = false;
    let mut acc = 0;

    // Iterate changing an instruction until it finds no loop
    while stop == false {
        let res = flip_jmp_or_nop(data, offset)?;
        offset = res.0+1;
        let new_data = res.1;
        let res = run_until_loop(&new_data)?;
        stop = !res.0;
        acc = res.1;
    }

    Ok(acc)
}

// vim:shiftwidth=4

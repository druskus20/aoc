/// Advent of Code 2025 - Day 6
use crate::{prelude::*, utils};

const USE_MOCK: bool = false;

/// Shape of the data is columnar, the last row is operators
const MOCK_DATA: &str = indoc! {"
    123 328  51 64 
    45 64  387 23 
    6 98  215 314
    *   +   *   +  
"};

type Input = Vec<String>;

/// Only separating the last line and cleaning the input.
#[aoc_generator(day6)]
pub fn input_gen(input: &str) -> Input {
    let input = if USE_MOCK {
        eprintln!("------------------");
        eprintln!("Using mock data!");
        eprintln!("------------------");
        MOCK_DATA
    } else {
        input
    };

    let input: Vec<String> = utils::clean_input(input)
        .map(|line| line.to_owned())
        .collect();

    input
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &Input) -> Result<u64> {
    let nums = &data[..data.len() - 1]; // all but last line
    let ops = {
        let last_line = &data[data.len() - 1];
        last_line
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<char>>()
    };

    //  matrix of numbers by column
    let mut num_matrix: Vec<Vec<u64>> = vec![vec![]; ops.len()];
    for row in nums {
        for (col, num) in row.split_whitespace().enumerate() {
            num_matrix[col].push(num.parse()?);
        }
    }

    let mut total_count = 0;
    for (i, col) in num_matrix.iter().enumerate() {
        let op = ops[i];
        println!("Processing column: {:?}, op: {}", col, op);
        let mut acc = if op == '+' { 0 } else { 1 };
        //
        for num in col {
            match op {
                '+' => acc += num,
                '*' => acc *= num,
                _ => panic!("Unknown operator: {}", op),
            }
            println!("{} {}", op, acc);
        }
        println!("Column result: {}", acc);
        total_count += acc;
    }

    Ok(total_count)
}

#[aoc(day6, part2)]
pub fn solve_part2(data: &Input) -> Result<u64> {
    todo!()
}

// Advent of Code 2021 - Day 11

use anyhow::Result;

pub type InputType = Vec<Vec<i32>>;
pub type OutputType = i32;

#[aoc_generator(day11)]
pub fn input_gen(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(data: &InputType) -> Result<OutputType> {
    const STEPS: i32 = 100;
    let octos = &mut data.clone();

    let mut total_flashes = 0;
    for _ in 0..STEPS {
        total_flashes += apply_step(octos);
    }
    Ok(total_flashes)
}

#[aoc(day11, part2)]
pub fn solve_part2(data: &InputType) -> Result<OutputType> {
    let octos = &mut data.clone();

    let mut step = 0;
    loop {
        step += 1;
        if apply_step(octos) == 100 {
            break;
        }
    }
    Ok(step)
}

fn apply_step(octos: &mut InputType) -> i32 {
    // Increment every octo
    for octo in octos.into_iter() {
        for i in 0..octo.len() {
            match octo[i] {
                -1 => octo[i] = 1,
                _ => octo[i] += 1,
            }
        }
    }

    pretty_print_octos(octos);
    println!();

    let mut total_flashes = 0;
    for j in 0..octos.len() {
        for i in 0..octos[j].len() {
            if octos[j][i] > 9 {
                total_flashes += flash(octos, j, i);
            }
        }
    }

    total_flashes
}

fn flash(octos: &mut InputType, j: usize, i: usize) -> i32 {
    let range_j = (j as isize - 1).max(0) as usize..(j + 2).min(octos.len());
    let range_i = (i as isize - 1).max(0) as usize..(i + 2).min(octos[0].len());

    let mut total_flashes = 1;
    octos[j][i] = -1;
    for new_j in range_j {
        for new_i in range_i.clone() {
            if octos[new_j][new_i] == -1 {
                continue;
            }
            octos[new_j][new_i] += 1;
            if octos[new_j][new_i] > 9 {
                total_flashes += flash(octos, new_j, new_i);
            }
        }
    }
    total_flashes
}

fn pretty_print_octos(octos: &InputType) {
    for octo in octos.iter() {
        for i in octo.iter() {
            if *i > 9 {
                print!("F");
            } else {
                print!("{}", i);
            }
        }
        println!();
    }
}

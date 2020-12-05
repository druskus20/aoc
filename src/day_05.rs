use anyhow::*;
use std::ops::RangeInclusive;

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn split_range(range: &RangeInclusive<usize>) -> usize {
    ((*range.end() as f32 - *range.start() as f32) / 2_f32).round() as usize
}

fn get_last_half(range: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    *range.start() + half_range(range)..=*range.end()
}

fn get_first_half(range: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    *range.start()..=*range.end() - half_range(range)
}

pub fn get_half_rows(code: char, range: &RangeInclusive<usize>) -> Result<RangeInclusive<usize>> {
    match code {
        'F' => Ok(get_first_half(range)),
        'B' => Ok(get_last_half(range)),
        _ => Err(anyhow!("Terrible mistake")),
    }
}

pub fn get_half_cols(code: char, range: &RangeInclusive<usize>) -> Result<RangeInclusive<usize>> {
    match code {
        'L' => Ok(get_first_half(range)),
        'R' => Ok(get_last_half(range)),
        _ => Err(anyhow!("This will never work :C")),
    }
}

pub fn find_row_col(code: &String) -> (usize, usize) {
    let mut row_range = 0..=127;
    let mut col_range = 0..=7;

    &code[..7]
        .chars()
        .for_each(|c| row_range = get_half_rows(c, &row_range).unwrap());
    &code[7..]
        .chars()
        .for_each(|c| col_range = get_half_cols(c, &col_range).unwrap());

    (*row_range.start(), *col_range.end())
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &Vec<String>) -> usize {
    let row_cols = data.iter().map(find_row_col).collect::<Vec<_>>();
    row_cols.iter().map(|p| p.0 * 8 + p.1).max().unwrap()
}

//#[aoc(day5, part2)]

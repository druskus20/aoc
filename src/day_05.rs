use anyhow::*;
use std::ops::RangeInclusive;
use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn split_range(range: &RangeInclusive<usize>) -> usize {
    ((*range.end() as f32 - *range.start() as f32) / 2_f32).round() as usize
}

fn get_last_half(range: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    *range.start() + split_range(range)..=*range.end()
}

fn get_first_half(range: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    *range.start()..=*range.end() - split_range(range)
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

pub fn  data_to_numeric(data: &Vec<String>) -> Vec<usize> {
    data.iter().map(find_row_col).map(|p| p.0 * 8 + p.1).collect::<Vec<_>>()
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &Vec<String>) -> usize {
    *data_to_numeric(data).iter().max().unwrap()
}

pub fn merge_seats(data: &Vec<usize>) -> Vec<usize> {
    let half = ((data.len() / 2) as f32).round() as usize;

    let mut merged_vector = Vec::<usize>::new();

    let lower_data = data[half..].to_vec();
    let mut higher_data = data[..half].to_vec();
    higher_data.reverse();

    lower_data
        .iter()
        .zip(higher_data.iter())
        .for_each(|(low_val, high_val)| merged_vector.append(&mut vec!(*low_val, *high_val)));

    merged_vector
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &Vec<String>) -> usize {

    let num_data = data_to_numeric(data);

    // Create an array of all possible seats within the range
    let possible_seats = (*num_data.iter().min().unwrap()..*num_data.iter().max().unwrap()).collect::<Vec<usize>>();

    // Create a hash of the present seats
    let num_data_hash : HashSet<usize> = HashSet::from_iter(num_data);

    // Merge the order of the possible seats
    let merged_possible_seats = merge_seats(&possible_seats);

    // Find the first missing seat (starting from the middle of the plane)
    *merged_possible_seats.iter().find(|v| !num_data_hash.contains(v)).unwrap()
}


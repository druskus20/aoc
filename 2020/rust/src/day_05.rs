use anyhow::*;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::RangeInclusive;

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

    let _ = &code[..7]
        .chars()
        .for_each(|c| row_range = get_half_rows(c, &row_range).unwrap());
    let _ = &code[7..]
        .chars()
        .for_each(|c| col_range = get_half_cols(c, &col_range).unwrap());

    (*row_range.start(), *col_range.end())
}

pub fn data_to_numeric(data: &Vec<String>) -> Vec<usize> {
    data.iter()
        .map(find_row_col)
        .map(|p| p.0 * 8 + p.1)
        .collect::<Vec<_>>()
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &Vec<String>) -> usize {
    *data_to_numeric(data).iter().max().unwrap()
}

// Orders a vector starting from the center (1 2 3 4 5 6) -> (3 4 2 5 1 6)
pub fn merge_seats(data: &Vec<usize>) -> Vec<usize> {
    let half = ((data.len() / 2) as f32).round() as usize;

    let mut merged_vector = Vec::<usize>::new();

    let lower_data = data[half..].to_vec();
    let mut higher_data = data[..half].to_vec();
    higher_data.reverse();

    lower_data
        .iter()
        .zip(higher_data.iter())
        .for_each(|(low_val, high_val)| merged_vector.append(&mut vec![*low_val, *high_val])); // This line is the worst

    merged_vector
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &Vec<String>) -> usize {
    let num_data = data_to_numeric(data);

    // Create an array of all possible seats within the range
    let possible_seats =
        (*num_data.iter().min().unwrap()..*num_data.iter().max().unwrap()).collect::<Vec<usize>>();

    // Create a hash of the present seats
    let num_data_hash: HashSet<usize> = HashSet::from_iter(num_data);

    // Merge the order of the possible seats
    let merged_possible_seats = merge_seats(&possible_seats);

    // Find the first missing seat (starting from the middle of the plane)
    *merged_possible_seats
        .iter()
        .find(|v| !num_data_hash.contains(v))
        .unwrap()
}

/*
--- Day 5: Binary Boarding ---
You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.

You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.

Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".

The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.

For example, consider just the first seven characters of FBFBBFFRLR:

Start by considering the whole range, rows 0 through 127.
F means to take the lower half, keeping rows 0 through 63.
B means to take the upper half, keeping rows 32 through 63.
F means to take the lower half, keeping rows 32 through 47.
B means to take the upper half, keeping rows 40 through 47.
B keeps rows 44 through 47.
F keeps rows 44 through 45.
The final F keeps the lower of the two, row 44.
The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.

For example, consider just the last 3 characters of FBFBBFFRLR:

Start by considering the whole range, columns 0 through 7.
R means to take the upper half, keeping columns 4 through 7.
L means to take the lower half, keeping columns 4 through 5.
The final R keeps the upper of the two, column 5.
So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.

Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.

Here are some other boarding passes:

BFFFBBFRRR: row 70, column 7, seat ID 567.
FFFBBBFRRR: row 14, column 7, seat ID 119.
BBFFBBFRLL: row 102, column 4, seat ID 820.
As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?


--- Part Two ---
Ding! The "fasten seat belt" signs have turned on. Time to find your seat.

It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.

Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.

What is the ID of your seat?
 */

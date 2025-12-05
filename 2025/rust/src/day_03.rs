/// Advent of Code 2025 - Day 3
use crate::{prelude::*, utils};

const USE_MOCK: bool = false;
const MOCK_DATA: &str = indoc! {"
    987654321111111
    811111111111119
    234234234234278
    818181911112111
"};

type Input = Vec<Vec<u64>>;

#[aoc_generator(day3)]
pub fn input_gen(input: &str) -> Input {
    let input = if USE_MOCK {
        eprintln!("------------------");
        eprintln!("Using mock data!");
        eprintln!("------------------");
        MOCK_DATA
    } else {
        input
    };

    utils::clean_input(input)
        .map(|bank| {
            bank.chars()
                .map(|c| char::to_digit(c, 10).unwrap() as u64)
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(data: &Input) -> Result<u64> {
    let mut acc = 0;

    for bank in data {
        // We always take the first two
        let mut first = bank[0];
        let mut second = bank[1];

        // then we go number by number and either:
        //  - the initial digit is "second"
        //  - the last digit is max(second, third)
        for third in &bank[2..] {
            if second > first {
                first = second;
                second = *third;
            } else {
                second = u64::max(second, *third)
            }
        }

        acc += (first * 10) + second
    }

    Ok(acc)
}

#[aoc(day3, part2)]
pub fn solve_part2(data: &Input) -> Result<u64> {
    // we always want to take the highest number possible in the left most position
    let mut acc = 0;

    // Iterate over banks
    for bank in data {
        acc += find_twelve_highest(&bank);
    }

    Ok(acc)
}

fn seq_to_u64(seq: &[u64]) -> u64 {
    seq.iter().fold(0, |acc, &d| acc * 10 + d)
}

fn find_twelve_highest(bank: &[u64]) -> u64 {
    let k = 12;
    let mut stack = Vec::with_capacity(k);

    for (i, &d) in bank.iter().enumerate() {
        let remaining = bank.len() - i;

        while let Some(&last) = stack.last() {
            if last < d && stack.len() - 1 + remaining >= k {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.len() < k {
            stack.push(d);
        }
    }

    seq_to_u64(&stack)
}

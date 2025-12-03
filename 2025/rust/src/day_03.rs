/// Advent of Code 2025 - Day 3
use crate::{prelude::*, utils};

const USE_MOCK: bool = true;
const MOCK_DATA: &str = indoc! {"
    818181911112111
"};
//    987654321111111
//    811111111111119
//    234234234234278

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

    for bank in data {
        // We always take the first twelve
        let mut seq = bank[0..12].to_vec();

        let len = bank.len();
        for i in 0..len {
            let next = bank[i];

            let len_left = len - i; // 3
            dbg!(len_left);

            // Compare next with each number in the sequence from left to right
            // if next is greater than any number, swap all the following numbers with the
            // contiguous numbers from the bank
            //
            // IMPORTANT This calculation is wrong. We should not only count the last few numbers,
            // we should start by 0 and go up to 12 - len_left
            eprintln!("Range: {}..{}", 0, len_left);
            for j in 0..u32::min(12, len_left as u32) as usize {
                let n = seq[j];

                eprintln!("Comparing next {next} with seq[{j}] = {n}, len_left = {len_left}");
                // swap numbers and fill the rest with the contiguous
                if next > n {
                    dbg!(format!("swapping {n} with {next} at pos {j}"));
                    assert!(next == bank[i]);
                    dbg!(format!("j = {j}"));
                    for k in j..12 {
                        seq[k] = bank[i + (k)];
                    }
                    break; // important
                }
            }
        }

        let seq = seq_to_u64(&seq);
        dbg!(seq);
        acc += seq;
    }

    Ok(acc)
}

fn seq_to_u64(seq: &[u64]) -> u64 {
    seq.iter().fold(0, |acc, &d| acc * 10 + d)
}

fn seq_to_string(seq: &[u64]) -> String {
    seq.iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join("")
}

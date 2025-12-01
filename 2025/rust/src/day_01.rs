// Advent of Code 2025 - Day 1

use crate::{prelude::*, utils};

const USE_MOCK: bool = false;
const MOCK_DATA: &str = indoc! { "
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
"};

#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> Vec<i32> {
    let input = if USE_MOCK {
        eprintln!("------------------");
        eprintln!("Using mock data!");
        eprintln!("------------------");
        MOCK_DATA
    } else {
        input
    };

    utils::clean_input(input)
        .map(|l| {
            let direction = match l.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!("Invalid direction"),
            };
            let amount: i32 = l[1..].parse().unwrap();
            amount * direction
        })
        .collect()
}

/// returns the new dial and how many times it wrapped
fn wrapping_rotation(dial: i32, rot: i32) -> (i32, i32) {
    let max = 100;
    let new_dial = ((dial + rot) % max + max) % max;
    let wrapped = if rot > 0 {
        (dial + rot) / max
    } else if rot < 0 {
        let t0: i32 = if dial == 0 { max } else { dial };
        if -rot >= t0 { 1 + (-rot - t0) / max } else { 0 }
    } else {
        0
    };
    (new_dial, wrapped)
}

#[cfg(test)]
mod test {
    use super::wrapping_rotation;
    #[test]
    fn test_wrapping_rotation() {
        assert_eq!(wrapping_rotation(50, 30), (80, 0));
        assert_eq!(wrapping_rotation(80, 30), (10, 1));
        assert_eq!(wrapping_rotation(10, -20), (90, 1));
        // gotcha - the rotations are not constrained to 0..99, we could have R250 for example
        assert_eq!(wrapping_rotation(90, 250), (40, 3));
        assert_eq!(wrapping_rotation(50, -200), (50, 2));
    }
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[i32]) -> Result<i32> {
    const START: i32 = 50;

    let mut c = 0;
    let mut i = START;
    for d in data {
        i = wrapping_rotation(i, *d).0;
        if i == 0 {
            c += 1;
        }
    }

    Ok(c)
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[i32]) -> Result<i32> {
    const START: i32 = 50;

    let mut c = 0;
    let mut i = START;
    for d in data {
        let (new_i, wrapped) = wrapping_rotation(i, *d);
        i = new_i;
        c += wrapped;
    }

    Ok(c)
}

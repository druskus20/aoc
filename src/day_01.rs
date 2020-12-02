// Although brute forcing your way in seems like an easy solution, its better to use
//  a hash to check if the second number exists instead of iterating through the whole
//  set of numbers twice. The second part is pretty similar to the first one and can follow the same
//  principle.

use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> HashSet<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &HashSet<u64>) -> Result<u64, &str> {
    for i in data {
        let j = 2020 - i;
        if data.contains(&j) {
            return Ok(i * j);
        }
    }
    Err("There's been a terrible problem")
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &HashSet<u64>) -> Result<u64, &str> {
    for i in data {
        for j in data {
            // This prevents overflow subtraction
            if i + j < 2020 {
                let k = 2020 - i - j;
                if data.contains(&k) {
                    return Ok(i * j * k);
                }
            }
        }
    }
    Err("There's been a terrible problem again")
}

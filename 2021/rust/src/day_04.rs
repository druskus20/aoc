// Advent of Code 2021 - Day 3
//
use colored::Colorize;
use std::str::FromStr;

use anyhow::{bail, Result};

pub type NumberOrder = Vec<i32>;

pub type InputType = (NumberOrder, Vec<BingoTable>);
pub type OutputType = i32;

#[derive(Debug, Clone, Copy)]
struct Dims {
    rows: usize,
    cols: usize,
}

#[derive(Debug, Clone)]
pub struct BingoTable {
    nums: Vec<Vec<i32>>,
    marked_nums: Vec<Vec<bool>>,
    dims: Dims,
}

impl BingoTable {
    fn pretty_print(&self) {
        for i in 0..self.dims.rows {
            for j in 0..self.dims.cols {
                let n = self.nums[i][j];
                if self.marked_nums[i][j] {
                    print!("{:2} ", n.to_string().green());
                } else {
                    print!("{:2} ", n.to_string().red());
                }
            }
            println!();
        }
        println!("\n");
    }

    fn check_marked_columns(&self) -> bool {
        let mut res = true;
        for i in 0..self.dims.cols {
            for j in 0..self.dims.rows {
                res = res && self.marked_nums[j][i];
            }
            if res {
                return res;
            }
        }
        return false;
    }

    fn check_marked_rows(&self) -> bool {
        for i in 0..self.dims.rows {
            let mut res = true;
            for j in 0..self.dims.cols {
                res = res && self.marked_nums[i][j];
            }
            if res {
                return res;
            }
        }
        return false;
    }

    fn check_win(&self) -> bool {
        let row_sum = self.check_marked_rows();
        if row_sum {
            return row_sum;
        }
        let col_sum = self.check_marked_columns();
        if col_sum {
            return col_sum;
        }
        return false;
    }

    fn mark_rowcol(&mut self, row: usize, col: usize) -> Result<()> {
        if row > self.dims.rows || col > self.dims.cols {
            bail!("Invalid row/col");
        }
        self.marked_nums[row][col] = true;
        Ok(())
    }

    fn mark_num(&mut self, n: i32) -> Result<()> {
        for i in 0..self.dims.rows {
            for j in 0..self.dims.cols {
                if self.nums[i][j] == n {
                    self.mark_rowcol(i, j)?;
                }
            }
        }
        Ok(())
    }

    fn sum_of_unmarkeds(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.dims.rows {
            for j in 0..self.dims.cols {
                if !self.marked_nums[i][j] {
                    sum += self.nums[i][j];
                }
            }
        }
        return sum;
    }
}

impl FromStr for BingoTable {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split("\n")
            .map(|r| {
                r.split(" ")
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse().unwrap())
                    .collect::<_>()
            })
            .collect::<Vec<Vec<i32>>>();

        let dims = Dims {
            rows: nums.len(),
            cols: nums[0].len(),
        };

        let marked_nums = vec![vec![false; dims.cols as usize]; dims.rows as usize];

        Ok(BingoTable {
            nums,
            dims,
            marked_nums,
        })
    }
}

#[aoc_generator(day4)]
pub fn input_gen(input: &str) -> InputType {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let number_order = data
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<NumberOrder>();
    let tables = data
        .iter()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<BingoTable>>();

    (number_order, tables)
}

#[aoc(day4, part1)]
pub fn solve_part1((order, tables): &InputType) -> Result<OutputType> {
    let mut tables = tables.clone();
    for num in order {
        dbg!(num);
        for table in &mut tables {
            table.mark_num(*num)?;
            table.pretty_print();
            let score = table.check_win();
            if score {
                dbg!(table.sum_of_unmarkeds());
                return Ok(table.sum_of_unmarkeds() * num);
            }
        }
    }
    bail!("No winning table found");
}

#[aoc(day4, part2)]
pub fn solve_part2((order, tables): &InputType) -> Result<OutputType> {
    todo!()
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_day4_part1() -> Result<()> {
//        let raw_input = "";
//        let input = input_gen(&raw_input);
//        assert_eq!(solve_part1(&input)?, 1);
//        assert_eq!(solve_part2(&input)?, 1);
//        Ok(())
//    }
//}

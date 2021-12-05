// Advent of Code 2021 - Day 5

// Aproach: calculate the points of every line. Combine the points of all lines, group them and map-count
// the number of points of each group. If any group has count > 2, its an intersection. Probably
// very unnecessary, because I was determined to use a iterators / group by.

// Fml, decided to implement my own Range-like struct so that I can loop through decreasing ranges
// easily.

use anyhow::Result;
use itertools::{zip, Itertools};
use scan_fmt::scan_fmt;

type Point = (i32, i32);
pub type InputType = Vec<(Point, Point)>;
pub type OutputType = i32;

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> InputType {
    input
        .lines()
        .map(|l| {
            let (a1, a2, b1, b2) = scan_fmt!(l, "{d},{d} -> {d},{d}", i32, i32, i32, i32).unwrap();
            ((a1, a2), (b1, b2))
        })
        .collect::<InputType>()
}

// A more convenient range. It is inclusive on both ends, it also allows to iterate in reverse.
#[derive(Debug, Clone)]
struct MyRange<T> {
    end: T,
    current: T,
    reverse: bool,
}

impl MyRange<i32> {
    fn new(start: i32, end: i32) -> Self {
        let reverse = start > end;
        let start = if reverse { start + 1 } else { start - 1 };
        let end = if reverse { end - 1 } else { end };
        MyRange {
            end,
            current: start,
            reverse,
        }
    }
}

impl Iterator for MyRange<i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reverse {
            false => {
                if self.current < self.end {
                    self.current += 1;
                    Some(self.current)
                } else {
                    None
                }
            }
            true => {
                if self.current > self.end + 1 {
                    self.current -= 1;
                    Some(self.current)
                } else {
                    None
                }
            }
        }
    }
}

fn calc_horizontal_vertical_lines(a: Point, b: Point) -> Vec<Point> {
    let mut points = Vec::new();

    if a.0 != b.0 && a.1 != b.1 {
        return vec![];
    }

    for i in MyRange::new(a.0, b.0) {
        for j in MyRange::new(a.1, b.1) {
            points.push((i, j));
        }
    }
    points
}

fn calc_diagonals(a: Point, b: Point) -> Vec<Point> {
    let mut points = Vec::new();

    if a.0 == b.0 || a.1 == b.1 {
        return vec![];
    }

    for (i, j) in zip(MyRange::new(a.0, b.0), MyRange::new(a.1, b.1)) {
        points.push((i, j));
    }
    points
}

fn calc_lines(a: Point, b: Point) -> Vec<Point> {
    if a.0 != b.0 && a.1 != b.1 {
        calc_diagonals(a, b)
    } else if a.0 == b.0 || a.1 == b.1 {
        calc_horizontal_vertical_lines(a, b)
    } else {
        vec![]
    }
}

fn calc_intersections<'a>(points: impl IntoIterator<Item = &'a (Point, Point)>) -> i32 {
    points
        .into_iter()
        .flat_map(|(x, y)| calc_lines(*x, *y))
        .sorted()
        .map(|p| p)
        .group_by(|p| *p)
        .into_iter()
        .filter_map(
            |(_, group)| {
                if group.count() >= 2 {
                    Some(true)
                } else {
                    None
                }
            },
        )
        .count() as i32
}

#[aoc(day5, part1)]
pub fn solve_part1(points: &InputType) -> Result<OutputType> {
    Ok(calc_intersections(
        // filter out diagonals
        points.iter().filter(|(a, b)| !(a.0 != b.0 && a.1 != b.1)),
    ))
}

#[aoc(day5, part2)]
pub fn solve_part2(points: &InputType) -> Result<OutputType> {
    Ok(calc_intersections(points))
}

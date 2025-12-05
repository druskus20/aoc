/// Advent of Code 2025 - Day 5
use crate::{prelude::*, utils};

const USE_MOCK: bool = false;
const MOCK_DATA: &str = indoc! {"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"};

pub struct Data {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}
type Input = Data;

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: u64,
    end: u64,
}

#[derive(Debug)]
struct Node {
    interval: Interval,
    max_end: u64,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(interval: Interval) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            max_end: interval.end,
            interval,
            left: None,
            right: None,
        }))
    }

    fn insert(root: &mut Option<Rc<RefCell<Node>>>, interval: Interval) {
        if let Some(node_rc) = root {
            let mut node = node_rc.borrow_mut();
            node.max_end = max(node.max_end, interval.end);

            if interval.start < node.interval.start {
                Node::insert(&mut node.left, interval);
            } else {
                Node::insert(&mut node.right, interval);
            }
        } else {
            *root = Some(Node::new(interval));
        }
    }

    fn contains(root: &Option<Rc<RefCell<Node>>>, x: u64) -> bool {
        if let Some(node_rc) = root {
            let node = node_rc.borrow();

            if x >= node.interval.start && x <= node.interval.end {
                return true;
            }

            if let Some(left) = &node.left
                && x <= left.borrow().max_end
            {
                return Node::contains(&node.left, x);
            }

            return Node::contains(&node.right, x);
        }

        false
    }
}

impl Node {
    /// collects all intervals in the tree into a sorted Vec<Interval>
    #[allow(dead_code)]
    fn collect_intervals(root: &Option<Rc<RefCell<Node>>>, out: &mut Vec<Interval>) {
        if let Some(node_rc) = root {
            let node = node_rc.borrow();
            // in-order traversal: left, self, right
            Node::collect_intervals(&node.left, out);
            out.push(node.interval);
            Node::collect_intervals(&node.right, out);
        }
    }

    /// collapses all the intervals in the tree and returns a vec of non overlapping intervals
    #[allow(dead_code)]
    fn collapse1(root: &Option<Rc<RefCell<Node>>>) -> Vec<Interval> {
        let mut intervals = Vec::new();
        Node::collect_intervals(root, &mut intervals);

        let mut merged: Vec<Interval> = Vec::new();
        for interval in intervals {
            if let Some(last) = merged.last_mut() {
                if interval.start <= last.end + 1 {
                    last.end = last.end.max(interval.end);
                } else {
                    merged.push(interval);
                }
            } else {
                merged.push(interval);
            }
        }

        merged
    }

    fn collapse_rec(root: &Option<Rc<RefCell<Node>>>, merged: &mut Vec<Interval>) {
        if let Some(node_rc) = root {
            let node = node_rc.borrow();

            // LEFT
            Node::collapse_rec(&node.left, merged);

            // Merge current interval
            if let Some(last) = merged.last_mut() {
                if node.interval.start <= last.end + 1 {
                    last.end = last.end.max(node.interval.end);
                } else {
                    merged.push(node.interval);
                }
            } else {
                merged.push(node.interval);
            }

            // RIGHT
            Node::collapse_rec(&node.right, merged);
        }
    }

    fn collapse2(root: &Option<Rc<RefCell<Node>>>) -> Vec<Interval> {
        let mut merged: Vec<Interval> = Vec::new();
        Node::collapse_rec(root, &mut merged);
        merged
    }
}

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> Input {
    let input = if USE_MOCK {
        eprintln!("------------------");
        eprintln!("Using mock data!");
        eprintln!("------------------");
        MOCK_DATA
    } else {
        input
    };

    let mut sections = input.split("\n\n");
    let ranges_section = sections.next().unwrap();
    let ids_section = sections.next().unwrap();

    let ranges = utils::clean_input(ranges_section)
        .map(|line| {
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();

    let ids = utils::clean_input(ids_section)
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // assert! there's no repeat ids
    let mut id_set = std::collections::HashSet::new();
    for &id in &ids {
        if !id_set.insert(id) {
            panic!("duplicate ID: {}", id);
        }
    }

    Data { ranges, ids }
}

#[aoc(day5, part1)]
pub fn _solve_part1(data: &Input) -> Result<u64> {
    let mut root: Option<Rc<RefCell<Node>>> = None;

    // Construct the tere
    for (start, end) in &data.ranges {
        Node::insert(
            &mut root,
            Interval {
                start: *start,
                end: *end,
            },
        );
    }

    // check every id
    let mut count = 0;
    for id in &data.ids {
        if Node::contains(&root, *id) {
            count += 1;
        }
    }

    Ok(count)
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &Input) -> Result<u64> {
    let mut root: Option<Rc<RefCell<Node>>> = None;

    // Construct the tere
    for (start, end) in &data.ranges {
        Node::insert(
            &mut root,
            Interval {
                start: *start,
                end: *end,
            },
        );
    }

    let merged_intervals = Node::collapse2(&root);

    let total_covered: u64 = merged_intervals
        .iter()
        .map(|interval| interval.end - interval.start + 1)
        .sum();

    Ok(total_covered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_inside_intervals() {
        let mut root = None;
        Node::insert(&mut root, Interval { start: 1, end: 5 });
        Node::insert(&mut root, Interval { start: 4, end: 10 });
        Node::insert(&mut root, Interval { start: 12, end: 15 });
        Node::insert(&mut root, Interval { start: 20, end: 25 });

        assert!(Node::contains(&root, 3)); // in [1,5]
        assert!(Node::contains(&root, 6)); // in [4,10]
        assert!(Node::contains(&root, 13)); // in [12,15]
        assert!(Node::contains(&root, 20)); // in [20,25]
        assert!(Node::contains(&root, 25)); // edge case
        //
        assert!(!Node::contains(&root, 0)); // before all intervals
        assert!(!Node::contains(&root, 11)); // between intervals
        assert!(!Node::contains(&root, 26)); // after all intervals

        assert!(Node::contains(&root, 4));
        assert!(Node::contains(&root, 5));
        assert!(Node::contains(&root, 9));
    }

    #[test]
    fn test_collapse_intervals() {
        let mut root = None;
        Node::insert(&mut root, Interval { start: 1, end: 5 });
        Node::insert(&mut root, Interval { start: 4, end: 10 });
        Node::insert(&mut root, Interval { start: 12, end: 15 });
        Node::insert(&mut root, Interval { start: 14, end: 20 });
        Node::insert(&mut root, Interval { start: 22, end: 25 });

        let merged = Node::collapse1(&root);
        let expected = [
            Interval { start: 1, end: 10 },
            Interval { start: 12, end: 20 },
            Interval { start: 22, end: 25 },
        ];

        assert_eq!(merged.len(), expected.len());
        for (m, e) in merged.iter().zip(expected.iter()) {
            assert_eq!(m.start, e.start);
            assert_eq!(m.end, e.end);
        }
        let merged = Node::collapse2(&root);
        let expected = [
            Interval { start: 1, end: 10 },
            Interval { start: 12, end: 20 },
            Interval { start: 22, end: 25 },
        ];
        assert_eq!(merged.len(), expected.len());
    }
}

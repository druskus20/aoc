/// Advent of Code 2025 - Day 3
use crate::{prelude::*, utils};

const USE_MOCK: bool = false;
const MOCK_DATA: &str = indoc! {"
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.
"};

pub struct Matrix {
    data: Box<[bool]>,
    cols: usize,
    rows: usize,
}

impl std::ops::Index<usize> for Matrix {
    type Output = [bool];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &mut self.data[start..end]
    }
}

impl Matrix {
    fn deep_clone(&self) -> Matrix {
        Matrix {
            data: self.data.clone(), // Box<[bool]> implements Clone
            cols: self.cols,
            rows: self.rows,
        }
    }
}

type Input = Matrix;

#[aoc_generator(day4)]
pub fn input_gen(input: &str) -> Input {
    let input = if USE_MOCK {
        eprintln!("------------------");
        eprintln!("Using mock data!");
        eprintln!("------------------");
        MOCK_DATA
    } else {
        input
    };

    let data: Vec<Vec<bool>> = utils::clean_input(input)
        .map(|l| l.chars().map(|c| c == '@').collect::<Vec<bool>>())
        .collect();

    Matrix {
        cols: data[0].len(),
        rows: data.len(),
        data: data.into_iter().flatten().collect::<Box<[bool]>>(),
    }
}

struct Kernel([[bool; 3]; 3]);
impl Kernel {
    // Construct a 3x3 Kernel from a huge array, at a given index
    // if the index is at the edge, fill with false
    fn from_matrix(matrix: &Matrix, index: usize) -> Kernel {
        let rows = matrix.rows;
        let cols = matrix.cols;
        let mut kernel = [[false; 3]; 3];

        #[allow(clippy::needless_range_loop)]
        for i in 0..3 {
            for j in 0..3 {
                let row = index / cols + i - 1;
                let col = index % cols + j - 1;

                if row < rows && col < cols {
                    kernel[i][j] = matrix[row][col];
                } else {
                    kernel[i][j] = false;
                }
            }
        }

        Kernel(kernel)
    }

    const fn up(&self) -> bool {
        self.0[0][1]
    }
    const fn right(&self) -> bool {
        self.0[1][2]
    }
    const fn down(&self) -> bool {
        self.0[2][1]
    }
    const fn left(&self) -> bool {
        self.0[1][0]
    }
    const fn up_right(&self) -> bool {
        self.0[0][2]
    }
    const fn down_right(&self) -> bool {
        self.0[2][2]
    }
    const fn down_left(&self) -> bool {
        self.0[2][0]
    }
    const fn up_left(&self) -> bool {
        self.0[0][0]
    }
}

/// Sums a 3x3 kernel based on the given directional flags
/// Including corners
/// 1 2 3
/// 4 C 6
/// 7 8 9
fn check_kernel(kernel: &Kernel) -> bool {
    const THRESHOLD: usize = 4;
    let mut sum = 0;

    sum += kernel.up() as usize; // 2
    sum += kernel.right() as usize; // 6
    sum += kernel.down() as usize; // 8
    sum += kernel.left() as usize; // 4
    sum += kernel.up_right() as usize; // 3
    sum += kernel.down_right() as usize; // 9
    sum += kernel.down_left() as usize; // 7
    sum += kernel.up_left() as usize; // 1

    sum < THRESHOLD
}

#[aoc(day4, part1)]
pub fn solve_part1(data: &Input) -> Result<u64> {
    let rows = data.rows;
    let cols = data.cols;

    let mut count = 0u64;
    for r in 0..rows {
        for c in 0..cols {
            if data[r][c] {
                let index = r * cols + c;
                let kernel = Kernel::from_matrix(data, index);
                if check_kernel(&kernel) {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

#[aoc(day4, part2)]
pub fn solve_part2(data: &Input) -> Result<u64> {
    let mut count = 0;
    let mut again = true;
    let mut data = data.deep_clone();
    while again {
        again = false;
        let rows = data.rows;
        let cols = data.cols;

        for r in 0..rows {
            for c in 0..cols {
                if data[r][c] {
                    let index = r * cols + c;
                    let kernel = Kernel::from_matrix(&data, index);
                    if check_kernel(&kernel) {
                        // remove this index
                        data[r][c] = false;
                        count += 1;
                        again = true;
                    }
                }
            }
        }
    }
    Ok(count)
}

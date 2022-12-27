use std::ops::{Add, AddAssign};

use aoc_driver::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Loc {
    height: i8,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    is_start: bool,
    is_end: bool,
}

impl Loc {
    fn new(ch: char) -> Self {
        let mut newloc = Self {
            height: 0,
            left: false,
            right: false,
            up: false,
            down: false,
            is_start: false,
            is_end: false,
        };

        match ch {
            'S' => {
                newloc.height = 1;
                newloc.is_start = true;
            }
            'E' => {
                newloc.height = 26;
                newloc.is_end = true;
            }
            _ => {
                if ch.is_lowercase() {
                    newloc.height = ch as i8 - 'a' as i8;
                } else if ch != '\n' {
                    panic!("Invalid character {ch} inputted!");
                }
            }
        }

        newloc
    }
}

impl<T: Add<Output = T>> Add<T> for Loc
where
    i8: Add<T>,
    i8: AddAssign<T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let mut new = self.clone();
        new.height += rhs;
        new
    }
}

// fn adj(i: usize, j: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
//     let mut results = vec![];

//     if i > 0 {
//         results.push((i - 1, j));
//     }
//     if j > 0 {
//         results.push((i, j - 1))
//     }
//     if i < height - 1 {
//         results.push((i + 1, j))
//     }
//     if j < width - 1 {
//         results.push((i, j + 1))
//     }

//     results
// }

fn solution(i: &str) -> String {
    let lines = i.lines();

    let mut grid = vec![];

    for l in lines {
        let mut newline = vec![];
        for ch in l.chars() {
            if ch == '\n' {
                continue;
            }
            newline.push(Loc::new(ch));
        }

        grid.push(newline);
    }

    // construct links
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if i > 0 {
                if grid[i - 1][j] <= (grid[i][j] + 1) {
                    grid[i][j].up = true;
                }
            }
            if j > 0 {
                if grid[i][j - 1] <= (grid[i][j] + 1) {
                    grid[i][j].left = true;
                }
            }
            if i < grid.len() - 1 {
                if grid[i + 1][j] <= (grid[i][j] + 1) {
                    grid[i][j].down = true;
                }
            }
            if j < grid[0].len() - 1 {
                if grid[i][j + 1] <= (grid[i][j] + 1) {
                    grid[i][j].right = true;
                }
            }

            // for (a, b) in adj(i, j, grid.len(), grid[0].len()) {}
        }
    }

    todo!();
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:12:1, solution).unwrap()
}

use std::{
    fmt::Display,
    io::{stdin, Read},
};

use aoc_driver::*;

#[derive(Debug, Clone, Copy)]
enum Position {
    Start,
    Visited,
    NotVisited,
}

fn solution(i: &str) -> String {
    let lines: Vec<&str> = i.lines().collect();

    let mut grid = vec![vec![Position::NotVisited; 1000]; 1000];
    // Start in bottom right
    grid[500][500] = Position::Start;
    let mut head = (500, 500);
    let mut tail = (500, 500);

    for l in lines {
        let input: Vec<&str> = l.split_ascii_whitespace().collect();
        let (dir, dist) = (
            input[0].chars().nth(0).unwrap(),
            input[1].parse::<usize>().unwrap(),
        );

        match dir {
            'U' => head.0 += dist,
            'D' => head.0 -= dist,
            'L' => head.1 -= dist,
            'R' => head.1 += dist,
            _ => panic!("Unreachable!"),
        }

        while tail.0 < head.0 - 1 {
            tail.0 += 1;
            grid[tail.0][tail.1] = Position::Visited
        }

        while tail.0 > head.0 + 1 {
            tail.0 -= 1;
            grid[tail.0][tail.1] = Position::Visited
        }

        while tail.1 < head.1 - 1 {
            tail.1 += 1;
            grid[tail.0][tail.1] = Position::Visited
        }

        while tail.1 > head.1 + 1 {
            tail.1 -= 1;
            grid[tail.0][tail.1] = Position::Visited
        }

        // print_grid(&grid);
        // let buf: &mut [u8] = &mut [0 as u8; 1000];
        // stdin().read(buf).unwrap_or_default();
    }

    let mut total = 0;
    grid.iter().for_each(|e| {
        let mut c = 0;
        e.iter().for_each(|p| match *p {
            Position::Start => c += 1,
            Position::Visited => c += 1,
            Position::NotVisited => (),
        });
        total += c;
    });

    println!("{total}");
    todo!();
    total.to_string()
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Position::Start => 's',
                Position::Visited => '#',
                Position::NotVisited => '.',
            }
        )
    }
}

fn print_grid(grid: &Vec<Vec<Position>>) {
    for l in grid.iter().rev() {
        for c in l {
            print!("{c}");
        }
        print!("\n");
    }
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:9:1, solution).unwrap()
}

use std::{
    collections::VecDeque,
    ops::{Add, AddAssign},
    vec,
};

use termion::{
    self,
    style::{Bold, NoBold, NoUnderline, Reset, Underline},
};

use aoc_driver::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Parent {
    Start,
    NotFound,
    Parent(Node),
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
struct Loc {
    height: u8,
    adj: Vec<Node>,
    // left: bool,
    // right: bool,
    // up: bool,
    // down: bool,
    is_start: bool,
    is_end: bool,
    parent: Parent,
    visited: bool,
}

impl Loc {
    fn new(ch: char) -> Self {
        let mut newloc = Self {
            height: 0,
            adj: vec![],
            // left: false,
            // right: false,
            // up: false,
            // down: false,
            is_start: false,
            is_end: false,
            parent: Parent::NotFound,
            visited: false,
        };

        match ch {
            'S' => {
                newloc.height = 0;
                newloc.is_start = true;
                newloc.parent = Parent::Start;
            }
            'E' => {
                newloc.height = 25;
                newloc.is_end = true;
            }
            _ => {
                if ch.is_lowercase() {
                    newloc.height = ch as u8 - 'a' as u8;
                } else if ch != '\n' {
                    panic!("Invalid character {ch} inputted!");
                }
            }
        }

        newloc
    }
}

impl PartialOrd for Loc {
    fn ge(&self, other: &Self) -> bool {
        self.height >= other.height
    }
    fn gt(&self, other: &Self) -> bool {
        self.height > other.height
    }
    fn le(&self, other: &Self) -> bool {
        self.height <= other.height
    }
    fn lt(&self, other: &Self) -> bool {
        self.height < other.height
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.gt(other) {
            Some(std::cmp::Ordering::Greater)
        } else if self.lt(other) {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl<T: Add<Output = T>> Add<T> for Loc
where
    u8: Add<T>,
    u8: AddAssign<T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let mut new = self.clone();
        new.height += rhs;
        new
    }
}

fn adj(pos: Node, height: usize, width: usize) -> Vec<Node> {
    let mut results = vec![];

    if pos.0 > 0 {
        results.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        results.push((pos.0, pos.1 - 1))
    }
    if pos.0 < height - 1 {
        results.push((pos.0 + 1, pos.1))
    }
    if pos.1 < width - 1 {
        results.push((pos.0, pos.1 + 1))
    }

    results
}

fn solution(i: &str) -> String {
    let lines = i.lines();

    let mut grid = vec![];

    for l in lines {
        let mut newline = vec![];
        for ch in l.chars() {
            if ch == '\n' {
                continue;
            }
            let loc = Loc::new(ch);

            newline.push(loc);
        }

        grid.push(newline);
    }

    let (mut start, mut end) = ((0, 0), (0, 0));

    // construct links
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j].is_end {
                end = (i, j);
            }
            if grid[i][j].is_start {
                start = (i, j);
            }
            for (a, b) in adj((i, j), grid.len(), grid[0].len()) {
                if grid[a][b] <= (grid[i][j].clone() + 1) {
                    grid[i][j].adj.push((a, b));
                }
            }
        }
    }

    // Do a BFS -- PART A
    let path = bfs(&mut grid, start, end);

    // PART B
    // Done by counting back from part A on output path in terminal
    // let path = bfs2(&mut grid, start, end);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let ch = ('a' as u8 + grid[i][j].height) as char;
            if path.contains(&(i, j)) {
                print!("{}{}{}{}", Underline, Bold, ch, Reset)
            } else {
                print!("{}", ch)
            }
        }
        print!("\n");
    }

    println!("Path len is {}", path.len());

    (path.len() - 1).to_string()
}

type Node = (usize, usize);

// #[derive(Debug, Clone)]
// struct BFSNode {

// }

// impl BFSNode {
//     fn new() -> Self {
//         Self {
//             parent: Parent::NotFound,
//             visited: false,
//         }
//     }
// }

fn bfs(grid: &mut Vec<Vec<Loc>>, start: Node, end: Node) -> VecDeque<Node> {
    // let (height, width) = (grid.len(), grid[0].len());

    let mut to_check = VecDeque::new();
    to_check.push_back(start);

    while !to_check.is_empty() {
        let curr = to_check.pop_front().unwrap();

        if grid[curr.0][curr.1].is_end {
            break;
        }

        for n in grid[curr.0][curr.1].adj.clone() {
            let this = &mut grid[n.0][n.1];
            if !this.visited {
                to_check.push_back(n);
                this.visited = true;
                this.parent = Parent::Parent((curr.0, curr.1));
                for d in &this.adj {
                    to_check.push_back(*d)
                }
            }

            if this.is_end {
                break;
            }
        }
    }

    // let reseen = vec![];

    let mut path = VecDeque::new();
    let mut curr = end;
    loop {
        if path.contains(&curr) {
            eprintln!("Double appearance at {:?}!", curr);
            path.pop_front();
            break;
        };
        path.push_front(curr);
        eprintln!("Added ({:?}) to the path...", curr);
        curr = match grid[curr.0][curr.1].parent {
            Parent::Parent(n) => n,
            Parent::NotFound => panic!("No route back from {:?}!", curr),
            Parent::Start => break,
        }
    }

    path
}

fn bfs2(grid: &mut Vec<Vec<Loc>>, start: Node, end: Node) -> VecDeque<Node> {
    // let (height, width) = (grid.len(), grid[0].len());

    let mut to_check = VecDeque::new();
    to_check.push_back(start);

    while !to_check.is_empty() {
        let curr = to_check.pop_front().unwrap();

        if grid[curr.0][curr.1].is_end {
            break;
        }

        for n in grid[curr.0][curr.1].adj.clone() {
            let this = &mut grid[n.0][n.1];
            if !this.visited {
                to_check.push_back(n);
                this.visited = true;
                this.parent = Parent::Parent((curr.0, curr.1));
                for d in &this.adj {
                    to_check.push_back(*d)
                }
            }

            if this.is_end {
                break;
            }
        }
    }

    // let reseen = vec![];

    let mut path = VecDeque::new();
    let mut curr = end;
    loop {
        if path.contains(&curr) {
            eprintln!("Double appearance at {:?}!", curr);
            path.pop_front();
            break;
        };
        path.push_front(curr);
        eprintln!("Added ({:?}) to the path...", curr);
        curr = match grid[curr.0][curr.1].parent {
            Parent::Parent(n) => n,
            Parent::NotFound => panic!("No route back from {:?}!", curr),
            Parent::Start => break,
        }
    }

    path
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:12:2, solution).unwrap()
}

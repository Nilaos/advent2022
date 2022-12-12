use std::collections::VecDeque;

use aoc_driver::*;

fn solution(i: &str) -> String {
    let lines: Vec<&str> = i.split('\n').collect();

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

    for l in &lines[0..8] {
        let mut v = vec![];
        let mut cur = *l;
        while !cur.is_empty() {
            let (chunk, rest) = cur.split_at(std::cmp::min(4, cur.len()));
            v.push(chunk.to_string());
            cur = rest;
        }

        // println!("{:?}", v)
        for i in 0..stacks.len() {
            let str = v.get(i).unwrap();
            if str.contains("[") {
                // Lazyness yes
                let c = str.chars().nth(1).unwrap();
                stacks.get_mut(i).unwrap().push_front(c);
            }
        }
    }

    for l in &lines[10..] {
        let instruction: Vec<&str> = l.split_ascii_whitespace().collect();

        let num = instruction[1].parse::<usize>().unwrap();
        let src = instruction[3].parse::<usize>().unwrap() - 1;
        let dst = instruction[5].parse::<usize>().unwrap() - 1;
        // Ended up just modifying this one rather than writing a second one today
        let mut crates = VecDeque::new();
        for _i in 0..num {
            let data: char = stacks[src].pop_back().unwrap();
            crates.push_front(data);
        }

        for _i in 0..num {
            let data = crates.pop_front().unwrap();
            stacks[dst].push_back(data);
        }
    }

    println!("{:?}", stacks);

    let mut str = String::new();
    for s in stacks {
        match s.back() {
            Some(c) => str.push(*c),
            None => (),
        }
    }
    println!("{str}");
    // panic!();

    str
}

// fn solution2(i: &str) -> String {
//     let mut score = 0;

//     let lines = i.split('\n');

//     println!("{score}");
//     // panic!();
//     score.to_string()
// }

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:5:2, solution).unwrap()
}

use std::collections::VecDeque;

use aoc_driver::*;

fn solution(i: &str) -> String {
    let mut buf = VecDeque::new();
    let mut pos = 0;

    for c in i.chars() {
        if buf.len() > 14 {
            buf.pop_front();

            let mut bk = true;
            for ch in &buf {
                if buf.count(ch) != 1 {
                    bk = false
                }
            }

            if bk {
                break;
            }
        }
        pos += 1;
        buf.push_back(c);
    }

    println!("{pos}");
    // panic!();
    pos.to_string()
}

trait Count<T: Eq> {
    fn count(&self, item: &T) -> usize;
}

impl<T: Eq> Count<T> for VecDeque<T> {
    fn count(&self, item: &T) -> usize {
        let mut count = 0;
        for it in self.iter() {
            if *it == *item {
                count += 1;
            }
        }
        count
    }
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
    aoc_magic!(&session, 2022:6:2, solution).unwrap()
}

use aoc_driver::*;

fn solution(i: &str) -> String {
    // let mut v = vec![0];
    // let mut idx = 0;
    let _score = 0;

    let _lines = i.split('\n');

    todo!();
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:1:1, solution).unwrap()
}

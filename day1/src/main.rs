use aoc_driver::*;

fn solution(i: &str) -> String {
    unimplemented!()
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:1:1, solution).unwrap()
}

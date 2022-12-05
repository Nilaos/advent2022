use aoc_driver::*;

// enum Play {
//     Rock,
//     Paper,
//     Scissors,
// }

// enum Result {
//     Win,
//     Lose,
//     Draw,
// }

// use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Play {
    oppt: char,
    act: char,
}

fn solution(i: &str) -> String {
    // let mut v = vec![0];
    // let mut idx = 0;
    let mut score = 0;

    for ln in i.split('\n') {
        let (opp, result) = (ln.chars().nth(0).unwrap(), ln.chars().nth(3).unwrap());

        score += match opp {
            'A' => match result {
                'X' => 0,
                'Y' => 0,
                'Z' => 0,
                _ => panic!(),
            },
            'B' => match result {
                'X' => 0,
                'Y' => 0,
                'Z' => 0,
                _ => panic!(),
            },
            'C' => match result {
                'X' => 0,
                'Y' => 0,
                'Z' => 0,
                _ => panic!(),
            },
            _ => panic!("Error!"),
        }
    }
    score.to_string()
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:1:1, solution).unwrap()
}

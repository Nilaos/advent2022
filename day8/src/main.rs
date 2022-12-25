use std::fs;

use aoc_driver::*;

fn solution(i: &str) -> String {
    // let alt_input = fs::read_to_string("inputs/2022/sample.txt").unwrap();

    let lines: Vec<&str> = i.lines().collect();
    // let lines: Vec<&str> = alt_input.lines().collect();

    // println!("{:?}", lines);

    let mut forest: Vec<Vec<i32>> = vec![];
    for l in lines {
        let mut new_ln = vec![];
        for c in l.chars() {
            new_ln.push(c.to_digit(10).unwrap() as i32);
        }
        forest.push(new_ln);
    }

    let mut visible = vec![vec![false; forest[0].len()]; forest.len()];
    // println!("{:?}", visible);

    for i in 0..forest.len() {
        let mut max_ht = -1;
        for j in 0..forest[0].len() {
            if forest[i][j] > max_ht {
                max_ht = forest[i][j];
                visible[i][j] = true;
            }
        }
    }
    // println!("{:?}", visible);

    for i in 0..forest.len() {
        let mut max_ht = -1;
        for j in 0..forest[0].len() {
            if forest[j][i] > max_ht {
                max_ht = forest[j][i];
                visible[j][i] = true;
            }
        }
    }
    // println!("{:?}", visible);

    for i in (0..forest.len()).rev() {
        let mut max_ht = -1;
        for j in (0..forest[0].len()).rev() {
            if forest[i][j] > max_ht {
                max_ht = forest[i][j];
                visible[i][j] = true;
            }
            print!("({i},{j}): {}  ", visible[i][j]);
        }
        println!();
    }
    // println!("{:?}", visible);

    for i in (0..forest.len()).rev() {
        let mut max_ht = -1;
        for j in (0..forest[0].len()).rev() {
            if forest[j][i] > max_ht {
                max_ht = forest[j][i];
                visible[j][i] = true;
            }
        }
    }
    // println!("{:?}", visible);

    let mut count = 0;
    for l in &visible {
        for v in l {
            if *v {
                count += 1;
            }
        }
    }

    println!("{count}");
    // todo!();
    count.to_string()
}

fn solution2(i: &str) -> String {
    // let alt_input = fs::read_to_string("inputs/2022/sample.txt").unwrap();

    let lines: Vec<&str> = i.lines().collect();
    // let lines: Vec<&str> = alt_input.lines().collect();

    // println!("{:?}", lines);

    let mut forest: Vec<Vec<i32>> = vec![];
    for l in lines {
        let mut new_ln = vec![];
        for c in l.chars() {
            new_ln.push(c.to_digit(10).unwrap() as i32);
        }
        forest.push(new_ln);
    }

    // let mut visible = vec![vec![false; forest[0].len()]; forest.len()];
    // println!("{:?}", visible);

    let mut score = 0;

    for i in 0..(forest.len()) {
        for j in 0..(forest[0].len()) {
            let mut loc_score = 1;

            let this_ht = forest[i][j];
            print!("Tree {i},{j}:{this_ht}: (");

            let mut row_mod: i32 = i as i32 - 1;
            let mut up_score = 0;
            while row_mod >= 0 {
                up_score += 1;
                if forest[row_mod as usize][j] >= this_ht {
                    break;
                }
                row_mod -= 1;
            }
            print!("{up_score},");
            loc_score *= up_score;

            let mut row_mod: i32 = i as i32 + 1;
            let mut down_score = 0;
            while row_mod < forest.len() as i32 {
                down_score += 1;
                if forest[row_mod as usize][j] >= this_ht {
                    break;
                }
                row_mod += 1;
            }
            print!("{down_score},");
            loc_score *= down_score;

            let mut col_mod: i32 = j as i32 - 1;
            let mut left_score = 0;
            while col_mod >= 0 {
                left_score += 1;
                if forest[i][col_mod as usize] >= this_ht {
                    break;
                }
                col_mod -= 1;
            }
            print!("{left_score},");
            loc_score *= left_score;

            let mut col_mod: i32 = j as i32 + 1;
            let mut right_score = 0;
            while col_mod < forest[0].len() as i32 {
                right_score += 1;
                if forest[i][col_mod as usize] >= this_ht {
                    break;
                }
                col_mod += 1;
            }
            loc_score *= right_score;
            println!("{right_score}) : {loc_score}");

            if loc_score > score {
                println!("New Max of {loc_score} ({up_score},{down_score},{left_score},{right_score}) replacing {score} at {i},{j}: {this_ht}");
                score = loc_score
            }
        }
    }

    println!("{score}");
    todo!();
    score.to_string()
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:8:2, solution2).unwrap()
}

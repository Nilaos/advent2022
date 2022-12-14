use std::ops::RangeInclusive;

use aoc_driver::*;

fn solution(i: &str) -> String {
    // let mut v = vec![0];
    // let mut idx = 0;
    let mut score = 0;

    let lines = i.split('\n');

    for l in lines {
        let parts = l.split(&[',', '-'][..]).collect::<Vec<&str>>();

        let seta = RangeInclusive::new(
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap(),
        );
        let setb = RangeInclusive::new(
            parts[2].parse::<i32>().unwrap(),
            parts[3].parse::<i32>().unwrap(),
        );

        if seta.contains(setb.start()) && seta.contains(setb.end()) {
            score += 1;
            continue;
        }

        if setb.contains(seta.start()) && setb.contains(seta.start()) {
            score += 1;
            continue;
        }
    }

    println!("{score}");
    // panic!();
    score.to_string()
}

fn solution2(i: &str) -> String {
    // let mut v = vec![0];
    // let mut idx = 0;
    let mut score = 0;

    let lines = i.split('\n');

    for l in lines {
        let parts = l.split(&[',', '-'][..]).collect::<Vec<&str>>();

        // println!("{:?}", parts);

        let seta = RangeInclusive::new(
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap(),
        );
        let setb = RangeInclusive::new(
            parts[2].parse::<i32>().unwrap(),
            parts[3].parse::<i32>().unwrap(),
        );

        // if seta.any(|v| setb.contains(&v)) {
        //     score += 1;
        //     continue;
        // }

        if seta.contains(setb.start()) || seta.contains(setb.end()) {
            score += 1;
            continue;
        }

        if setb.contains(seta.start()) || setb.contains(seta.start()) {
            score += 1;
            continue;
        }
        // panic!();

        // if parts[0].parse::<i32>().unwrap() <= parts[2].parse::<i32>().unwrap()
        //     && parts[1].parse::<i32>().unwrap() >= parts[3].parse::<i32>().unwrap()
        // {
        //     score += 1;
        //     continue;
        // }

        // if parts[0].parse::<i32>().unwrap() >= parts[2].parse::<i32>().unwrap()
        //     && parts[1].parse::<i32>().unwrap() <= parts[3].parse::<i32>().unwrap()
        // {
        //     score += 1;
        //     continue;
        // }

        // if parts[0].parse::<i32>().unwrap() >= parts[2].parse::<i32>().unwrap()
        //     && parts[0].parse::<i32>().unwrap() <= parts[3].parse::<i32>().unwrap()
        // {
        //     score += 1;
        //     continue;
        // }

        // if parts[1].parse::<i32>().unwrap() >= parts[2].parse::<i32>().unwrap()
        //     && parts[1].parse::<i32>().unwrap() <= parts[3].parse::<i32>().unwrap()
        // {
        //     score += 1;
        //     continue;
        // }
    }

    println!("{score}");
    // panic!();
    score.to_string()
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:4:2, solution2).unwrap()
}

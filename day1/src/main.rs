use aoc_driver::*;

fn solution(i: &str) -> String {
    let mut v = vec![0];
    let mut idx = 0;

    for ln in i.split('\n') {
        if ln.len() <= 1 {
            v.push(0);
            idx += 1;
        } else {
            v[idx] += ln.parse::<i32>().unwrap();
        }
    }
    v.sort();

    // P1
    // println!("{}", v.last().unwrap());
    // v.last().unwrap().to_string()

    // P2
    let l = v.len();
    println!("{}", v[l - 3..l].iter().sum::<i32>());
    v[l - 3..l].iter().sum::<i32>().to_string()
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:1:1, solution).unwrap()
}

use std::cell::RefCell;

use aoc_driver::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Val {
    Old,
    N(u128),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Inst {
    Plus,
    Times,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MonkeyBusiness {
    act: Inst,
    v1: Val,
    v2: Val,
}

impl MonkeyBusiness {
    fn go(&self, old: u128) -> u128 {
        let a0 = match &self.v1 {
            Val::Old => &old,
            Val::N(n) => &n,
        };

        let a1 = match &self.v2 {
            Val::Old => &old,
            Val::N(n) => &n,
        };

        match self.act {
            Inst::Plus => a0 + a1,
            Inst::Times => a0 * a1,
        }
    }
}

#[derive(Debug, Eq, Ord)]
struct Monkey {
    items: Vec<u128>,
    operation: MonkeyBusiness,
    test_div: u128,
    true_to: usize,
    false_to: usize,
    activity: i32,
}

impl Monkey {
    fn new(i: &str) -> Self {
        let lines: Vec<&str> = i.lines().collect();

        let calc: Vec<&str> = lines[2].split("new =").collect::<Vec<&str>>()[1]
            .split_ascii_whitespace()
            .collect();

        let first = if "old" == calc[0] {
            Val::Old
        } else {
            Val::N(calc[0].parse().unwrap())
        };

        let oper = if "+" == calc[1] {
            Inst::Plus
        } else if "*" == calc[1] {
            Inst::Times
        } else {
            panic!("Unreachable!")
        };

        let second = if "old" == calc[2] {
            Val::Old
        } else {
            Val::N(calc[2].parse().unwrap())
        };

        Monkey {
            items: lines[1].split(": ").collect::<Vec<&str>>()[1]
                .split_ascii_whitespace()
                .map(|i| i.trim_end_matches(",").parse::<u128>().unwrap())
                .collect(),
            operation: MonkeyBusiness {
                act: oper,
                v1: first,
                v2: second,
            },
            test_div: lines[3].split("divisible by ").collect::<Vec<&str>>()[1]
                .parse()
                .unwrap(),
            true_to: lines[4].split("monkey ").collect::<Vec<&str>>()[1]
                .parse()
                .unwrap(),
            false_to: lines[5].split("monkey ").collect::<Vec<&str>>()[1]
                .parse()
                .unwrap(),
            activity: 0,
        }
    }

    fn recv(&mut self, item: u128) {
        self.items.push(item);
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.activity == other.activity
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.activity.partial_cmp(&other.activity)
    }
}

fn solution(i: &str) -> String {
    let monkeys: Vec<RefCell<Monkey>> = i
        .split("\n\n")
        .map(|m| RefCell::new(Monkey::new(m)))
        .collect();

    let worry_div = 3;

    for _turn in 0..20 {
        for m in &monkeys {
            let mut this_monkey = m.borrow_mut();
            while !this_monkey.items.is_empty() {
                let old = this_monkey.items.pop().unwrap();
                let new_worry = this_monkey.operation.go(old) / worry_div;
                if (new_worry % this_monkey.test_div) == 0 {
                    monkeys[this_monkey.true_to].borrow_mut().recv(new_worry);
                } else {
                    monkeys[this_monkey.false_to].borrow_mut().recv(new_worry);
                }
                this_monkey.activity += 1;
            }
        }
    }

    for m in monkeys {
        println!("Monkey inspected items {} times.", m.borrow().activity);
    }

    todo!();
}

fn vec_lcm(v: Vec<u128>) -> u128 {
    let mut lcm = v[0];

    for i in 1..v.len() {
        lcm = num::integer::lcm(lcm, v[i]);
    }
    lcm
}

fn solution2(i: &str) -> String {
    let mut monkeys: Vec<RefCell<Monkey>> = i
        .split("\n\n")
        .map(|m| RefCell::new(Monkey::new(m)))
        .collect();

    let mut tests = vec![];
    for m in &monkeys {
        tests.push(m.borrow().test_div);
    }

    // get LCM of tests and do modulo that
    let lcm = vec_lcm(tests);

    // let worry_div = 3;

    for _turn in 0..10000 {
        for m in &monkeys {
            let mut this_monkey = m.borrow_mut();
            while !this_monkey.items.is_empty() {
                let old = this_monkey.items.pop().unwrap();
                let new_worry = this_monkey.operation.go(old) % lcm;
                if (&new_worry % &this_monkey.test_div) == 0 {
                    monkeys[this_monkey.true_to].borrow_mut().recv(new_worry);
                } else {
                    monkeys[this_monkey.false_to].borrow_mut().recv(new_worry);
                }
                this_monkey.activity += 1;
            }
        }
    }

    monkeys.sort();

    for m in monkeys {
        println!("Monkey inspected items {} times.", m.borrow().activity);
    }

    todo!();
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:11:2, solution2).unwrap()
}

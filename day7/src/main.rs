use std::{
    collections::HashMap,
    fs::{self, File},
    rc::Rc,
    sync::RwLock,
};

use aoc_driver::*;

#[derive(Debug, Clone)]
struct TrNode {
    _name: String,
    files: HashMap<String, usize>,
    dirs: HashMap<String, Rc<Box<RwLock<TrNode>>>>,
    parent: Option<Rc<Box<RwLock<TrNode>>>>,
}

// enum State {
//     ReadingDir,
//     ChDir,
// }
// sum of all directories <= 100000 in size

fn solution(i: &str) -> String {
    let alt_input = fs::read_to_string("inputs/2022/sample.txt").unwrap();

    // let lines: Vec<&str> = i.split("\n").collect();
    let lines: Vec<&str> = alt_input.lines().collect();

    let fs = Rc::new(Box::new(RwLock::new(TrNode {
        _name: "/".to_string(),
        files: HashMap::new(),
        dirs: HashMap::new(),
        parent: None,
    })));

    let mut curr = (&fs).clone();
    // let mut curr_st = State::ReadingDir;

    for l in lines.iter().skip(1) {
        println!("Parsing {l}...");
        let line: Vec<&str> = l.split_whitespace().collect();

        if line[0].contains('$') {
            if line[1].contains("cd") {
                if line[2].contains("..") {
                    println!("Moving back up to {}", curr.read().unwrap()._name);
                    let tmp = curr.clone();
                    curr = match &tmp.read().unwrap().parent {
                        Some(s) => s.clone(),
                        None => panic!(),
                    };
                } else {
                    println!("Moving to {}", line[2]);
                    let tmp = curr.clone();
                    curr = tmp.read().unwrap().dirs.get(line[2]).unwrap().clone();
                }
            } else if line[1].contains("ls") {
                println!("Printing contents of {}", curr.read().unwrap()._name);
                continue;
            }
        } else {
            if line[0].contains("dir") {
                println!("Adding new directory {}", line[1]);
                let new_node = TrNode {
                    _name: line[1].to_string(),
                    files: HashMap::new(),
                    dirs: HashMap::new(),
                    parent: Some(curr.clone()),
                };
                curr.write().unwrap().dirs.insert(
                    line[1].to_string(),
                    Rc::new(Box::new(RwLock::new(new_node))),
                );
            } else {
                println!("Adding new file {}", line[1]);
                curr.write()
                    .unwrap()
                    .files
                    .insert(line[1].to_string(), line[0].parse().unwrap());
            }
        }
    }

    let sz = fs.read().unwrap().calculate_size();

    println!("{sz}");
    // todo!();
    sz.to_string()
}

trait CalcSize {
    fn calculate_size(&self) -> usize;
}

impl CalcSize for TrNode {
    fn calculate_size(&self) -> usize {
        let mut sz = 0;
        for (_f, s) in &self.files {
            sz += s;
        }

        // let mut szs = vec![];
        let mut subdir_sz = 0;
        for (_f, d) in &self.dirs {
            let size = d.read().unwrap().calculate_size();
            sz += size;
            if size <= 100000 {
                subdir_sz += size;
            }
        }
        if sz > 100000 {
            sz = 0;
        }
        sz + subdir_sz
    }
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:7:1, solution).unwrap()
}

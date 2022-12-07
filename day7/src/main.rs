use std::{collections::HashMap, fs, rc::Rc, sync::RwLock};

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

const MIN: usize = 44274331 - (70000000 - 30000000);

fn solution(i: &str) -> String {
    // let alt_input = fs::read_to_string("inputs/2022/sample.txt").unwrap();

    let lines: Vec<&str> = i.split("\n").collect();
    // let lines: Vec<&str> = alt_input.lines().collect();

    let fs = Rc::new(Box::new(RwLock::new(TrNode {
        _name: "/".to_string(),
        files: HashMap::new(),
        dirs: HashMap::new(),
        parent: None,
    })));

    let mut curr = (&fs).clone();
    // let mut curr_st = State::ReadingDir;

    for l in lines.iter().skip(1) {
        // println!("Parsing {l}...");
        let line: Vec<&str> = l.split_whitespace().collect();

        if line[0].contains('$') {
            if line[1].contains("cd") {
                if line[2].contains("..") {
                    // println!("Moving back up to {}", curr.read().unwrap()._name);
                    let tmp = curr.clone();
                    curr = match &tmp.read().unwrap().parent {
                        Some(s) => s.clone(),
                        None => panic!(),
                    };
                } else {
                    // println!("Moving to {}", line[2]);
                    let tmp = curr.clone();
                    curr = tmp.read().unwrap().dirs.get(line[2]).unwrap().clone();
                }
            } else if line[1].contains("ls") {
                // println!("Printing contents of {}", curr.read().unwrap()._name);
                continue;
            }
        } else {
            if line[0].contains("dir") {
                // println!("Adding new directory {}", line[1]);
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
                // println!("Adding new file {}", line[1]);
                curr.write()
                    .unwrap()
                    .files
                    .insert(line[1].to_string(), line[0].parse().unwrap());
            }
        }
    }

    let total = Rc::new(RwLock::new(0));
    let min = Rc::new(RwLock::new(44274331));

    let sz = fs
        .read()
        .unwrap()
        .calculate_size(total.clone(), min.clone());

    println!("{sz} {}, {}", total.read().unwrap(), min.read().unwrap());
    // todo!();
    let res = min.read().unwrap();
    (*res).to_string()
}

trait CalcSize {
    fn calculate_size(&self, count: Rc<RwLock<usize>>, min_sz: Rc<RwLock<usize>>) -> usize;
}

impl CalcSize for TrNode {
    fn calculate_size(&self, count: Rc<RwLock<usize>>, min_sz: Rc<RwLock<usize>>) -> usize {
        let mut sz = 0;
        // print!("{} contains files: ", self._name);
        for (_f, s) in &self.files {
            // if sz <= 100000 {
            sz += s;
            // }
            // print!("{_f}:{s} ");
        }
        // print!("\n");

        let mut szs = vec![];
        // let mut subdir_sz = 0;
        for (_f, d) in &self.dirs {
            let size = d
                .read()
                .unwrap()
                .calculate_size(count.clone(), min_sz.clone());
            szs.push(size);
            // if size <= 100000 {
            sz += size;
            // }
            // subdir_sz += size;
        }

        print!("{} contains files: ", self._name);
        for (_f, s) in &self.files {
            print!("{_f}:{s} ");
        }
        print!("\n");
        println!(
            "{} is of size {} with subdirectories of ({:?} {:?})",
            self._name,
            sz,
            // subdir_sz,
            &self.dirs.keys(),
            szs
        );

        if sz <= 100000 {
            let mut guard = count.write().unwrap();
            *guard += sz;
        }

        let mut guard = min_sz.write().unwrap();
        if sz > MIN && sz < *guard {
            *guard = sz;
        }

        sz
    }
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:7:2, solution).unwrap()
}

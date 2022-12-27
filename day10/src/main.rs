use aoc_driver::*;

#[derive(Debug)]
enum Instr {
    Addx(i32),
    Noop,
}

// struct AddxCallback {
//     deadline: i32,
//     action: i32,
// }

impl TryFrom<&str> for Instr {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let elems: Vec<&str> = value.split_ascii_whitespace().collect();
        match elems[0] {
            "addx" => Ok(Instr::Addx(elems[1].parse().unwrap())),
            "noop" => Ok(Instr::Noop),
            _ => Err("Unable to parse ".to_string() + elems[0]),
        }
    }
}

fn solution(i: &str) -> String {
    let inst: Vec<Instr> = i.lines().map(|it| it.try_into().unwrap()).collect();

    let mut x = 1;
    let mut it = 1;
    let mut sum = 0;

    // let mut callbacks = VecDeque::new();

    for i in inst {
        sum += add_if_match(it, &x);

        if it > 215 && it < 224 {
            println!("{it}: {:?} @ {x}", i);
        }

        match i {
            Instr::Addx(c) => {
                it += 1;
                sum += add_if_match(it, &x);
                it += 1;
                x += c;
            }
            Instr::Noop => {
                it += 1;
            }
        }

        // while match callbacks.front() {
        //     Some(cb) => cb.deadline <= it,
        //     None => false,
        // } {
        //     match callbacks.pop_front() {
        //         Some(c) => x += c.action,
        //         None => (),
        //     }
        // }
        // it += 1;
    }

    println!("Final: {x} in {it} cycles");
    println!("Sum: {sum}");
    // todo!();
    sum.to_string()
}

fn add_if_match(it: i32, x: &i32) -> i32 {
    match it {
        20 => {
            println!("20th: {} @ {x}", 20 * x);
            20 * x
        }
        60 => {
            println!("60th: {} @ {x}", 60 * x);
            60 * x
        }
        100 => {
            println!("100th: {} @ {x}", 100 * x);
            100 * x
        }
        140 => {
            println!("140th: {} @ {x}", 140 * x);
            140 * x
        }
        180 => {
            println!("180th: {} @ {x}", 180 * x);
            180 * x
        }
        220 => {
            println!("220th: {} @ {x}", 220 * x);
            220 * x
        }
        _ => 0,
    }
}

fn solution2(i: &str) -> String {
    let inst: Vec<Instr> = i.lines().map(|it| it.try_into().unwrap()).collect();

    let mut x = 1;
    let mut it = 1;
    // let mut sum = 0;

    let mut screen: [[char; 41]; 6] = [['.'; 41]; 6];

    for i in inst {
        // sum += add_if_match(it, &x);
        if it >= 240 {
            break;
        }
        draw_px(&mut screen, it, x);

        // if it > 215 && it < 224 {
        //     println!("{it}: {:?} @ {x}", i);
        // }

        match i {
            Instr::Addx(c) => {
                it += 1;
                // sum += add_if_match(it, &x);
                draw_px(&mut screen, it, x);
                it += 1;
                x += c;
            }
            Instr::Noop => {
                it += 1;
            }
        }

        // while match callbacks.front() {
        //     Some(cb) => cb.deadline <= it,
        //     None => false,
        // } {
        //     match callbacks.pop_front() {
        //         Some(c) => x += c.action,
        //         None => (),
        //     }
        // }
        // it += 1;
    }

    // println!("Final: {x} in {it} cycles");
    // println!("Sum: {sum}");
    for row in screen.iter() {
        println!("{}", row.iter().collect::<String>())
    }
    panic!();
    // sum.to_string()
}

fn draw_px(screen: &mut [[char; 41]; 6], it: usize, x: i32) {
    let row = (it - 1) / 40;
    let col = (it - 1) % 40;

    // println!("{row}:{col}");
    if it < 25 {
        println!("Drawing pixel in position {col} @ {x}")
    }
    if x >= (col as i32 - 1) && x <= (col as i32 + 1) {
        screen[row][col] = '#';
    }
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(&session, 2022:10:2, solution2).unwrap()
}

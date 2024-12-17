pub mod boilerplate;

use std::collections::VecDeque;

fn main() {
    let raw_data = crate::boilerplate::get_sample_if_no_input();
    if let Err(ref problem) = raw_data {
        println!("Could not read input data: {:?}", problem);
        return;
    }
    let data = raw_data.unwrap();
    part_1(&data);
    // part_2(&data);
}

fn part_1(data: &str) {
    let (a, b, c) = parse_initial_state(data);
    let program = parse_program_from(data);
    let computer_state = ThreeBitComputer {
        reg_A: a,
        reg_B: b,
        reg_C: c
    };

    println!("{:?}", computer_state);
    println!("{:?}", program);
}

fn parse_initial_state(data: &str) -> (RegisterInteger, RegisterInteger, RegisterInteger) {
    let mut line_iter = data.lines().take_while(|line| !line.is_empty()).take(3);
    let a: RegisterInteger = line_iter.next().unwrap().split(":").nth(1).unwrap().trim().parse().unwrap();
    let b: RegisterInteger = line_iter.next().unwrap().split(":").nth(1).unwrap().trim().parse().unwrap();
    let c: RegisterInteger = line_iter.next().unwrap().split(":").nth(1).unwrap().trim().parse().unwrap();
    (a, b, c)
}

fn parse_program_from(data: &str) -> VecDeque<OpCode> {
    let program_line = data.lines().skip_while(|line| !line.starts_with("Program")).nth(0).unwrap();
    println!("{:?}", program_line);
    let ops: Vec<OpCode> = program_line.split(": ").nth(1).unwrap().chars().filter_map(|c| {
        match c {
            ',' => None,
            op => {
                let n: OpCode = op.to_string().parse().expect("Bad op code input");
                Some(n)
            }
        }
    }).collect();
    VecDeque::from(ops)
}

type OpCode = u64;
type RegisterInteger = i64;

#[derive(Debug)]
struct ThreeBitComputer {
    reg_A: RegisterInteger,
    reg_B: RegisterInteger,
    reg_C: RegisterInteger
}
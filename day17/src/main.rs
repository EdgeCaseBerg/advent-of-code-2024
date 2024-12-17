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
    let mut computer_state = ThreeBitComputer {
        reg_a: a,
        reg_b: b,
        reg_c: c
    };

    let mut instruction_pointer = 0;
    loop {
        if instruction_pointer >= program.len() {
            break;
        }

        let (instruction, literal_operand) = program[instruction_pointer];
        let output = computer_state.do_instruction(instruction, literal_operand);
        if let Some(output) = output {
            println!("OUTPUT: {:?}", output);
        }

        // Since we have tuples, this is not += 2, but just += 1.
        instruction_pointer += 1;
    }

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

fn parse_program_from(data: &str) -> VecDeque<(Instruction, Operand)> {
    let program_line = data.lines().skip_while(|line| !line.starts_with("Program")).nth(0).unwrap();
    let ops: Vec<usize> = program_line.split(": ").nth(1).unwrap().chars().filter_map(|c| {
        match c {
            ',' => None,
            op => {
                let n: usize = op.to_string().parse().expect("Bad op code input");
                Some(n)
            }
        }
    }).collect();
    let mut program = VecDeque::new();
    let mut ops = ops.iter();
    loop {
        let i = ops.next();
        let o = ops.next();
        if let (Some(instruction), Some(operand)) = (i, o) {
            let parsed_i = Instruction::from(*instruction);
            program.push_back((parsed_i, *operand as Operand));
        } else {
            break;
        }
    }

    program
}

type Operand = u64;
type RegisterInteger = i64;

#[derive(Debug)]
struct ThreeBitComputer {
    reg_a: RegisterInteger,
    reg_b: RegisterInteger,
    reg_c: RegisterInteger
}

impl ThreeBitComputer {
    fn do_instruction(&mut self, instruction: Instruction, operand: Operand) -> Option<String> {
        None
    }

    fn get_combo_operand(&self, operand: Operand) -> RegisterInteger {
        match operand {
            0..=3 => operand as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("7 is a reserved operand and should not appear in a valid program"),
            other => panic!("{} is not a known operand", operand),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl Instruction {
    fn from(raw: usize) -> Instruction {
        let i = vec![
            Instruction::ADV,
            Instruction::BXL,
            Instruction::BST,
            Instruction::JNZ,
            Instruction::BXC,
            Instruction::OUT,
            Instruction::BDV,
            Instruction::CDV
        ];
        i[raw]
    }
}

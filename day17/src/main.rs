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
        reg_c: c,
        instruction_pointer: 0
    };

    println!("{:?}", computer_state);
    println!("{:?}", program);

    let mut temp_too_long = 0;

    loop {
        // Temp because the sample contains a JNZ 0 which loops the program which is
        // detrimental to my quick checking right now
        temp_too_long += 1;
        if computer_state.instruction_pointer >= program.len() || temp_too_long > 10 {
            break;
        }

        let (instruction, literal_operand) = program[computer_state.instruction_pointer];
        let output = computer_state.do_instruction(instruction, literal_operand);
        if let Some(output) = output {
            println!("OUTPUT: {:?}", output);
        }

        // Since we have tuples, this is not += 2, but just += 1.
        // Though we'll probably need to tweak this for jump commands and the like
        computer_state.instruction_pointer += 1;
    }

    println!("{:?}", computer_state);

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
    reg_c: RegisterInteger,
    instruction_pointer: usize
}

impl ThreeBitComputer {
    fn do_instruction(&mut self, instruction: Instruction, operand: Operand) -> Option<String> {
        match instruction {
            Instruction::ADV => {
                let result = self.divide(self.reg_a, self.get_combo_operand(operand));
                self.reg_a = result;
                None
            },
            Instruction::BXL => {
                let result = self.bitwise_xor(self.reg_b, operand as i64);
                self.reg_b = result;
                None
            },
            Instruction::BST => {
                self.reg_b = self.modulo_8(self.get_combo_operand(operand));
                None
            },
            Instruction::JNZ => {
                if self.reg_a == 0 {
                    return None;
                }
                self.instruction_pointer = operand as usize;
                None
            },
            Instruction::BXC => {
                self.reg_b = self.bitwise_xor(self.reg_b, self.reg_c);
                None
            },
            Instruction::OUT => {
                let value = self.get_combo_operand(operand);
                Some(value.to_string())
            },
            Instruction::BDV => {
                let result = self.divide(self.reg_a, self.get_combo_operand(operand));
                self.reg_b = result;
                None
            },
            Instruction::CDV => {
                let result = self.divide(self.reg_a, self.get_combo_operand(operand));
                self.reg_c = result;
                None
            }
        }
    }

    fn get_combo_operand(&self, operand: Operand) -> RegisterInteger {
        match operand {
            0..=3 => operand as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("7 is a reserved operand and should not appear in a valid program"),
            _ => panic!("{} is not a known operand", operand),
        }
    }

    fn divide(&self, numerator: RegisterInteger, denominator: RegisterInteger) -> RegisterInteger {
        // how do we do 3 bit division?
        // remember to truncate it
        numerator / denominator
    }

    fn bitwise_xor(&self, input1: RegisterInteger, input2: RegisterInteger) -> RegisterInteger {
        // TODO: Is there anything special about bitwise xor 'ing a 3 bit thing?
        input1 ^ input2
    }

    fn modulo_8(&self, to_modulo: RegisterInteger) -> RegisterInteger {
        // TODO keep only its lowest 3 bits
        to_modulo
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

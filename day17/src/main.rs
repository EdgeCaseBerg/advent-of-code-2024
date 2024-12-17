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

    let output = computer_state.do_program(program);
    println!("{:?}", output);
}

fn parse_initial_state(data: &str) -> (RegisterInteger, RegisterInteger, RegisterInteger) {
    let mut line_iter = data.lines().take_while(|line| !line.is_empty()).take(3);
    let a: RegisterInteger = line_iter.next().unwrap().split(":").nth(1).unwrap().trim().parse().unwrap();
    let b: RegisterInteger = line_iter.next().unwrap().split(":").nth(1).unwrap().trim().parse().unwrap();
    let c: RegisterInteger = line_iter.next().unwrap().split(":").nth(1).unwrap().trim().parse().unwrap();
    (a, b, c)
}

fn parse_program_from(data: &str) -> Program {
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

type Operand = u8;
type RegisterInteger = i64;
type Program = VecDeque<(Instruction, Operand)>;

#[derive(Debug)]
struct ThreeBitComputer {
    reg_a: RegisterInteger,
    reg_b: RegisterInteger,
    reg_c: RegisterInteger,
    instruction_pointer: usize
}

const MASK_3_BITS: i64 = 0b111;

impl ThreeBitComputer {
    fn do_program(&mut self, program: Program) -> String {
        let mut temp_too_long = 0;
        let time_to_run = 100;
        let mut previous_register_state_was_zero = false;
        let mut program_output = String::new();
        loop {
            // Temp because the sample contains a JNZ 0 which loops the program which is
            // detrimental to my quick checking right now
            temp_too_long += 1;
            if self.instruction_pointer >= program.len() || temp_too_long > time_to_run {
                break;
            }

            if self.reg_a == 0 {
                previous_register_state_was_zero = true;
            }

            let (instruction, literal_operand) = program[self.instruction_pointer];
            let output = self.do_instruction(instruction, literal_operand);
            if let Some(output) = output {
                program_output.push_str(&output);
                program_output.push_str(",");
            }

            // Since we have tuples, this is not += 2, but just += 1.
            // Though we'll probably need to tweak this for jump commands and the like
            match instruction {
                Instruction::JNZ => {
                    if previous_register_state_was_zero {
                       self.instruction_pointer += 1; 
                    }
                },
                _ => self.instruction_pointer += 1,
            }
        }
        program_output
    }
    fn do_instruction(&mut self, instruction: Instruction, operand: Operand) -> Option<String> {
        match instruction {
            Instruction::ADV => {
                self.reg_a = self.divide(self.reg_a, self.get_combo_operand(operand));
                None
            },
            Instruction::BXL => {
                let result = self.bitwise_xor_operand(self.reg_b, operand);
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
                self.reg_b = self.bitwise_xor_register(self.reg_b, self.reg_c);
                None
            },
            Instruction::OUT => {
                let value = self.get_combo_operand(operand);
                Some((value & MASK_3_BITS).to_string())
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

    fn divide(&self, numerator: RegisterInteger, combo: RegisterInteger) -> RegisterInteger {
        let denom = 2_i64.pow(combo as u32);

        if denom == 0 {
            // uhhh....
            panic!("Division by 0???");
        }
        let untruncated_result = numerator / denom;
        untruncated_result
    }

    fn bitwise_xor_operand(&self, input1: RegisterInteger, input2: Operand) -> RegisterInteger {
        let xor = input1 ^ input2 as i64;
        xor as RegisterInteger
    }

    fn bitwise_xor_register(&self, input1: RegisterInteger, input2: RegisterInteger) -> RegisterInteger {
        let xor = input1 ^ input2;
        xor as RegisterInteger
    }

    fn modulo_8(&self, to_modulo: RegisterInteger) -> RegisterInteger {
        to_modulo & MASK_3_BITS
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

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn if_c_has_9_program_2_6_sets_register_b_to_1() {
        let mut computer_state = ThreeBitComputer {
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            instruction_pointer: 0
        };
        computer_state.do_instruction(Instruction::from(2), 6);
        assert_eq!(1, computer_state.reg_b);
    }

    #[test]
    fn if_a_contains_10_program_505154_outputs_012() {
        let mut computer_state = ThreeBitComputer {
            reg_a: 10,
            reg_b: 0,
            reg_c: 0,
            instruction_pointer: 0
        };
        let outputs = vec![
            computer_state.do_instruction(Instruction::from(5), 0),
            computer_state.do_instruction(Instruction::from(5), 1),
            computer_state.do_instruction(Instruction::from(5), 4)
        ];
        assert_eq!(vec![
            Some("0".to_string()),
            Some("1".to_string()),
            Some("2".to_string()),
        ], outputs);
    }

    #[test]
    fn if_reg_a_contains_2024_program_015430_outputs_42567777310_and_has_reg_a_as_9() {
        // Let us consider doing this test after we have some others working
        // let mut computer_state = ThreeBitComputer {
        //     reg_a: 2024,
        //     reg_b: 0,
        //     reg_c: 0,
        //     instruction_pointer: 0
        // };
        // let outputs = vec![
        //     computer_state.do_instruction(Instruction::from(0), 1),
        //     computer_state.do_instruction(Instruction::from(5), 4),
        //     computer_state.do_instruction(Instruction::from(3), 0),
        //     // Note JNZ is a jump
        // ];
        // assert_eq!(vec![
        //     Some("4".to_string()),
        //     Some("2".to_string()),
        //     Some("5".to_string()),
        //     Some("6".to_string()),
        //     Some("7".to_string()),
        //     Some("7".to_string()),
        //     Some("7".to_string()),
        //     Some("7".to_string()),
        //     Some("3".to_string()),
        //     Some("1".to_string()),
        //     Some("0".to_string())
        // ], outputs);
        // assert_eq!(0, computer_state.reg_a);
    }

    #[test]
    fn if_register_b_contains_29_program_17_sets_register_b_to_26() {
        let mut computer_state = ThreeBitComputer {
            reg_a: 0,
            reg_b: 29,
            reg_c: 0,
            instruction_pointer: 0
        };
        computer_state.do_instruction(Instruction::from(1), 7);
        assert_eq!(26, computer_state.reg_b);
    }

    #[test]
    fn if_register_b_contains_2024_and_register_c_contains_43690_program_40_sets_register_b_to_44354() {
        let mut computer_state = ThreeBitComputer {
            reg_a: 0,
            reg_b: 2024,
            reg_c: 43690,
            instruction_pointer: 0
        };
        computer_state.do_instruction(Instruction::from(4), 0);
        assert_eq!(44354, computer_state.reg_b);
    }    

}
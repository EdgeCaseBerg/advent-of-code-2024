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
    part_2(&data);
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

fn part_2(data: &str) {
    /* since things always go down, we want it to END on a specific 
     * output, so rather than go forwards... what if we go backwards?
     * 
     * Since the first instruction is always A % 8, then we only really have 0 - 7
     * that can produce an output. So, let's try to work out what each of those cam
     * do for us and maybe we can figure out a way to get to this sequence or work
     * backwards in some way?
     *
     * 3  -> 0 
     * 6  -> 3
     * 6  -> 3
     * 3  -> 0 <--- but we don't care maybe because this is an op to bxc which ignores its input.
     *    -> 5
     *    -> 5
     *    -> 0
     *0,1 -> 4
     *    -> 5
     * 5  -> 1
     *    -> 5
     * 3  -> 7
     * 5  -> 1
     * 5  -> 1
     *0,1 -> 4
     * 6  -> 2
     *
     * But maybe not because that can't produce a 5 it seems???
     *
     * Let's think about a different approach maybe... 
     * If I step through the first iteration of the program it looks like this:
     *  
        A: 64854237  11110111011001100011011101

        1. (BST, 4) A % 8                   --> to B
        2. (BXL, 1) B ^ 1                   --> to B
        3. (CDV, 5) A / 2 ** B --> truncate --> to C
        4. (BXL, 5) B ^ 5                  ---> to B
        5. (BXC, 0) B ^ C                  ---> to B
        6. (OUT, 5)                          print B
        7. (ADV, 3) A / 2 ** 3 --> truncate --> to A
        8. (JNZ, 0) loop if A is not 0. <-- shift right 3, same as 3.


        1. B: 0101 (5)
        2. B: 1101 ^ 0001 => 1100 (12)
        3. C: (a) / 4096 <--- on first input.
        4. B: 1100 ^ 0101 => 1001
        5. B: 1001 ^ ((a) / 4096) <-- on first input
        6. Output B
        7. A: (a) / 8
        8. Loop if A not 0

     * So potentially... since A is never written to, maybe we could do all the operations at once without 
     * the computer itself... does this reduce to some sort of weird math problem again where a line intersects
     * or some other such nonsense? Like, could I "solve" for A such that B is equal to the output I want? 
     * Would that even work for each output? 
     *
     * So doing that... I can see 0 can come from 4 and 12, 20, 21
     * So, does 4 * 8 = 32 = 3? 
     

        // This probably would work. But was too brute force.
        let program_in_reverse = vec![0,3,3,0,5,5,0,4,5,1,5,7,1,1,4,2];
        let mut lowest_register_to_produce_self = 0;
        loop {
         let value = search_register_from_start(lowest_register_to_produce_self, 0, &program_in_reverse);
         if value == RegisterInteger::MAX {
             lowest_register_to_produce_self += 1;    
         } else {
             break;
         }
        }
        println!("output {:?}", lowest_register_to_produce_self);


        fn compute_output_register(a: RegisterInteger) -> RegisterInteger {
            let step_1 = a % 8;
            let step_2 = step_1 ^ 1;
            let step_3 = a >> step_2;
            let step_4 = step_2 ^ 5;
            let step_5 = step_4 ^ step_3;
            step_5 % 8
        }

        fn search_register_from_start(a: RegisterInteger, pointer: usize, program_in_reverse: &Vec<RegisterInteger>) -> RegisterInteger {
            let to_find = program_in_reverse[pointer];
            let output = compute_output_register(a);
            if output == to_find {
                println!("{:?}", pointer);
                if pointer + 1 == program_in_reverse.len() {
                    println!("We made it to the end? {:?}", a);
                    return a;
                }
                return search_register_from_start(a * 8, pointer + 1, program_in_reverse);
            }
            // Flag of failure.
            return RegisterInteger::MAX;
        }


        some more options because we do stuff by 8's a lot, so, is there a pattern with octals?

            let (_, b, c) = parse_initial_state(data);
            let program = parse_program_from(data);
            let target = print_program(&program.clone());
            println!("{:?}", target);
            // let mut upper_bound = 0o7777_7777_7777_7777;
            // let mut lower_bound = 0o1000_0000_0000_0000;
            // 0o_0001; -> 4 
            let mut string_rep = String::from("24117515405503300");
            let mut initial_a = u64::from_str_radix(&string_rep, 8).unwrap();
            let mut block = 1;
            for i in 0..string_rep.len() {
                for j in 0..8 {
                    let to_try = string_rep.replace_range(i..=i, &j.to_string());
                    let mut computer_state = ThreeBitComputer {
                        reg_a: u64::from_str_radix(&string_rep, 8).unwrap(),
                        reg_b: b,
                        reg_c: c,
                        instruction_pointer: 0
                    };
                    
                    let output = computer_state.do_program(program.clone());
                    println!("{:?} {:?}", string_rep, output);
                    if output == target {
                        break;
                    }
                    let to_try = string_rep.replace_range(i..=i, &j.to_string());
                    let string_rep = string_rep.chars().rev().collect::<String>();
                    let mut computer_state = ThreeBitComputer {
                        reg_a: u64::from_str_radix(&string_rep, 8).unwrap(),
                        reg_b: b,
                        reg_c: c,
                        instruction_pointer: 0
                    };
                    
                    let output = computer_state.do_program(program.clone());
                    println!("{:?} {:?}", string_rep, output);
                    if output == target {
                        break;
                    }
                }
            }

             let mut computer_state = ThreeBitComputer {
                reg_a: 0o4700_0000_0000_0000,
                reg_b: b,
                reg_c: c,
                instruction_pointer: 0
            };
            
            let output = computer_state.do_program(program.clone());
            println!("Try {:?} {:?}", string_rep, output);
    */

    // This probably would work. But was too brute force.
    let program_in_reverse = vec![0,3,3,0,5,5,0,4,5,1,5,7,1,1,4,2];
    let mut value = 0;
    let lowest_register_to_produce_self = search_register_from_start(value, 0, &program_in_reverse);
    println!("output {:?}", lowest_register_to_produce_self);
    // 18446744073709551615 too high
    // 20534878121431  too low
    println!("{:?}", lowest_register_to_produce_self);
    let mut computer_state = ThreeBitComputer {
        reg_a: lowest_register_to_produce_self,
        reg_b: 0,
        reg_c: 0,
        instruction_pointer: 0
    };

    let program = parse_program_from(data);
    let output = computer_state.do_program(program);
    println!("{:?}", output);
}

fn search_register_from_start(a: RegisterInteger, pointer: usize, program_in_reverse: &Vec<RegisterInteger>) -> RegisterInteger {
    let to_find = program_in_reverse[pointer];
    let mut answer = RegisterInteger::MAX;
    if pointer + 1 == program_in_reverse.len() {
        for i in 0..8 {
            let test_a = (a << 3) | i;
            let output = compute_output_register(test_a);
            if output == to_find {
                // Select the smallest values of these.
                if test_a < answer {
                    answer = test_a
                }
            }
        }
        return answer;
    } else {
        let mut answers = vec![];
        for i in 0..8 {
            let test_a = (a << 3) | i;
            let output = compute_output_register(test_a);
            if output == to_find {
                let n_answer = search_register_from_start(test_a, pointer + 1, program_in_reverse);
                answers.push(n_answer);
            }    
        }
        answers.sort();
        if answers.len() == 0 {
            return RegisterInteger::MAX
        }
        answer = answers[0];
    }
    
    return answer;
}

fn compute_output_register(a: RegisterInteger) -> RegisterInteger {
    let step_1 = a % 8;
    let step_2 = step_1 ^ 1;
    let step_3 = a >> step_2;
    let step_4 = step_2 ^ 5;
    let step_5 = step_4 ^ step_3;
    step_5 % 8
}

fn print_program(program: &Program) -> String {
    let mut out = String::new();
    let mut to_consume = program.clone();
    while let Some((instruction, operand)) = to_consume.pop_front() {
        out.push_str(&instruction.to_num().to_string());
        out.push_str(",");
        out.push_str(&operand.to_string());
        if !to_consume.is_empty() {
            out.push_str(",");
        }
    }
    out
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
type RegisterInteger = u64;
type Program = VecDeque<(Instruction, Operand)>;

#[derive(Debug)]
struct ThreeBitComputer {
    reg_a: RegisterInteger,
    reg_b: RegisterInteger,
    reg_c: RegisterInteger,
    instruction_pointer: usize
}

const MASK_3_BITS: RegisterInteger = 0b111;

impl ThreeBitComputer {
    fn do_program(&mut self, program: Program) -> String {
        let mut previous_register_state_was_zero = false;
        let mut program_output = String::new();
        let mut has_output = false;
        loop {
            // Temp because the sample contains a JNZ 0 which loops the program which is
            // detrimental to my quick checking right now
            if self.instruction_pointer >= program.len()  {
                break;
            }

            if self.reg_a == 0 {
                previous_register_state_was_zero = true;
            }

            let (instruction, literal_operand) = program[self.instruction_pointer];
            // println!("{:?} {:?}: {:?}", instruction, literal_operand, self);
            let output = self.do_instruction(instruction, literal_operand);
            if let Some(output) = output {
                if has_output {
                    program_output.push_str(",");
                }
                program_output.push_str(&output);
                has_output = true;
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
            0..=3 => operand as RegisterInteger,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("7 is a reserved operand and should not appear in a valid program"),
            _ => panic!("{} is not a known operand", operand),
        }
    }

    fn divide(&self, numerator: RegisterInteger, combo: RegisterInteger) -> RegisterInteger {
        let denom = 2_u64.pow(combo as u32);

        if denom == 0 {
            // uhhh....
            panic!("Division by 0???");
        }
        let untruncated_result = numerator / denom;
        untruncated_result
    }

    fn bitwise_xor_operand(&self, input1: RegisterInteger, input2: Operand) -> RegisterInteger {
        let xor = input1 ^ input2 as RegisterInteger;
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn to_num(&self) -> Operand {
        vec![
            Instruction::ADV,
            Instruction::BXL,
            Instruction::BST,
            Instruction::JNZ,
            Instruction::BXC,
            Instruction::OUT,
            Instruction::BDV,
            Instruction::CDV
        ].iter().position(|&i| i == *self).unwrap() as Operand
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
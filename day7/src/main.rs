use std::fs;
use std::collections::VecDeque;

fn main() {
    let raw_data = fs::read_to_string("./input.txt").expect("bad input data");
    let calibrations: Vec<Calibration> = raw_data.lines().map(|line| {
        Calibration::from(line)
    }).collect();
    
    let total_values_from_valid_calibrations = calibrations.iter().fold(0, |accum, calibration| {
        let operator_combinations: Vec<VecDeque<Operand>> = get_operator_combinations(calibration);
        let mut has_valid = false;
        for operators in operator_combinations {
            has_valid = has_valid || calibration.is_valid_with(operators);
        }
        if has_valid {
            accum + calibration.result
        } else {
            accum
        }
        
    });
    println!("{:?}", total_values_from_valid_calibrations);
    // sample should be 3749

}

#[derive(Debug, Clone)]
struct Calibration {
    result: u64,
    numbers: Vec<u64>
}

impl Calibration {
    fn from(line: &str) -> Calibration {
        let mut iter = line.split(":");
        let result = iter.next().unwrap().parse().unwrap();
        let numbers = iter.next().unwrap().split(" ").filter(|s| !s.is_empty()).map(|n| {
            n.parse().unwrap()
        }).collect();
        Calibration {
            result,
            numbers
        }
    }

    fn is_valid_with(&self, mut operands: VecDeque<Operand>) -> bool {
        if self.numbers.is_empty() || operands.is_empty() {
            return false
        }

        let mut nums = self.numbers.clone().into_iter();
        let mut left = nums.next().unwrap();
        for _ in 0..nums.len() {
            let right = nums.next().unwrap();
            let operator = operands.pop_front().unwrap(); 
            left = operator.apply(&left, &right);
        }
        self.result == left
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Plus,
    Multiply,
    Concat,
}

impl Operand {
    fn apply(&self, left: &u64, right: &u64) -> u64 {
        match self {
            Operand::Plus => left + right,
            Operand::Multiply => left * right,
            Operand::Concat => (left.to_string() + &right.to_string()).parse().unwrap()
        }
    }
}

fn get_operator_combinations(calibration: &Calibration) -> Vec<VecDeque<Operand>> {
    let operators_needed = calibration.numbers.len() - 1;
    generate_combinations(&['+', '*', '|'], operators_needed).into_iter().map(|string| {
        let mut combo = VecDeque::new();
        for c in string.chars() {
            let op = match c {
                '+' => Operand::Plus,
                '|' => Operand::Concat,
                _ => Operand::Multiply,
            };
            combo.push_back(op)
        }
        combo
    }).collect::<Vec<VecDeque<Operand>>>()
}

fn generate_combinations(symbols: &[char], n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let mut result = Vec::new();
    for symbol in symbols {
        let sub_combinations = generate_combinations(symbols, n - 1);
        for sub in sub_combinations {
            result.push(format!("{}{}", symbol, sub));
        }
    }

    result
}
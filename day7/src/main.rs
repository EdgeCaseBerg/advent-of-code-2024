use std::fs;
use std::collections::VecDeque;

fn main() {
    let raw_data = fs::read_to_string("./input.txt").expect("bad input data");
    let calibrations: Vec<Calibration> = raw_data.lines().map(|line| {
        Calibration::from(line)
    }).collect();
    
    println!("{:?}", calibrations);

    let total_values_from_valid_calibrations = calibrations.iter().fold(0, |accum, calibration| {
        let operator_combinations: Vec<VecDeque<Operand>> = get_operator_combinations(&calibration);
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
            result: result,
            numbers: numbers
        }
    }

    fn is_valid_with(&self, mut operands: VecDeque<Operand>) -> bool {
        // println!("{:?} {:?}", self, operands.as_slices());
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
        return self.result == left;
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Plus,
    Multiply,
}

impl Operand {
    fn apply(&self, left: &u64, right: &u64) -> u64 {
        match self {
            Operand::Plus => left + right,
            Operand::Multiply => left * right,
        }
    }
}

fn get_operator_combinations(calibration: &Calibration) -> Vec<VecDeque<Operand>> {
    let operators_needed = calibration.numbers.len() - 1;
    let operators = Vec::from([Operand::Plus, Operand::Multiply]);


    let r = generate_combinations(&['+', '*'], operators_needed).into_iter().map(|string| {
        let mut combo = VecDeque::new();
        for c in string.chars() {
            let op = match c {
                '+' => Operand::Plus,
                _ => Operand::Multiply,
            };
            combo.push_back(op)
        }
        combo
    }).collect::<Vec<VecDeque<Operand>>>();
    r
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
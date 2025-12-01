use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let raw_data = fs::read_to_string("./input.txt").expect("bad input data");
    let mut calibrations: Vec<Calibration> = raw_data.lines().map(|line| {
        Calibration::from(line)
    }).collect();

    let mut permutation_cache = HashMap::new();
    
    let total_values_from_valid_calibrations = calibrations.iter_mut().fold(0, |accum, calibration| {
        let operator_combinations: Vec<VecDeque<Operand>> = calibration.get_operator_combinations(&mut permutation_cache);
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
    // not 57229 ???
    //  yes 1289579105366
    //  yes 92148721834692
}

#[derive(Debug, Clone)]
struct Calibration {
    result: u64,
    numbers: Vec<u64>
}

impl Calibration {
    fn new(result: u64, numbers: Vec<u64>) -> Calibration {
        Calibration {
            result,
            numbers
        }
    }

    fn from(line: &str) -> Calibration {
        let mut iter = line.split(":");
        let result = iter.next().unwrap().parse().unwrap();
        let numbers = iter.next().unwrap().split(" ").filter(|s| !s.is_empty()).map(|n| {
            n.parse().unwrap()
        }).collect();
        Calibration::new(result, numbers)
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

    fn get_operator_combinations(&mut self, permutation_cache: &mut HashMap<usize, Vec<VecDeque<Operand>>>) -> Vec<VecDeque<Operand>> {
        let operators_needed = self.numbers.len() - 1;
        if permutation_cache.contains_key(&operators_needed) {
            return permutation_cache.get(&operators_needed).unwrap().to_vec();
        }

        let o = vec![Operand::Multiply, Operand::Plus, Operand::Concat];
        let f: Vec<Vec<Operand>> = itertools::repeat_n(o, operators_needed).multi_cartesian_product().collect();
        let combinations: Vec<VecDeque<Operand>> = f.iter().map(|v| {
            let mut vdq = VecDeque::new();
            for o in v.iter() {
                vdq.push_back(*o);
            }
            vdq
        } ).collect();
        permutation_cache.insert(operators_needed, combinations.clone());
        combinations
    }
}

#[derive(Debug, Clone, Copy)]
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



// let combinations = generate_combinations(&[Operand::Concat, Operand::Plus, Operand::Multiply], operators_needed);
// permutation_cache.insert(operators_needed, combinations.clone());
// return combinations
        
// fn generate_combinations(symbols: &[Operand], n: usize) -> Vec<VecDeque<Operand>> {
//     // return nothing to add,
//     // when n = 1 this will end up with [Operand]
//     // when n = 2 this will have [Operand, Operand]
//     // n = 1 will vary between the different operands, so by concatenating
//     // the bits together, we can build up the different combos.
//     if n == 0 {
//         return vec![VecDeque::new()];
//     }

//     let mut result = Vec::new();
//     for symbol in symbols {
//         let sub_combinations = generate_combinations(symbols, n - 1);
//         for mut sub in sub_combinations {
//             sub.push_front(*symbol);
//             result.push(sub);
//         }
//     }

//     result
// }
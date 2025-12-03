use std::fs;

fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    p1(raw_data);
    p2(raw_data);
}

fn p1(input: &str) {
    let joltage: usize = input.lines().map(|line| {
        if line.is_empty() {
            0
        } else {
            compute_joltage_p1(line)
        }
    }).sum();
    println!("{:?}", joltage);
}

fn p2(input: &str) {
    let joltage: usize = input.lines().map(|line| {
        if line.is_empty() {
            0
        } else {
            compute_joltage(line, 12)
        }
    }).sum();
    println!("{:?}", joltage);
}

fn compute_joltage_p1(line: &str) -> usize {
    let mut joltage = 0;
    let bank_length = line.len();

    for (left_idx, left) in line.chars().enumerate() {
        if left == '0' {
            continue;
        }
        // Skip it if you can't turn on N batteries anyway
        if left_idx > bank_length - 2 {
            continue;
        }
        let mut banks_on = String::new();
        banks_on.push(left);
        for right in line.chars().skip(left_idx + 1) {
            let mut bank = banks_on.clone();
            let combined: usize = format!("{}{}", left, right).parse().expect("Cant convert left and right to number");
            if combined > joltage {
                joltage = combined;
            }
        }
    }
    joltage
}

fn compute_joltage(line: &str, allowed_batteries_on: usize) -> usize {
    //  stop being dumb and use a stack.
    0
}

// this will probably run to the end of the universe.
// use itertools::Itertools;
// fn compute_joltage(line: &str, allowed_batteries_on: usize) -> usize {
//     let mut joltage = 0;
//     let mut holes: Vec<bool> = Vec::new();
//     for (idx, _) in line.chars().enumerate() {
//         let state = if idx < allowed_batteries_on { true } else { false };
//         holes.push(state);
//     }
//     let len = holes.len();
//     let mut combinations = holes.into_iter().permutations(len).unique();
//     println!("{:?}", combinations);

//     while let Some(combination) = combinations.next() {
//         println!("{:?}", combination);
//         let mut bank = String::from("");
//         for (idx, &state) in combination.iter().enumerate() {
//             if state {
//                 bank.push_str(&line[idx..idx + 1]);    
//             }
//         }
//         println!("{:?}", bank);
//         let combination_joltage: usize = bank.parse().expect("Cant convert combination to number");
//         if combination_joltage > joltage {
//             joltage = combination_joltage
//         }
//         println!("{:?}", combination_joltage);
//     }
//     println!("best joltage {:?} {:?}", line, joltage);

//     return joltage;
// }
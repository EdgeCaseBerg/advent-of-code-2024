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
    let mut selected_batteries = Vec::with_capacity(allowed_batteries_on);
    let length = line.chars().count();
    let mut remaining_batteries = line.chars().enumerate();
    while let Some((idx, battery_value)) = remaining_batteries.next() {
        let battery = battery_value as i32 - 0x30;

        // Seed value for the stack.
        if idx < 1 {
            selected_batteries.push(battery);
            continue;
        }

        // TODO If the top fo the stack is less than the current battery then
        // we should be able to replace it since it will make a bigger number.
        // we should be able to do this WHILE the top is less, unless we have
        // only a few more batteries to go, in which case we'd be better off
        // just appending it so the number can grow to its maximum size.
        let top = selected_batteries.pop().expect("The top should never be empty");
        let remaining_battery_count = length - idx;        
        let stack_size = selected_batteries.len();

        // this is all wrong ugh. but I know what I want to do its just hard to express it
        let left_to_check = length - idx;
        if battery > top && stack_size < allowed_batteries_on {
            while let Some((_, battery_value)) = remaining_batteries.next() {
                let battery = battery_value as i32 - 0x30;
                if battery <= top {
                    break;
                }
                selected_batteries.push(battery);
            }
            selected_batteries.push(battery);
            continue;
        }

        if battery == top {
            selected_batteries.push(top);
            selected_batteries.push(battery);
            continue
        }

        // In the case where battery is less than the top
        if stack_size < allowed_batteries_on {
            selected_batteries.push(top);
            selected_batteries.push(battery);
        }
    }

    while selected_batteries.len() > allowed_batteries_on {
        selected_batteries.pop();
    }
    let mut combined = String::with_capacity(allowed_batteries_on);
    for battery in selected_batteries {
        combined.push((battery as u8 + 0x30) as char);
    }
    let final_joltage = combined.parse().expect("joltage did not convert");
    println!("{:?}", final_joltage);
    return final_joltage;
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
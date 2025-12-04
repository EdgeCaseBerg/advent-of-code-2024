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

fn compute_joltage(battery_bank: &str, allowed_batteries_on: usize) -> usize {
    let mut enabled_digits = Vec::with_capacity(allowed_batteries_on);
    let digits = battery_bank.chars().enumerate();
    let length = battery_bank.chars().count();

    for (idx, battery_character) in digits {
        let battery_value = battery_character as u8 - b'0';

        while let Some(&top) = enabled_digits.last() {

            let enabled_count = enabled_digits.len();
            let remaining_digit_count = length - idx;
            let battery_provides_better_joltage = top < battery_value;
            let can_still_fill_bank_to_limit = enabled_count + remaining_digit_count > allowed_batteries_on;

            if battery_provides_better_joltage && can_still_fill_bank_to_limit {
                enabled_digits.pop();
            } else {
                break;
            }
        }

        if enabled_digits.len() < allowed_batteries_on {
            enabled_digits.push(battery_value);
        }
    }


    let mut combined = String::with_capacity(allowed_batteries_on);
    for battery in enabled_digits {
        combined.push((battery as u8 + b'0') as char);
    }
    let final_joltage = combined.parse().expect("joltage did not convert");
    return final_joltage;
}
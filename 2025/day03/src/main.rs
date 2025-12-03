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
            compute_joltage(line, 2)
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

fn compute_joltage(line: &str, allowed_batteries_on: usize) -> usize {
    let mut joltage = 0;
    let bank_length = line.len();

    for (left_idx, left) in line.chars().enumerate() {
        if left == '0' {
            continue;
        }
        // Skip it if you can't turn on N batteries anyway
        if left_idx > bank_length - allowed_batteries_on {
            continue;
        }
        let mut banks_on = String::new();
        banks_on.push(left);
        for right in line.chars().skip(left_idx + 1) {
            let mut bank = banks_on.clone();
            // turn on "allowed_batteries_on" banks then check joltage.
            // we can only ever move rightwards since you can't re-arrange batteries
            // so it's a question of [L,_,_,+,_,_+] which banks to the right of the L should we turn on for max jolt?
            // TODO: get the permutation to the right. Perhaps as a bitset for added fun.
            // 4095 is the binary number of 1111 1111 1111 so perhaps using all numebrs between there to flip on and
            // check joltage would work. But for now. To work I go.
            let combined: usize = format!("{}{}", left, right).parse().expect("Cant convert left and right to number");
            if combined > joltage {
                joltage = combined;
            }
        }
    }
    joltage
}
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
            compute_joltage(line)
        }
    }).sum();
    println!("{:?}", joltage);
}

fn p2(_: &str) {
    
}

fn compute_joltage(line: &str) -> usize {
    let mut joltage = 0;

    for (leftIndex, left) in line.chars().enumerate() {
        if left == '0' {
            continue;
        }
        for right in line.chars().skip(leftIndex + 1) {
            let combined: usize = format!("{}{}", left, right).parse().expect("Cant convert left and right to number");
            if combined > joltage {
                joltage = combined;
            }
        }
    }
    joltage
}
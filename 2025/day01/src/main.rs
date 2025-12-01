use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    // 0 - 99 in order
    // clicks each time
    // L--, R++
    //11 -> R8 -> 19
    // 0 left wraps to 99, 99 -> 0/
    // dial starts at 50
    // password is how many times it hits 0.
    let mut start = 50;
    let mut clicks = 0;
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let operations = raw_data.lines().filter(|line| !line.is_empty());
    for op in operations {
        let mut turns = 1;
        match op.chars().take(1).last().unwrap() {
            'R' => {
                let num:i32 = op[1..].parse().unwrap();
                for i in 0..num {
                    start+=1;
                    if start == 100 {
                        start = 0
                    }
                }
            }
            'L' => {
                let num: i32 = op[1..].parse().unwrap();
                for i in 0..num {
                    start-=1;
                    if start == -1 {
                        start = 99
                    }
                }
            },
            _ => todo!()
        }
        if start == 0 {
            clicks = clicks + turns;
        }
    }
    println!("{:?}", clicks);
}

fn part2() {
    // 0 - 99 in order
    // clicks each time
    // L--, R++
    //11 -> R8 -> 19
    // 0 left wraps to 99, 99 -> 0/
    // dial starts at 50
    // password is how many times it hits 0.
    let mut start = 50;
    let mut clicks = 0;
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let operations = raw_data.lines().filter(|line| !line.is_empty());
    for op in operations {
        let mut turns = 0;
        match op.chars().take(1).last().unwrap() {
            'R' => {
                let num:i32 = op[1..].parse().unwrap();
                for _ in 0..num {
                    start+=1;
                    if start == 100 {
                        start = 0;
                    }
                    if start == 0 {
                        turns += 1;
                    }
                }
            }
            'L' => {
                let num: i32 = op[1..].parse().unwrap();
                for _ in 0..num {
                    start-=1;
                    if start == -1 {
                        start = 99;
                    }
                    if start == 0 {
                        turns += 1;
                    }
                }
            },
            _ => todo!()
        }
        clicks = clicks + turns;

    }
    println!("{:?}", clicks);
}
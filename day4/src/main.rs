use std::fs;


fn main() {
    let raw_data = fs::read_to_string("small_input_day_4.txt").expect("bad input data");
    // let raw_data = fs::read_to_string("../../input-day-4.txt").expect("bad input data");
    let needle = "XMAS";

    // Break the string up into a box
    let matrix: Vec<Vec<String>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars().map(|s| {
                if needle.contains(s) {
                    s.to_string()
                } else {
                    " ".to_string()
                }
            }).collect()
        })
        .collect();

    let mut going_forward: Vec<String> = Vec::new();
    let mut going_backwards: Vec<String> = Vec::new();
    let mut going_down: Vec<String> = Vec::new();
    let mut going_up: Vec<String> = Vec::new();
    let mut going_diagonial: Vec<String> = Vec::new();

    for y in 0..matrix.len() {
        let line = &matrix[y];
        going_forward.push(line.clone().into_iter().collect());
        going_backwards.push(line.clone().into_iter().rev().collect());
        for x in 0..line.len() {
            // downwards
            if y == 0 {
                let mut downwards = String::new();
                for y2 in 0..matrix.len() {
                    downwards.push_str(&matrix[y2][x]);
                }
                going_down.push(downwards.clone());
                going_up.push(downwards.clone().chars().rev().collect());
            }
        }
    }

    // Diagonals confuse my brain, do them separately.
    for y in 0..matrix.len() {
        if y != 0 {
            continue;
        }
        let line = &matrix[0];
        for x in 0..line.len() {
            // diag is tricky ish
            let mut diag = String::new();
            for offset in 0..matrix.len() {
                if y + offset < matrix.len() && x + offset < line.len() {
                    diag.push_str(&matrix[y + offset][x + offset]);
                }
            }
            going_diagonial.push(diag.clone());
            going_diagonial.push(diag.clone().chars().rev().collect());
        }
    }



    println!("{:?}", going_diagonial);

    let mut counts = 0;
    counts += going_forward.iter().fold(0, |a, hay| a + hay.matches(needle).count());
    println!("going_forward {:?}", going_forward.iter().fold(0, |a, hay| a + hay.matches(needle).count()));
    counts += going_backwards.iter().fold(0, |a, hay| a + hay.matches(needle).count());
    println!("going_backwards {:?}", going_backwards.iter().fold(0, |a, hay| a + hay.matches(needle).count()));
    counts += going_down.iter().fold(0, |a, hay| a + hay.matches(needle).count());
    println!("going_down {:?}", going_down.iter().fold(0, |a, hay| a + hay.matches(needle).count()));
    counts += going_up.iter().fold(0, |a, hay| a + hay.matches(needle).count());
    println!("going_up {:?}", going_up.iter().fold(0, |a, hay| a + hay.matches(needle).count()));
    counts += going_diagonial.iter().fold(0, |a, hay| a + hay.matches(needle).count());
    println!("going_diagonial {:?}", going_diagonial.iter().fold(0, |a, hay| a + hay.matches(needle).count()));

    // 96416 is NOT the answer
    println!("{:?}", counts);
}

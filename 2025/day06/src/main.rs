use std::fs;

fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    p1(raw_data);
    p2(raw_data);
}

fn p1(raw_data: &str) {
    let lines: Vec<Vec<&str>> = raw_data.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.trim().split(" ").map(|raw| raw.trim()).filter(|raw| !raw.is_empty()).collect()
        }).collect();

    let copy = lines.clone();
    let ops = copy.last().expect("cant get op line");
    let rows_of_numbers_count = lines.len() - 1;

    let mut cursors: Vec<_> = 
        lines.into_iter()
            .take(rows_of_numbers_count)
            .map(|split_line| split_line.into_iter())
            .collect();

    let mut sum = 0;
    for op in ops {
        match *op {
            "+" => {
                for i in 0..rows_of_numbers_count {
                    let num: usize = cursors[i].next().expect("no next number").parse().expect("couldnt parse it");
                    sum += num;
                }
            }
            "*" => {
                let mut tmp = 1;
                for i in 0..rows_of_numbers_count {
                    let num: usize = cursors[i].next().expect("no next number").parse().expect("couldnt parse it");
                    tmp *= num;
                }
                sum += tmp;
            }
            _ => unreachable!()
        }
    }
    println!("{:?}", sum);
}

fn p2(raw_data: &str) {
    // white space is now significant
    // but the operator defines the boundary leftmost part which should help.
    // but maybe the simpler thing to do is to just transpose the input itself
    
    let (matrix, rows, cols) = make_matrix(raw_data);
    // read right to left
    let mut numbers_in_problem: Vec<usize> = Vec::new();
    let mut total = 0;
    for col in (0..cols).rev() {
        // Most significant digit is at the top, so read top to bottom
        println!("{:?}", col);
        let mut number = String::with_capacity(rows);
        for row in 0..rows {
            println!("{:?}", col);
            // Do we have an operand?
            let operand_row = row == rows - 1;
            println!("{:?} {:?} {:?}", operand_row, number, matrix[row][col]);
            match (matrix[row][col], operand_row) {
                (' ', true) => {
                    // end of the number we're building.
                    println!("{:?} r{:?},c{:?}", number, row, col);
                    if number.trim().is_empty() {
                        numbers_in_problem = Vec::new();
                        continue;
                    }
                    let number_value: usize = number.trim().parse().expect("couldnt convert number!");
                    numbers_in_problem.push(number_value);
                    number = String::with_capacity(rows);
                },
                ('+', true) => {
                    let number_value: usize = number.trim().parse().expect("couldnt convert number!");
                    numbers_in_problem.push(number_value);
                    total += numbers_in_problem.into_iter().sum::<usize>();
                    numbers_in_problem = Vec::new();
                    number = String::with_capacity(rows);
                },
                ('*', true) => {
                    let number_value: usize = number.trim().parse().expect("couldnt convert number!");
                    numbers_in_problem.push(number_value);
                    total += numbers_in_problem.into_iter().reduce(|a, b| a * b).expect("no numbers to multiply?");
                    numbers_in_problem = Vec::new();
                    number = String::with_capacity(rows);
                },
                (c, false) => {
                    number.push(c);
                },
                _ => unreachable!()
            }
        }
    }

    println!("{:?}", total);
}

fn make_matrix(raw_data: &str) -> (Vec<Vec<char>>, usize, usize) {
        // Parse the matrix
    let matrix: Vec<Vec<char>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = matrix.len();
    let cols = matrix.get(0).map(|m| m.len()).unwrap_or(0);
    (matrix, rows, cols)
}
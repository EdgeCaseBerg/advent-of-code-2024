use std::fs;

fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    p1(raw_data);
    p2(raw_data);
}

fn p1(input: &str) {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = matrix.len();
    let cols = matrix.get(0).map(|m| m.len()).unwrap_or(0);
    let mut result = 0;

    for y in 0..rows {
        for x in 0..cols {
            if matrix[y][x] == '@' {
                if count_paper_around_point(&matrix, y as isize, x as isize) < 4 {
                    result += 1
                }
            }
        }
    }

    println!("{:?}", result);
}

fn count_paper_around_point(matrix: &Vec<Vec<char>>, row: isize, col: isize) -> usize {
    let rows = matrix.len() as isize;
    let cols = matrix.get(0).map(|m| m.len()).unwrap_or(0) as isize;
    let mut positions_around_us = Vec::with_capacity(8);
    println!("!!!");
    for i in -1..=1 {
        for j in -1..=1 {
            println!("{:?} {:?}", i, j);
            if i == 0 && j == 0 {
                continue;
            }
            println!("? {:?}{:?}", row, col);
            if 0 <= row + i && row + i < rows {
                if 0 <= col + j && col + j < cols {
                    let row = (row + i) as usize;
                    let col = (col + j) as usize;
                    println!("?? {:?},{:?}", row, col);
                    positions_around_us.push(matrix[row][col] == '@');
                }
            }
        }
    }
    positions_around_us.iter().filter(|&is_paper| *is_paper).count()
}

fn p2(input: &str) {
    let mut matrix: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = matrix.len();
    let cols = matrix.get(0).map(|m| m.len()).unwrap_or(0);
    let mut result = 0;

    loop {
        let mut total_removed = 0;
        for y in 0..rows {
            for x in 0..cols {
                if matrix[y][x] == '@' {
                    if count_paper_around_point(&matrix, y as isize, x as isize) < 4 {
                        result += 1;
                        total_removed += 1;
                        matrix[y][x] = 'x';
                    }
                }
            }
        }
        if total_removed == 0 {
            break;
        }
    }
    println!("{:?}", result);
}

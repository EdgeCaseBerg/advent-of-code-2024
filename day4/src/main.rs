use std::fs;

fn main() {
    let raw_data = fs::read_to_string("../../input-day-4.txt").expect("bad input data");
    let needle = "XMAS";

    // Parse the matrix
    let matrix: Vec<Vec<char>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut going_forward = vec![];
    let mut going_backwards = vec![];
    let mut going_down = vec![];
    let mut going_up = vec![];
    let mut going_diagonal = vec![];

    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };

    // Horizontal directions (forward and backward)
    for row in &matrix {
        let line: String = row.iter().collect();
        going_forward.push(line.clone());
        going_backwards.push(line.chars().rev().collect());
    }

    // Vertical directions (down and up)
    for col in 0..cols {
        let mut downwards = String::new();
        for row in 0..rows {
            downwards.push(matrix[row][col]);
        }
        going_down.push(downwards.clone());
        going_up.push(downwards.chars().rev().collect());
    }

    // Diagonal directions
    for start_row in 0..rows {
        // Down-right diagonal from left column
        let mut diag = String::new();
        let mut offset = 0;
        while start_row + offset < rows && offset < cols {
            diag.push(matrix[start_row + offset][offset]);
            offset += 1;
        }
        going_diagonal.push(diag.clone());
        going_diagonal.push(diag.chars().rev().collect());
    }

    for start_col in 1..cols {
        // Down-right diagonal from top row
        let mut diag = String::new();
        let mut offset = 0;
        while offset < rows && start_col + offset < cols {
            diag.push(matrix[offset][start_col + offset]);
            offset += 1;
        }
        going_diagonal.push(diag.clone());
        going_diagonal.push(diag.chars().rev().collect());
    }

    for start_row in 0..rows {
        // Down-left diagonal from right column
        let mut diag = String::new();
        let mut offset = 0;
        while start_row + offset < rows && cols as isize - 1 - offset as isize >= 0 {
            diag.push(matrix[start_row + offset][(cols as isize - 1 - offset as isize) as usize]);
            offset += 1;
        }
        going_diagonal.push(diag.clone());
        going_diagonal.push(diag.chars().rev().collect());
    }

    for start_col in (0..cols - 1).rev() {
        // Down-left diagonal from top row
        let mut diag = String::new();
        let mut offset = 0;
        while offset < rows && start_col as isize - offset as isize >= 0 {
            diag.push(matrix[offset][(start_col as isize - offset as isize) as usize]);
            offset += 1;
        }
        going_diagonal.push(diag.clone());
        going_diagonal.push(diag.chars().rev().collect());
    }

    // Count occurrences of the needle
    let mut counts = 0;
    counts += count_matches(&going_forward, needle);
    counts += count_matches(&going_backwards, needle);
    counts += count_matches(&going_down, needle);
    counts += count_matches(&going_up, needle);
    counts += count_matches(&going_diagonal, needle);

    println!("Total matches for '{}': {}", needle, counts);
}

fn count_matches(lines: &[String], needle: &str) -> usize {
    lines.iter().map(|line| line.match_indices(needle).count()).sum()
}

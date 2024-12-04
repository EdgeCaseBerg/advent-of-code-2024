use std::fs;

fn main () {
    let raw_data = fs::read_to_string("../../input-day-4.txt").expect("bad input data");
    // Parse the matrix
    let matrix: Vec<Vec<char>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = matrix.len();
    let cols = matrix.get(0).map(|m| m.len()).unwrap_or(0);

    let mut count = 0;
    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            if matrix[y][x] == 'A' {
                // Just brute force it with a lookup around us
                /* M M  M S S M S S 
                 *  A    A   A   A
                 * S S  M S S M M M
                 */
                let top_left_to_bottom_right = matrix[y - 1][x - 1] == 'M' && matrix[y + 1][x + 1] == 'S';
                let bottom_left_to_top_right = matrix[y + 1][x - 1] == 'S' && matrix[y - 1][x + 1] == 'M';
                let case_1 = top_left_to_bottom_right && bottom_left_to_top_right;

                let top_left_to_bottom_right = matrix[y - 1][x - 1] == 'M' && matrix[y + 1][x + 1] == 'S';
                let bottom_left_to_top_right = matrix[y + 1][x - 1] == 'M' && matrix[y - 1][x + 1] == 'S';
                let case_2 = top_left_to_bottom_right && bottom_left_to_top_right;

                let top_left_to_bottom_right = matrix[y - 1][x - 1] == 'S' && matrix[y + 1][x + 1] == 'M';
                let bottom_left_to_top_right = matrix[y + 1][x - 1] == 'S' && matrix[y - 1][x + 1] == 'M';
                let case_3 = top_left_to_bottom_right && bottom_left_to_top_right;

                let top_left_to_bottom_right = matrix[y - 1][x - 1] == 'S' && matrix[y + 1][x + 1] == 'M';
                let bottom_left_to_top_right = matrix[y + 1][x - 1] == 'M' && matrix[y - 1][x + 1] == 'S';
                let case_4 = top_left_to_bottom_right && bottom_left_to_top_right;

                if case_1 || case_2 || case_3 || case_4 {
                    count += 1;
                }
            }
        }
    }

    part_1();
    println!("part 2 {:?}", count);

}

fn part_1() {
    let raw_data = fs::read_to_string("../../input-day-4.txt").expect("bad input data");
    let needle = "XMAS";

    // Parse the matrix
    let matrix: Vec<Vec<char>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut going_forward = Vec::new();
    let mut going_backwards = Vec::new();
    let mut going_down = Vec::new();
    let mut going_up = Vec::new();
    let mut going_diagonal = Vec::new();

    let rows = matrix.len();
    let cols = matrix.get(0).map(|m| m.len()).unwrap_or(0);

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

    // Diagonal directions ARE SO DAMN CONFUSING TO THINK ABOUT
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
    counts += count_matches_p1(&going_forward, needle);
    counts += count_matches_p1(&going_backwards, needle);
    counts += count_matches_p1(&going_down, needle);
    counts += count_matches_p1(&going_up, needle);
    counts += count_matches_p1(&going_diagonal, needle);

    println!("Part 1 {:?}", counts);
}

fn count_matches_p1(lines: &[String], needle: &str) -> usize {
    lines.iter().map(|line| line.match_indices(needle).count()).sum()
}

// 96416 is NOT the answer




pub mod boilerplate;
use std::collections::HashMap;
// use std::cmp::Reverse;
// use std::collections::BinaryHeap;
use std::collections::VecDeque;
 use std::collections::HashSet;

fn main() {
    let raw_data = crate::boilerplate::get_sample_if_no_input();
    if let Err(ref problem) = raw_data {
        println!("Could not read input data: {:?}", problem);
        return;
    }
    let data = raw_data.unwrap();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &str) {
    let (matrix, start_pos, _) = parse_data_to_graph(data);
    // Part 1 isn't asking for cheats along the best path, it's asking for ANY cheat that saves 100 seconds.
    // ANYWHERE.
    // So instead of djikstra, just BFS from the start and fill up each path bit with a count of how many
    // steps it takes to get there. THEN, figure out where you can nix the wall and you can hop to another
    // region that would have been 100 away.
    let in_bounds = |row: usize, col: usize| -> bool {
        let within_row = row < matrix.len();
        let within_col = col < matrix[row].len();
        within_row && within_col
    };
    let direction: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut step_counts_by_zone = Vec::new();
    for row in &matrix {
        let mut cols = Vec::new();
        for _ in 0..row.len() {
            cols.push(i64::MAX);
        }
        step_counts_by_zone.push(cols);
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut distance_to_get_to: i64 = 0;
    queue.push_back(start_pos);
    step_counts_by_zone[start_pos.0][start_pos.1] = 0;
    while let Some(current_pos) = queue.pop_front() {
        if visited.contains(&current_pos) {
            continue;
        }

        visited.insert(current_pos);
        distance_to_get_to += 1;

        for dir in &direction {
            let next_row = (current_pos.0 as isize + dir.0) as usize;
            let next_col = (current_pos.1 as isize + dir.1) as usize;
            if !in_bounds(next_row, next_col) {
                continue;
            }

            if matrix[next_row][next_col] == NodeType::Wall {
                continue;
            }

            if !visited.contains(&(next_row, next_col)) {
                step_counts_by_zone[next_row][next_col] = distance_to_get_to;
            }
            queue.push_front((next_row, next_col));
        }
    }
    
    // Ok, we have a giant matrix of step counts to path. Check for cheats.
    // Ignore first row/column and last to not consider walls.
    let mut number_of_cheats_saving_time = 0;
    let at_least_this_much = 100;
    let mut cheats_by_saved = HashMap::new();
    for row in 1..matrix.len() - 1 {
        for col in 1..matrix[row].len() - 1 {
            if matrix[row][col] == NodeType::Wall {
                let south_row = (row as isize - 1) as usize;
                let north_row = (row as isize + 1) as usize;
                let east_col =  (col as isize + 1) as usize;
                let west_col =  (col as isize - 1) as usize;

                // Check horizontal cheat across this wall from w -> e
                if matrix[row][west_col] != NodeType::Wall && matrix[row][east_col] != NodeType::Wall {
                    let west_count = step_counts_by_zone[row][west_col];
                    let east_count   = step_counts_by_zone[row][east_col];
                    let diff_in_steps = west_count - east_count;
                    cheats_by_saved.entry(diff_in_steps.abs() - 2).and_modify(|c| *c += 1).or_insert(1);
                    // -2 because we dont count the two steps we'll take as part of the saved time
                    if diff_in_steps.abs() - 2 >= at_least_this_much {
                        number_of_cheats_saving_time += 1
                    }
                }
                // Check vert
                if matrix[south_row][col] != NodeType::Wall && matrix[north_row][col] != NodeType::Wall {
                    let south_count = step_counts_by_zone[south_row][col];
                    let north_count = step_counts_by_zone[north_row][col];
                    let diff_in_steps = south_count - north_count;
                    // -2 because we dont count the two steps we'll take as part of the saved time
                    cheats_by_saved.entry(diff_in_steps.abs() - 2).and_modify(|c| *c += 1).or_insert(1);
                    if diff_in_steps.abs() - 2 >= at_least_this_much {
                        number_of_cheats_saving_time += 1
                    }
                }
            }
        }
    }
    
    // sample should say 2 + 4 + 6 + 8 + 10+ 12+ 20+ 36+ 38+ 40+ 64 = 240
    // too high 11278, 5639
    // too low 702, not right 10633, not right 7043
    let mut huh = 0;
    for (key, count) in &cheats_by_saved {
        if *key >= at_least_this_much {
            huh += count;
        }
    }
    println!("Part 1: {:?} {:?}", huh, number_of_cheats_saving_time);
}

fn part_2(data: &str) {
    // Ok this sucks. But... let's do the same thing as before, where
    // we build up an initial cost matrix that tells us much we save
    // from moving from point x to poiny y.
    let (matrix, start_pos, _) = parse_data_to_graph(data);
        let in_bounds = |row: usize, col: usize| -> bool {
        let within_row = row < matrix.len();
        if !within_row {
            return false;
        }
        let within_col = col < matrix[row].len();
        within_row && within_col
    };
    let direction: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];


    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut the_golden_path = vec![];
    queue.push_back(start_pos);
    the_golden_path.push(start_pos);
    while let Some(current_pos) = queue.pop_front() {
        if visited.contains(&current_pos) {
            continue;
        }

        visited.insert(current_pos);

        for dir in &direction {
            let next_row = (current_pos.0 as isize + dir.0) as usize;
            let next_col = (current_pos.1 as isize + dir.1) as usize;
            if !in_bounds(next_row, next_col) {
                continue;
            }

            if matrix[next_row][next_col] == NodeType::Wall {
                continue;
            }

            if visited.contains(&(next_row, next_col)) {
                continue;
            }
            the_golden_path.push((next_row, next_col));
            queue.push_front((next_row, next_col));
        }
    }
    let mut valid_cheats = 0;
    // let mut cheats_by_time = HashMap::new();
    for current_index in 0..the_golden_path.len() {
        let current_node = the_golden_path[current_index];

        let offset = current_index + 1;
        for check_against_index in offset..the_golden_path.len() {

            let compare_node = the_golden_path[check_against_index];
            let manhatten = (current_node.0 as i64 - compare_node.0 as i64).abs() + (current_node.1 as i64 - compare_node.1 as i64).abs();
            let time_savings = check_against_index  as i64 - current_index  as i64 - manhatten;
            if manhatten <= 20 && time_savings >= 100 {
                // cheats_by_time.entry(time_savings).and_modify(|c| *c += 1).or_insert(1);
                valid_cheats += 1;
            }
        }
    }
    // sample has 285 cheats that save 50ps or more.
    // println!("{:?}", cheats_by_time);
    println!("{:?}", valid_cheats);
}


type Matrix = Vec<Vec<NodeType>>;
type Position = (usize, usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum NodeType {
    Start,
    End,
    Path,
    Wall
}



fn parse_data_to_graph(data: &str) -> (Matrix, Position, Position) {
    let matrix = data.lines().map(|line| {
        line.chars().map(|char_in_line| {
            match char_in_line {
                '.' => NodeType::Path,
                'S' => NodeType::Start,
                'E' => NodeType::End,
                _   => NodeType::Wall,
            }
        }).collect::<Vec<NodeType>>()
    }).collect::<Vec<Vec<NodeType>>>();

    let mut graph = Vec::new();
    let mut start_node = (0,0);
    let mut end_node = (0,0);
    for (row, cols) in matrix.iter().enumerate() {
        let mut r = Vec::new();
        for (col, &node) in cols.iter().enumerate() {
            if node == NodeType::Start {
                start_node = (row, col);
            }

            if node == NodeType::End {
                end_node = (row, col);
            }
            r.push(matrix[row][col]);
        }
        graph.push(r);
    }
    
    (graph, start_node, end_node)
}

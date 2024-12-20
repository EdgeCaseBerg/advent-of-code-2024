pub mod boilerplate;
// use std::collections::HashMap;
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
    let (matrix, start_pos, end_pos) = parse_data_to_graph(data);
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
    for row in 0..matrix.len() {
        let mut cols = Vec::new();
        for col in 0..matrix[row].len() {
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
    let at_least_this_much = 0;
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
                    let east_count   = step_counts_by_zone[row][west_col];
                    // cheats are distinct by start to end position. So east to west and west to east are different.
                    if west_count - east_count >= at_least_this_much {
                        number_of_cheats_saving_time += 1
                    }
                    if east_count - west_count >= at_least_this_much {
                        number_of_cheats_saving_time += 1
                    }
                }
                // Check vert
                if matrix[south_row][col] != NodeType::Wall && matrix[north_row][col] != NodeType::Wall {
                    let south_count = step_counts_by_zone[south_row][col];
                    let north_count = step_counts_by_zone[north_row][col];
                    // cheats are distinct by start to end position.
                    if south_count - north_count >= at_least_this_much {
                        number_of_cheats_saving_time += 1
                    }
                    if north_count - south_count >= at_least_this_much {
                        number_of_cheats_saving_time += 1
                    }
                }
            }
        }
    }
    
    // sample should say 2 + 4 + 6 + 8 + 10+ 12+ 20+ 36+ 38+ 40+ 64 = 240
    // too high 11278, 5639
    // too low 702, not right 10633
    println!("Part 1: {:?}", number_of_cheats_saving_time);
}

fn part_2(data: &str) {
    let _foo = data;
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

fn apply_cheat(matrix: &Matrix, from_position: Position) -> Vec<Matrix> {
    let direction: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let in_bounds = |row: usize, col: usize| -> bool {
        let within_row = row < matrix.len();
        let within_col = col < matrix[row].len();
        within_row && within_col
    };

    let mut matrices = vec![];
    for dir in &direction {
        let cheat_row_start = (from_position.0 as isize + dir.0) as usize;
        let cheat_col_start = (from_position.1 as isize + dir.1) as usize;

        let cheat_row_end = (from_position.0 as isize + dir.0 * 2) as usize;
        let cheat_col_end = (from_position.1 as isize + dir.1 * 2) as usize;

        if !in_bounds(cheat_row_start, cheat_row_end) || !in_bounds(cheat_row_end, cheat_col_end) {
            continue;
        }

        if matrix[cheat_row_start][cheat_col_start] != NodeType::Wall {
            continue;
        }

        if matrix[cheat_row_end][cheat_col_end] != NodeType::Path && matrix[cheat_row_end][cheat_col_end] != NodeType::End {
            continue;
        }

        let mut cheat_worldview = matrix.clone();
        cheat_worldview[cheat_row_start][cheat_col_start] = NodeType::Path;
        matrices.push(cheat_worldview);
    }
    matrices
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

// #[derive(Debug)]
// struct DijkstraResult {
//     distances: HashMap<Position, usize>, // Distance to each position
//     prev: HashMap<Position, Option<Position>>, // Previous node in the path
//     cheats_beating_min_time: u64,
// }

// fn dijkstra(matrix: &Matrix, start: Position, end: Position, cheat_position: Option<Position>, time_to_beat: u64) -> Option<DijkstraResult> {
//     let rows = matrix.len();
//     let cols = matrix[0].len();
    
//     let mut distances: HashMap<Position, usize> = HashMap::new();
//     let mut prev: HashMap<Position, Option<Position>> = HashMap::new();
//     let mut heap = BinaryHeap::new(); // Priority queue, use `Reverse` for a min-heap
    
//     // Initialize distances and the heap
//     for r in 0..rows {
//         for c in 0..cols {
//             let pos = (r, c);
//             distances.insert(pos, usize::MAX);
//             prev.insert(pos, None);
//         }
//     }
    
//     // Start position distance is 0
//     distances.insert(start, 0);
//     heap.push(Reverse((0, start))); // Push (distance, position) into the heap
    
//     // Helper function to get neighbors
//     let neighbors = |pos: Position| -> Vec<Position> {
//         let (r, c) = pos;
//         let mut result = vec![];
//         if r > 0 { result.push((r - 1, c)); } // Up
//         if r < rows - 1 { result.push((r + 1, c)); } // Down
//         if c > 0 { result.push((r, c - 1)); } // Left
//         if c < cols - 1 { result.push((r, c + 1)); } // Right
//         result
//     };
    
//     // Main loop
//     while let Some(Reverse((current_distance, current_position))) = heap.pop() {
//         // If we reach the end, stop
//         if current_position == end {
//             break;
//         }
        
//         // Skip if this distance is outdated
//         if current_distance > distances[&current_position] {
//             continue;
//         }


//         if cheat_position.is_some() && cheat_position.unwrap() == current_position {
//             let must_save_at_least = 100;
//             let new_matrices = apply_cheat(&matrix, current_position);
//             let mut number_of_cheats_saving_time_at_this_position = 0;
//             if new_matrices.len() > 0 {
//                 for cheated_matrix in new_matrices {
//                     let cheat_result = dijkstra(&cheated_matrix, start, end, None, time_to_beat).unwrap();
//                     let time = cheat_result.distances[&end];
//                     if (time as u64) < time_to_beat && time_to_beat - time as u64 <= must_save_at_least {
//                         println!("save {:?}", time_to_beat - time as u64);
//                         number_of_cheats_saving_time_at_this_position += 1
//                     }
//                 }
//             }
//             return Some(DijkstraResult { distances, prev, cheats_beating_min_time: number_of_cheats_saving_time_at_this_position });
//         }
        
//         // Explore neighbors
//         for neighbor in neighbors(current_position) {
//             if let NodeType::Wall = matrix[neighbor.0][neighbor.1] {
//                 continue; // Skip walls
//             }
            
//             let tentative_distance = current_distance + 1; // Distance to the neighbor
            
//             // If this path is shorter, update it
//             if tentative_distance < distances[&neighbor] {
//                 distances.insert(neighbor, tentative_distance);
//                 prev.insert(neighbor, Some(current_position));
//                 heap.push(Reverse((tentative_distance, neighbor)));
//             }
//         }
//     }
    
//     // If we never reach the end, return None
//     if distances[&end] == usize::MAX {
//         return None;
//     }
    
//     Some(DijkstraResult { distances, prev, cheats_beating_min_time: 0 })
// }


pub mod boilerplate;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
    // First let's find the length of the best path so we know how many seconds we would save.

    let no_cheating_result = dijkstra(&matrix, start_pos, end_pos).unwrap();
    println!("Time to beat is {:?}", no_cheating_result.distances[&end_pos]);

    
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

#[derive(Debug)]
struct DijkstraResult {
    distances: HashMap<Position, usize>, // Distance to each position
    prev: HashMap<Position, Option<Position>>, // Previous node in the path
}

fn dijkstra(matrix: &Matrix, start: Position, end: Position) -> Option<DijkstraResult> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    
    let mut distances: HashMap<Position, usize> = HashMap::new();
    let mut prev: HashMap<Position, Option<Position>> = HashMap::new();
    let mut heap = BinaryHeap::new(); // Priority queue, use `Reverse` for a min-heap
    
    // Initialize distances and the heap
    for r in 0..rows {
        for c in 0..cols {
            let pos = (r, c);
            distances.insert(pos, usize::MAX);
            prev.insert(pos, None);
        }
    }
    
    // Start position distance is 0
    distances.insert(start, 0);
    heap.push(Reverse((0, start))); // Push (distance, position) into the heap
    
    // Helper function to get neighbors
    let neighbors = |pos: Position| -> Vec<Position> {
        let (r, c) = pos;
        let mut result = vec![];
        if r > 0 { result.push((r - 1, c)); } // Up
        if r < rows - 1 { result.push((r + 1, c)); } // Down
        if c > 0 { result.push((r, c - 1)); } // Left
        if c < cols - 1 { result.push((r, c + 1)); } // Right
        result
    };
    
    // Main loop
    while let Some(Reverse((current_distance, current_position))) = heap.pop() {
        // If we reach the end, stop
        if current_position == end {
            break;
        }
        
        // Skip if this distance is outdated
        if current_distance > distances[&current_position] {
            continue;
        }
        
        // Explore neighbors
        for neighbor in neighbors(current_position) {
            if let NodeType::Wall = matrix[neighbor.0][neighbor.1] {
                continue; // Skip walls
            }
            
            let tentative_distance = current_distance + 1; // Distance to the neighbor
            
            // If this path is shorter, update it
            if tentative_distance < distances[&neighbor] {
                distances.insert(neighbor, tentative_distance);
                prev.insert(neighbor, Some(current_position));
                heap.push(Reverse((tentative_distance, neighbor)));
            }
        }
    }
    
    // If we never reach the end, return None
    if distances[&end] == usize::MAX {
        return None;
    }
    
    Some(DijkstraResult { distances, prev })
}
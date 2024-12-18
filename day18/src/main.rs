pub mod boilerplate;

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

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
    let rows = 71; // 7 for example, 70 for real
    let cols = 71; // 7 for example, 70 for real
    let steps_to_simulate = 1024;
    let graph = get_empty_matrix(rows, cols);
    let obstacles = get_obstacles(data);
    let graph = place_obstacles_on_graph(&graph, &obstacles, steps_to_simulate);
    let start = (0,0);
    let end = (70,70);
    print_graph(&graph);
    let steps = do_dijkstra(graph, start, end);
    println!("Part 1 {:?}", steps);
}

fn part_2(data: &str) {
    let rows = 71; // 7 for example, 70 for real
    let cols = 71; // 7 for example, 70 for real
    let steps_to_simulate = 1024;
    let graph = get_empty_matrix(rows, cols);
    let obstacles = get_obstacles(data);
    for step_no in steps_to_simulate..obstacles.len() {
        let corrupted_graph = place_obstacles_on_graph(&graph, &obstacles, step_no);
        let start = (0,0);
        let end = (70,70);
        if dijkstra(&corrupted_graph, start, end).is_none() {
            println!("step_no: {}", step_no);
            break;
        }
    }
    println!("Part 2 done I guess");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NodeType {
    Open,
    Corrupted,
    Start,
    End
}

type Matrix = Vec<Vec<NodeType>>;
type Position = (usize, usize);
fn get_empty_matrix(rows: usize, cols: usize) -> Matrix {
    let mut graph = Vec::new();
    for _ in 0..rows {
        let mut the_row = Vec::new();
        for _ in 0..cols {
            the_row.push(NodeType::Open);
        }
        graph.push(the_row);
    }
    graph
}

fn get_obstacles(data: &str) -> Vec<Position> {
    data.lines().map(|line| {
        let pair: Vec<&str> = line.split(",").collect();
        let x: usize = pair[0].to_string().parse().unwrap();
        let y: usize = pair[1].to_string().parse().unwrap();
        (y, x)
    }).collect()
}

fn place_obstacles_on_graph(graph: &Matrix, obstacles: &Vec<Position>, max_to_place: usize) -> Matrix {
    let mut updated = graph.clone();
    let mut to_place = obstacles.iter().take(max_to_place);
    while let Some(corruption) = to_place.next() {
        let (row, col) = corruption;
        updated[*row][*col] = NodeType::Corrupted;
    }
    updated
}

fn print_graph(graph: &Matrix) {
    for (_, row) in graph.iter().enumerate() {
        for (_, node) in row.iter().enumerate() {
            let to_print = match node {
                NodeType::Corrupted => "#",
                _ => "."
            };
            print!("{}", to_print);
        }
        println!();
    }
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
            if let NodeType::Corrupted = matrix[neighbor.0][neighbor.1] {
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

fn do_dijkstra(matrix: Matrix, start: Position, end: Position) {
    if let Some(result) = dijkstra(&matrix, start, end) {
        println!("Shortest distance to end: {:?}", result.distances[&end]);
    } else {
        println!("No path found!");
    }
}

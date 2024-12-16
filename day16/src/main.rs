pub mod boilerplate;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Ordering;             


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
    let (graph, start, target) = parse_data_to_graph(data);

    // Begin A* search.
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<((usize, usize),&(isize, isize)), State> = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    let east = (0, 1);
    let north = (-1, 0);
    let west = (0, -1);
    let south = (1, 0);
    let direction = vec![east, north, west, south];

    let in_bounds = |row: usize, col: usize| -> bool {
        let within_row = row < graph.len();
        let within_col = col < graph[row].len();
        within_row && within_col
    };

    let weight_to = |facing: (isize, isize), neighbor: (isize, isize)| -> i32 {
        let row_diff = facing.0 - neighbor.0;
        let col_diff = facing.1 - neighbor.1;

        match (row_diff, col_diff) {
            (0, 0) => 1, // Same dir moving forward is cheap
            (1, _) => 1001,
            (-1,_) => 1001,
            _ => 2001, // 2000 to move backwards
        }
    };

    g_score.insert((start, &east), 0);
    f_score.insert((start, &east), 0);
    open_set.push(State { cost: 0, position: start, facing: east });
    let mut result_path = VecDeque::new();
    while let Some(current) = open_set.pop() {
        if current.position == target {
            // Break out of loop and then construct path from the set we've built
            let mut total_path = VecDeque::new();
            total_path.push_front(current);

            let mut key = (current.position, &current.facing);
            loop {

                if !came_from.contains_key(&key) {
                    break;
                }
                let current = came_from.get(&key).unwrap();
                key = (current.position, &current.facing);
                total_path.push_front(*current);
            }
            
            result_path = total_path;
            break;
        }

        
        for dir in &direction {
            let neighbor = ((current.position.0 as isize + dir.0) as usize, (current.position.1 as isize + dir.1) as usize);
            if !in_bounds(neighbor.0, neighbor.1) {
                continue;
            }
            if graph[neighbor.0][neighbor.1] == NodeType::Wall {
                continue;
            }

            let neighbor_key = (neighbor, dir);
            let current_key = (current.position, &current.facing);
            let tentative_score = g_score.get(&current_key).unwrap() + weight_to(current.facing, *dir); // <-- I think this is wrong? d() vs h()
            if tentative_score < *g_score.get(&neighbor_key).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor_key, current);
                g_score.insert(neighbor_key, tentative_score);
                f_score.insert(neighbor_key, tentative_score + weight_to(current.facing, *dir));

                let neighbor_in_set = open_set.iter().find(|state| state.position == neighbor_key.0 && state.facing == *dir);
                if neighbor_in_set.is_none() {
                    open_set.push(State { cost: tentative_score + weight_to(current.facing, *dir), facing: *dir, position: neighbor })
                }
            }
        }
    }

    while let Some(state) = result_path.pop_front() {
        println!("{:?}", state);
    }
}

fn part_2(data: &str) {
    let _foo = data;
    // Not 445, it's not just the total path length of the traversal there.
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum NodeType {
    Start,
    End,
    Path,
    Wall
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
     cost: i32,
     position: (usize, usize),
     facing: (isize, isize)
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.facing.cmp(&other.facing))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn parse_data_to_graph(data: &str) -> (Vec<Vec<NodeType>>, (usize, usize), (usize, usize)) {
    let matrix = data.lines().map(|line| {
        line.chars().filter_map(|char_in_line| {
            match char_in_line {
                '.' => Some(NodeType::Path),
                'S' => Some(NodeType::Start),
                'E' => Some(NodeType::End),
                _   => Some(NodeType::Wall),
            }
        }).collect::<Vec<NodeType>>()
    }).collect::<Vec<Vec<NodeType>>>();

    let mut graph = Vec::new();
    let mut start_node = (0,0);
    let mut end_node = (0,0);
    for row in 0..matrix.len() {
        let mut r = Vec::new();
        for col in 0..matrix[row].len() {
            if matrix[row][col] == NodeType::Start {
                start_node = (row, col);
            }

            if matrix[row][col] == NodeType::End {
                end_node = (row, col);
            }
            r.push(matrix[row][col]);
        }
        graph.push(r);
    }
    
    (graph, start_node, end_node)
}
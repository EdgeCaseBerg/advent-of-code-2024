pub mod boilerplate;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashSet;
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
    let mut came_from: HashMap<(Position, &(isize, isize)), State> = HashMap::new();
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

    let mut state_r = None;
    while let Some(state) = result_path.pop_front() {
        state_r = Some(state);
    }

    println!("The answer for part 1 is {:?}", state_r.unwrap().cost - 1);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct P2HeapOrder {
     cost: u64,
     key: (usize, usize, (isize, isize))
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for P2HeapOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.key.cmp(&other.key))
    }
}

impl PartialOrd for P2HeapOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_2(data: &str) {
    let (graph, start, target) = parse_data_to_graph(data);
    let in_bounds = |row: usize, col: usize| -> bool {
        let within_row = row < graph.len();
        let within_col = col < graph[row].len();
        within_row && within_col
    };

    let east = (0, 1);
    let north = (-1, 0);
    let west = (0, -1);
    let south = (1, 0);
    let directions = vec![east, north, west, south];

    // Let's do dijkstra this time instead.
    let mut distances_by_location_and_dir = HashMap::new();
    let mut previous_nodes_into_location = HashMap::new();
    let mut queue     = BinaryHeap::new();

    // First precompute all distances as infinitely far
    for row in 0..graph.len() {
        for col in 0..graph[row].len() {
            if graph[row][col] == NodeType::Wall{
                continue;
            }

            for dir in &directions {
                // I shouldn't technically have to do this since Out of bounds is beyond the wall, but let's be safe.
                if !in_bounds((row as isize + dir.0) as usize, (col as isize + dir.1) as usize) {
                    continue;
                }

                if graph[(row as isize + dir.0) as usize][(col as isize + dir.0) as usize] == NodeType::Wall {
                    continue;
                }

                let node_key = (row, col, *dir);
                let empty_set: HashSet<(usize, usize, (isize, isize))> = HashSet::new();
                
                distances_by_location_and_dir.insert(node_key, u64::MAX);
                previous_nodes_into_location.insert(node_key, empty_set);

                // Setup our starting point as the most minimal of distance.
                // And also add in the min heap queue for processing
                // Reindeer start facing _east_ only
                if (row, col) == start && *dir == east {
                    distances_by_location_and_dir.insert(node_key, 0);
                    queue.push(P2HeapOrder { cost: 0, key: node_key });
                } else {
                    queue.push(P2HeapOrder { cost: u64::MAX, key: node_key });
                }
            }
        }
    }

    while let Some(next_node) = queue.pop() {
        let P2HeapOrder { cost: _, key: node_key } = next_node;
        let (row, col, facing) = node_key;

        if (row, col) == target {
            break;
        }

        for dir in &directions {
            let next_row = (row as isize + dir.0) as usize;
            let next_col = (col as isize + dir.1) as usize;

            if !in_bounds(next_row, next_col) {
                continue;
            }
            if graph[next_row][next_col] == NodeType::Wall {
                continue;
            }

            let mut distance_to_next: u64 = *(distances_by_location_and_dir.get(&node_key).unwrap_or(&u64::MAX));
            if facing == *dir {
                distance_to_next = distance_to_next.saturating_add(1);    // just step forward
            } else {
                distance_to_next = distance_to_next.saturating_add(1001); // turn then step
            }

            let neighbor_key = (next_row, next_col, *dir);
            if distance_to_next <= *(distances_by_location_and_dir.get(&neighbor_key).unwrap_or(&u64::MAX)) {
                let mut hash_set = (previous_nodes_into_location.get(&neighbor_key).unwrap_or(&HashSet::new())).clone();
                hash_set.insert(node_key);
                previous_nodes_into_location.insert(neighbor_key, hash_set); // mark the neighbor as being traversale to from our current node
                distances_by_location_and_dir.insert(neighbor_key, distance_to_next);
                queue.push(P2HeapOrder { cost: distance_to_next,  key: neighbor_key });
            }
        }
    }

    let mut cost = u64::MAX;
    let mut best_direction = (start.0, start.1, east);
    for dir in &directions {
        if let Some(distance) = distances_by_location_and_dir.get(&(target.0, target.1, *dir)) {
            if *distance < cost {
                cost = *distance;
                best_direction = (target.0, target.1, *dir);
            } else if *distance == cost {
                println!("Potential tie?");
            }
        }
    }
    println!("Best cost of {:?} from {:?}", cost, best_direction);

    // Now we use the fact that we stored a Set of previous nodes as we traversed to construct
    // a graph that is the best path AND any other paths that may have been possible for it.
    let mut unique = HashSet::new();
    unique.insert((best_direction.0, best_direction.1));

    // So DFS from the End location all the way back to the start
    let mut stack = VecDeque::new();
    stack.push_front(best_direction); //FILO, LILO hm... fun.
    while let Some(current) = stack.pop_back() {
        match previous_nodes_into_location.get(&current) {
            None => {
                // Continue... unless it's the source?
                continue;
            }
            Some(previous_set_of_nodes) => {
                for node_key in previous_set_of_nodes { 
                    unique.insert((current.0, current.1));
                    stack.push_front(*node_key);
                }
            }
        }
    }

    // + 1 becuase we never count the actual start node.
    println!("The number of unique tiles is {:?}", unique.len() + 1);

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

// Taken and modified from https://doc.rust-lang.org/nightly/std/collections/binary_heap/index.html
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

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
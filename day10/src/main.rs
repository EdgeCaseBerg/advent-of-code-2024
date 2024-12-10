use std::fs;
use std::collections::{HashSet, VecDeque};

fn main() {
    // a hiking trail is any path that starts at height 0, 
    // ends at height 9, and always increases by a height
    // of exactly 1 at each step. Hiking trails never include diagonal steps - only up, down, left, or right
    let raw_data = fs::read_to_string("../input.txt").expect("bad input data");
    let raw_data = raw_data.as_str();    
    let matrix = make_matrix(raw_data);
    let trail_scores =  find_trailhead_scores(matrix);
    println!("Part 1: {:?}", trail_scores.into_iter().reduce(|a, s| a + s));
}


fn find_trailhead_scores(grid: Vec<Vec<u8>>) -> Vec<usize> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut trailhead_scores = Vec::new();

    // Directions for moving up, down, left, and right
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Helper function to check if a position is valid
    let is_valid = |x: isize, y: isize| -> bool {
        x >= 0 && y >= 0 && (x as usize) < rows && (y as usize) < cols
    };

    // BFS function to find reachable '9' positions from a given trailhead
    let bfs = |start_x: usize, start_y: usize| -> HashSet<(usize, usize)> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut reachable_nines = HashSet::new();

        queue.push_back((start_x, start_y));
        visited.insert((start_x, start_y));

        while let Some((x, y)) = queue.pop_front() {
            let current_height = grid[x][y];

            for (dx, dy) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if is_valid(nx, ny) {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    let next_height = grid[nx][ny];
                    if !visited.contains(&(nx, ny)) && next_height == current_height + 1 {
                        visited.insert((nx, ny));
                        queue.push_back((nx, ny));

                        if next_height == 9 {
                            reachable_nines.insert((nx, ny));
                        }
                    }
                }
            }
        }

        reachable_nines
    };

    // Find all trailheads (positions with height 0)
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                let reachable = bfs(i, j);
                trailhead_scores.push(reachable.len());
            }
        }
    }

    trailhead_scores
}

fn make_matrix(raw_data: &str) -> Vec<Vec<u8>> {
    // Parse the matrix
    let matrix: Vec<Vec<u8>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars().map(|c| {
                let n: u8 = c.to_string().parse().unwrap();
                n
            }).collect()
        })
        .collect();
    matrix
}
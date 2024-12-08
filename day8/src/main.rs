use std::fs;
use std::collections::HashSet;

fn main() {
    let raw_data = fs::read_to_string("../input.txt").expect("bad input data");
    let raw_data = raw_data.as_str();    
    let (matrix, rows, cols) = make_matrix(raw_data);
    let antinodes = create_antinodes(matrix, rows, cols);
    let mut count_of_antinodes = 0;
    for row in antinodes {
        for maybe_node in row {
            if maybe_node.is_some() {
               // print!("{:?} ", maybe_node);
               count_of_antinodes += 1;
            }
            
        }
        // println!("")
    }
    // your answer it too low:  221
    // your answer is too high: 456
    println!("{:?}", count_of_antinodes);
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Antenna {
    frequency: char,
}

#[derive(Debug)]
struct AntiNode {
    known_locations: HashSet<AntennaLocation>
}

impl AntiNode {
    fn with(a: AntennaLocation) -> AntiNode {
        let mut node = AntiNode {
            known_locations: HashSet::new()
        };
        node.add_node(a);
        node
    }
    fn add_node(&mut self, a: AntennaLocation) {
        self.known_locations.insert(a);
    }
    fn size(&self) -> usize {
        self.known_locations.len()
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct AntennaLocation {
    row: usize,
    col: usize,
    antenna: Antenna,
}

impl AntennaLocation {
    fn distance_to(&self, to_row: usize, to_col: usize) -> usize {
        let row_distance = (self.row as isize) - (to_row) as isize;
        let col_distance = (self.col as isize) - (to_col) as isize;
        let distance = row_distance.abs() + col_distance.abs();
        distance as usize
    }
    fn new(antenna: Antenna, row: usize, col: usize) -> AntennaLocation {
        AntennaLocation {
            row, col, antenna
        }
    }
}

impl Antenna {
    fn new(frequency: char) -> Antenna {
        Antenna {
            frequency
        }
    }
    fn find_same_frequency(&self, map: &Vec<Vec<Option<Antenna>>>, rows: usize, cols: usize) -> Vec<AntennaLocation> {
        let mut friends = Vec::new();
        for row in 0..rows {
            for col in 0..cols {
                if let Some(other) = &map[row][col] {
                    if other.frequency == self.frequency {
                        friends.push(
                            AntennaLocation::new(
                                other.clone(),
                                row,
                                col
                            )
                        );
                    }
                }
            }
        }
        friends
    }
}

fn make_matrix(raw_data: &str) -> (Vec<Vec<Option<Antenna>>>, usize, usize) {
    // Parse the matrix
    let matrix: Vec<Vec<Option<Antenna>>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars().map(|c| {
                if c == '.' {
                    None
                } else {
                    Some(Antenna::new(c))
                }
            }).collect()
        })
        .collect();

    let rows = matrix.len();
    let cols = matrix.get(0).map(|m| m.len()).unwrap_or(0);

    (matrix, rows, cols)
}

fn make_antinode_matrix(rows: usize, cols: usize) -> Vec<Vec<Option<AntiNode>>> {
    let mut m = Vec::new();
    for _ in 0..rows {
        let mut r = Vec::new();
        for _ in 0..cols {
            r.push(None);
        }
        m.push(r);
    }
    m
}


fn create_antinodes(map: Vec<Vec<Option<Antenna>>>, rows: usize, cols: usize) -> Vec<Vec<Option<AntiNode>>> {
    let mut antinodes = make_antinode_matrix(rows, cols);
    for row in 0..rows {
        for col in 0..cols {
            if let Some(antenna) = &map[row][col] {
                let others = antenna.find_same_frequency(&map, rows, cols);
                for other in others {
                    let d = other.distance_to(row,col);
                    if d % 2  == 0 {
                        // an antinode occurs at any point that is perfectly in line 
                        // with two antennas of the same frequency - but only when one
                        // of the antennas is twice as far away as the other
                        if row + d < rows && col + d < cols {
                            let location = AntennaLocation::new(
                                antenna.clone(),
                                row + d,
                                col + d
                            );
                            match &antinodes[row][col] {
                                None => {
                                    antinodes[row][col] = Some(AntiNode::with(location));
                                }
                                Some(node) => {
                                    let known = node.known_locations.clone();
                                    let mut antinode = AntiNode::with(location);
                                    for n in known {
                                        antinode.add_node(n);
                                    }
                                    antinodes[row][col] = Some(antinode);
                                }
                            }
                        }
                        if (row as isize) - d as isize > 0 && (col as isize) - d as isize > 0 {
                             let location = AntennaLocation::new(
                                antenna.clone(),
                                row - d,
                                col - d
                            );
                            match &antinodes[row][col] {
                                None => {
                                    antinodes[row][col] = Some(AntiNode::with(location));
                                }
                                Some(node) => {
                                    let known = node.known_locations.clone();
                                    let mut antinode = AntiNode::with(location);
                                    for n in known {
                                        antinode.add_node(n);
                                    }
                                    antinodes[row][col] = Some(antinode);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    antinodes
}
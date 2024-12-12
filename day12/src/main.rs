use std::fs;
use std::error::Error;
use std::env;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn Error>> {
    // let maybe_filename = get_filename_from_args();
    let maybe_filename = Some(String::from("../sample1.txt"));
    if maybe_filename.is_none() {
        return Err("No file provided".into());
    }
    let input: String = fs::read_to_string(maybe_filename.unwrap())?;
    let plants = make_matrix(&input);
    let regions = find_regions(&plants);
    println!("{:?}", regions);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Plant {
    value: char
}

impl Plant {
    fn new(value: char) -> Plant {
        Plant { value }
    }
}

#[derive(Debug, Clone)]
struct Region {
    planted: Plant,
    plants: Vec<(usize, usize)>,
    adjacent: HashSet<(usize, usize)>,
    regions_within_this: Vec<Region>
}

impl Region {
    fn area(&self) -> u64 {
        self.plants.len() as u64
    }
    fn price(&self) -> u64 {
        self.area() * self.perimeter()
    }

    fn of(plant: Plant) -> Region {
        Region {
            planted: plant,
            plants: Vec::new(),
            regions_within_this: Vec::new(),
            adjacent: HashSet::new()
        }
    }

    fn add_plot(&mut self, plot: (usize, usize)) {
        self.plants.push(plot);
    }

    fn add_region(&mut self, region: Region) {
        self.regions_within_this.push(region);
    }

    fn add_boundary_plant(&mut self, at: (usize, usize)) {
        self.adjacent.insert(at);
    }

    fn perimeter(&self) -> u64 {
        0
    }
}


fn get_filename_from_args() -> Option<String> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.is_empty() {
        return None;
    }
    let mut arguments = arguments.iter();
    arguments.next(); // skip the name of the program being ran
    arguments.next().cloned()
}

fn make_matrix(raw_data: &str) -> Vec<Vec<Plant>> {
    // Parse the matrix
    let matrix: Vec<Vec<Plant>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars().map(|c| {
                Plant::new(c)
            }).collect()
        })
        .collect();
    matrix
}

fn find_regions(plots: &Vec<Vec<Plant>>) -> Vec<Region> {
    let rows = plots.len();
    let cols = plots[0].len();

    let mut regions = Vec::new();
    let mut visited_already = HashSet::new();
    for row in 0..rows {
        for col in 0..cols {
            if visited_already.contains(&(row,col)) {
                continue;
            }
            let plant = plots[row][col].clone();
            // Find the plot.
            let mut region = Region::of(plant);
            let region = bfs(&plots, &plant, row, col);
            region.plants.iter().for_each(|coord| {
                visited_already.insert(coord.clone());    
            });
            regions.push(region)
        }
    }
    regions
}

fn bfs(plots: &Vec<Vec<Plant>>, of_type: &Plant, start_row: usize, start_col: usize) -> Region {
    let rows = plots.len();
    let cols = plots[0].len();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let is_same_plant = |row: isize, col: isize| -> bool {
        let within_bounds = row >= 0 && col >= 0 && (row as usize) < rows && (col as usize) < cols;
        if !within_bounds {
            return false;
        }
        let plant_here = plots[row as usize][col as usize];
        plant_here.value == of_type.value
    };

    let mut region = Region::of(of_type.clone());
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start_row, start_col));
    region.add_plot((start_row, start_col));

    while let Some((row, col)) = queue.pop_front() {
        for (d_row, d_col) in &directions {
            let new_row = row as isize + d_row;
            let new_col = col as isize + d_col;
            if is_same_plant(new_row, new_col) && !visited.contains(&(new_row, new_col)) {
                let row = new_row as usize;
                let col = new_col as usize;
                visited.insert((new_row, new_col));
                queue.push_back((row, col));
                region.add_plot((row, col));
            } else {
                region.add_boundary_plant((row,col));
            }
        }
    }
    region
}
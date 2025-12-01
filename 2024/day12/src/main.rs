use std::fs;
use std::error::Error;
use std::env;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let maybe_filename = get_filename_from_args();
    // let maybe_filename = Some(String::from("../input.txt"));
    if maybe_filename.is_none() {
        return Err("No file provided".into());
    }
    let input: String = fs::read_to_string(maybe_filename.unwrap())?;
    let plants = make_matrix(&input);
    let regions = find_regions(&plants);
    let mut cost = 0;
    let mut discount = 0;
    for region in regions {
        cost += region.price();
        discount += region.area() * region.sides();
    }
    println!("Cost {:?}", cost);
    println!("Discount {:?}", discount);

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
    touching: HashSet<(isize, isize)>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn of(row: isize, col: isize) -> Direction {
        match (row, col) {
            (0, 1) => Direction::Right,
            (1, 0) => Direction::Down,
            (0, -1) => Direction::Left,
            (-1, 0) => Direction::Up,
            _ => {
                println!("impossible! {}, {}", row, col);
                Direction::Left
            }
        }
    }
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
            touching: HashSet::new()
        }
    }

    fn add_plot(&mut self, plot: (usize, usize)) {
        self.plants.push(plot);
    }

    fn add_boundary_plant(&mut self, at: (isize, isize)) {
        self.touching.insert(at);
    }

    fn perimeter(&self) -> u64 {
        if self.plants.len() == 1 {
            return 4;
        }
        let mut p = 0;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for plot in self.plants.iter() {
            let mut exposed_edges = 0;
            for (d_row, d_col) in &directions {
                let new_row = plot.0 as isize + d_row;
                let new_col = plot.1 as isize + d_col;
                if self.touching.contains(&(new_row, new_col)) {
                    exposed_edges += 1;
                }
            }
            p += exposed_edges;
        }
        if p == 0 {
            println!("{:?}", self);
        }
        p
    }

    fn sides(&self) -> u64 {
        if self.plants.len() == 1 {
            return 4;
        }
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut sides_and_directions: HashMap<Direction, Vec<(usize, usize)>> = HashMap::new();
        for plot in self.plants.iter() {
            for (d_row, d_col) in &directions {
                let new_row = plot.0 as isize + d_row;
                let new_col = plot.1 as isize + d_col;
                if self.touching.contains(&(new_row, new_col)) {
                    let key = Direction::of(*d_row, *d_col);
                    let mut plots_with_sides_this_way = Vec::new();
                    if sides_and_directions.contains_key(&key) {
                        let mut already_found: Vec<(usize, usize)> = sides_and_directions.get(&key).unwrap().to_vec();
                        already_found.push(*plot);
                        plots_with_sides_this_way = already_found;
                        sides_and_directions.remove(&key);
                        sides_and_directions.insert(key, plots_with_sides_this_way);
                    } else {
                        plots_with_sides_this_way.push(*plot);
                        sides_and_directions.insert(key, plots_with_sides_this_way);
                    }
                    
                }
            }
        }

        let mut sides_across_directions = 0;
        // we now just need to collapse any of the touching bits based on direction.
        for direction in sides_and_directions.keys() {
            match direction {
                Direction::Up | Direction::Down => {
                    match sides_and_directions.get(direction) {
                        None => {}
                        Some(plots_with_side_facing_up) => {
                            let mut by_row = plots_with_side_facing_up.to_vec();
                            by_row.sort_by_key(|(r, _)| *r);
                            // Now we just find any deltas in the col per row to count how many upward facing sides we've got :)
                            let mut sides = 1;
                            let mut current_row = by_row[0].0;
                            let mut col_by_row: HashMap<usize, Vec<usize>> = HashMap::new();
                            for plot in by_row.iter() {
                                if current_row != plot.0 {
                                    sides += 1;
                                    current_row = plot.0;
                                }
                                if col_by_row.contains_key(&plot.0) {
                                    let mut v: Vec<usize> = col_by_row.get(&plot.0).unwrap().to_vec();
                                    v.push(plot.1);
                                    col_by_row.remove(&plot.0);
                                    col_by_row.insert(plot.0, v.to_vec());
                                } else {
                                    col_by_row.insert(plot.0, vec![plot.1]);
                                }
                            }

                            for columns in col_by_row.values() {
                                if columns.len() > 1 {
                                    let mut cols = columns.clone();
                                    cols.sort();
                                    let mut cols = cols.iter();
                                    let mut c1 = cols.next();
                                    loop {
                                        if c1.is_none() {
                                            break
                                        }

                                        let c2 = cols.next();
                                        if c2.is_none() {
                                            break;
                                        }

                                        let c2 = c2.unwrap();

                                        if (*c1.unwrap() as isize - *c2 as isize).abs() != 1 {
                                            sides += 1;
                                        }

                                        c1 = Some(c2);
                                    }
                                }
                            }
                            sides_across_directions += sides;
                        }
                    }
                }
                Direction::Right  | Direction::Left => {
                    match sides_and_directions.get(direction) {
                        None => {}
                        Some(plots_with_side_facing_up) => {
                            let mut by_col = plots_with_side_facing_up.to_vec();
                            by_col.sort_by_key(|(_, c)| *c);
                            // Now we just find any deltas in the col per row to count how many upward facing sides we've got :)
                            let mut sides = 1;
                            let mut current_col = by_col[0].1;
                            let mut row_by_col: HashMap<usize, Vec<usize>> = HashMap::new();
                            for plot in by_col.iter() {
                                if current_col != plot.1 {
                                    sides += 1;
                                    current_col = plot.1;
                                }
                                if row_by_col.contains_key(&plot.1) {
                                    let mut v: Vec<usize> = row_by_col.get(&plot.1).unwrap().to_vec();
                                    v.push(plot.0);
                                    row_by_col.remove(&plot.1);
                                    row_by_col.insert(plot.1, v.to_vec());
                                } else {
                                    row_by_col.insert(plot.1, vec![plot.0]);
                                }
                            }

                            for rows in row_by_col.values() {
                                if rows.len() > 1 {
                                    let mut rows = rows.clone();
                                    rows.sort();
                                    let mut rows = rows.iter();
                                    let mut r1 = rows.next();
                                    loop {
                                        if r1.is_none() {
                                            break
                                        }

                                        let r2 = rows.next();
                                        if r2.is_none() {
                                            break;
                                        }

                                        let r2 = r2.unwrap();

                                        if (*r1.unwrap() as isize - *r2 as isize).abs() != 1 {
                                            sides += 1;
                                        }

                                        r1 = Some(r2);
                                    }
                                }
                            }
                            sides_across_directions += sides;
                        }
                    }
                }
            }
        }

        sides_across_directions
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

fn find_regions(plots: &[Vec<Plant>]) -> Vec<Region> {
    let rows = plots.len();
    let cols = plots[0].len();

    let mut regions = Vec::new();
    let mut visited_already = HashSet::new();
    for row in 0..rows {
        for col in 0..cols {
            if visited_already.contains(&(row,col)) {
                continue;
            }
            let plant = plots[row][col];
            // Find the plot.
            let region = bfs(plots, &plant, row, col);
            region.plants.iter().for_each(|coord| {
                visited_already.insert(*coord);    
            });
            regions.push(region)
        }
    }
    regions
}

fn bfs(plots: &[Vec<Plant>], of_type: &Plant, start_row: usize, start_col: usize) -> Region {
    let rows = plots.len();
    let cols = plots[0].len();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let within_bounds = |row: isize, col: isize| -> bool {
        row >= 0 && col >= 0 && (row as usize) < rows && (col as usize) < cols
    };

    let is_same_plant = |row: isize, col: isize| -> bool {
        if !within_bounds(row, col) {
            return false;
        }
        let plant_here = plots[row as usize][col as usize];
        plant_here.value == of_type.value
    };

    let mut region = Region::of(*of_type);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start_row, start_col));

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
            } else if within_bounds(new_row, new_col) && !is_same_plant(new_row, new_col) {
                region.add_boundary_plant((new_row, new_col));
            } else if !within_bounds(new_row, new_col) {
                region.add_boundary_plant((new_row, new_col));
            }
        }
    }

    if region.plants.is_empty() {
        region.add_plot((start_row, start_col));
    }

    region
}
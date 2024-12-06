use std::fs;

fn main() {
    let raw_data = fs::read_to_string("../sample.txt").expect("bad input data");
    let raw_data = raw_data.as_str();

    // Uncomment this to generate anim.txt file input
    // generate_animation_txt(raw_data);
    // but comment out the rest below when you generate it
    
    let (matrix, rows, cols) = make_matrix(raw_data);
    let mut elven_mischief_possible = 0;
    for row in 0..rows {
        for col in 0..cols {
            if is_guard(&matrix[row][col]) {
                continue;
            }
            // Mark a spot as an obstacle
            let mut fiddle = matrix.clone();
            fiddle[row][col] = '#';
            match watch_guard_or_cycle(fiddle, rows, cols, false) {
                None => elven_mischief_possible += 1,
                Some(wait_time) => {} // println!("Position ({}, {}) lets the guard leave in {:?} steps", row, col, wait_time),
            }
        }
    }
    println!("Elven mischief possible {}", elven_mischief_possible);
}

fn generate_animation_txt(raw_data: &str) {
    let (mut matrix, rows, cols) = make_matrix(raw_data);
    watch_guard_or_cycle(matrix, rows, cols, true);
}

fn print_matrix(matrix: &Vec<Vec<char>>, rows: usize, cols: usize) {
    let mut string = String::new();
    for row in 0..rows {
        for col in 0..cols {
            string.push(matrix[row][col]);
        }
        string.push_str("\n");
    }
    println!("{}", string);
}

// Return None if cycle, number of steps to leave otherwise.
fn watch_guard_or_cycle(mut matrix: Vec<Vec<char>>, rows: usize, cols: usize, print_step: bool) -> Option<i32> {
    let visited_marker = 'X';
    let floor = '.';
    let mut guard_position = (0, 0);
    for row in 0..rows {
        for col in 0..cols {
            if is_guard(&matrix[row][col]) {
                guard_position = (row, col);
            }
        }
    }

    let mut path_marked = matrix.clone();
    let mut obstacles_hit_count = init_cycle_detector(rows, cols);

    loop {
        if print_step {
            print_matrix(&matrix, rows, cols);
        }
        if guard_has_left(rows, cols, guard_position) {
            break;
        }

        let direction = Direction::of(&matrix[guard_position.0][guard_position.1]).unwrap();
        let (newRow, newCol) = match direction {
            Direction::Left => (guard_position.0, (guard_position.1 as isize - 1) as usize),
            Direction::Right => (guard_position.0, guard_position.1 + 1),
            Direction::Up => ((guard_position.0 as isize - 1) as usize, guard_position.1),
            Direction::Down => (guard_position.0 + 1, guard_position.1),
        };

        if guard_has_left(rows, cols, (newRow, newCol)) {
            path_marked[guard_position.0][guard_position.1] = visited_marker;
            guard_position.0 = newRow;
            guard_position.1 = newCol;
            continue;
        }

        if is_obstacle(&matrix[newRow][newCol]) {
            if obstacles_hit_count[newRow][newCol].iter().any(|d| d.is_self(&direction)) {
                // cycle! We walked into the same obstacle in the same direction!
                // Note it's okay to walk into it from a different direction though.
                return None;
            }

            obstacles_hit_count[newRow][newCol].push(direction.clone());
            matrix[guard_position.0][guard_position.1] = direction.turn_right();
            continue;
        }

        matrix[guard_position.0][guard_position.1] = floor;
        path_marked[guard_position.0][guard_position.1] = visited_marker;
        guard_position.0 = newRow;
        guard_position.1 = newCol;
        matrix[guard_position.0][guard_position.1] = direction.to_char();
    }

    let mut distinct_positions_count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if path_marked[row][col] == visited_marker {
                distinct_positions_count += 1;
            }
        }
    }

    Some(distinct_positions_count)
}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    fn of(guard: &char) -> Option<Direction> {
        match guard {
            '>' => return Some(Direction::Right),
            '<' => return Some(Direction::Left),
            'v' => return Some(Direction::Down),
            '^' => return Some(Direction::Up),
            _ => return None,
        }
    }
    fn turn_right(&self) -> char {
        match self {
            Direction::Right => 'v',
            Direction::Left => '^',
            Direction::Down => '<',
            Direction::Up => '>',
        }
    }

    fn is_self(&self, other: &Direction) -> bool {
        self == other
    }

    fn to_char(&self) -> char {
        match self {
            Direction::Right => '>',
            Direction::Left => '<',
            Direction::Down => 'v',
            Direction::Up => '^',
        }
    }
}

fn init_cycle_detector(rows: usize, cols: usize) -> Vec<Vec<Vec<Direction>>> {
    let mut matrix = Vec::new();
    for r in 0..rows {
        matrix.push(Vec::new());
        for _ in 0..cols { 
            matrix[r].push(Vec::new())
        }
    }
    matrix
}

fn is_obstacle(c: &char) -> bool {
    *c == '#'
}

fn guard_has_left(rows: usize, cols: usize, pos: (usize, usize)) -> bool {
    if pos.0 < 0 || pos.1 < 0 {
        return true;
    }
    if pos.0 >= rows || pos.1 >= cols {
        return true;
    }
    return false;
}

fn is_guard(c: &char) -> bool {
    let g = vec!('^', '>', '<', 'v');
    g.iter().find(|&symbol| c == symbol).is_some()
}

fn make_matrix(raw_data: &str) -> (Vec<Vec<char>>, usize, usize) {
        // Parse the matrix
    let matrix: Vec<Vec<char>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = matrix.len();
    let cols = matrix.get(0).map(|m| m.len()).unwrap_or(0);
    (matrix, rows, cols)
}

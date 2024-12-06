use std::fs;

fn main() {
    let raw_data = fs::read_to_string("../input.txt").expect("bad input data");
    let raw_data = raw_data.as_str();
    let visited_marker = 'X';
    let floor = '.';

    let (mut matrix, rows, cols) = make_matrix(raw_data);

    let mut guard_position = (0, 0);
    for row in 0..rows {
        for col in 0..cols {
            if is_guard(&matrix[row][col]) {
                guard_position = (row, col);
            }
        }
    }
    println!("start pos {:?}", guard_position);

    let mut path_marked = matrix.clone();
    loop {
        if guard_has_left(rows, cols, guard_position) {
            break;
        }

        let direction = Direction::of(&matrix[guard_position.0][guard_position.1]).unwrap();
        let (newRow, newCol) = match direction {
            Direction::Left => (guard_position.0, (guard_position.1 as isize - 1) as usize),
            Direction::Right => (guard_position.0, guard_position.1 + 1),
            Direction::Up => (guard_position.0 - 1, guard_position.1),
            Direction::Down => (guard_position.0 + 1, guard_position.1),
        };
        println!("{:?} {}", newRow, newCol);

        if guard_has_left(rows, cols, (newRow, newCol)) {
            path_marked[guard_position.0][guard_position.1] = visited_marker;
            guard_position.0 = newRow;
            guard_position.1 = newCol;
            continue;
        }

        if is_obstacle(&matrix[newRow][newCol]) {
            matrix[guard_position.0][guard_position.1] = direction.turn_right();
            continue;
        }

        matrix[guard_position.0][guard_position.1] = floor;
        path_marked[guard_position.0][guard_position.1] = visited_marker;
        guard_position.0 = newRow;
        guard_position.1 = newCol;
        matrix[guard_position.0][guard_position.1] = direction.to_char();
    }

    println!("{:?}", path_marked);
    let mut distinct_positions_count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if path_marked[row][col] == visited_marker {
                distinct_positions_count += 1;
            }
        }
    }

    println!("Distinct Positions {:?}", distinct_positions_count);
}

#[derive(Debug)]
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
            other => return None,
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

    fn to_char(&self) -> char {
        match self {
            Direction::Right => '>',
            Direction::Left => '<',
            Direction::Down => 'v',
            Direction::Up => '^',
        }
    }
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

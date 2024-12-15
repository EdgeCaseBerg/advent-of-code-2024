pub mod boilerplate;

fn main() {
    let raw_data = crate::boilerplate::get_sample_if_no_input();
    if let Err(ref problem) = raw_data {
        println!("Could not read input data: {:?}", problem);
        return;
    }
    let data = raw_data.unwrap();
    let mut warehouse = Warehouse::from(&data);
    let robo_moves = parse_robot_input(&data);
    part_1(&mut warehouse, &robo_moves);
}

fn part_1(warehouse: &mut Warehouse, robo_moves: &[RoboMoves]) {
    for command in robo_moves {
        warehouse.update(*command);
        println!("Robot moved {:?}", command);
        warehouse.print_map();
    }
    println!("Sum of box GPS: {:?}", warehouse.gps_sum());
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum WarehouseItem {
    Wall,
    Robot,
    Box,
    Empty
}

impl WarehouseItem {
    fn from(c: char) -> Option<WarehouseItem> {
        match c {
            '#' => Some(WarehouseItem::Wall),
            '@' => Some(WarehouseItem::Robot),
            'O' => Some(WarehouseItem::Box),
            '.' => Some(WarehouseItem::Empty),
            _ => None
        }
    }

    fn display_char(&self) -> char {
        match self {
            WarehouseItem::Wall => '#',
            WarehouseItem::Robot => '@',
            WarehouseItem::Box => 'O',
            WarehouseItem::Empty => '.',
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    robot_position: (usize, usize),
    map: Vec<Vec<WarehouseItem>>
}

impl Warehouse {
    fn from(input: &str) -> Warehouse {
        let map = parse_warehouse(input);
        let mut position = (0, 0);
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                if map[row][col] == WarehouseItem::Robot {
                    position = (row, col)
                }
            }
        }
        Warehouse {
            map,
            robot_position: position
        }
    }

    fn update(&mut self, command: RoboMoves) {
        match command {
            RoboMoves::Up  => self.move_block(self.robot_position, (-1, 0)),
            RoboMoves::Down => self.move_block(self.robot_position, (1, 0)),
            RoboMoves::Right => self.move_block(self.robot_position, (0, 1)),
            RoboMoves::Left => self.move_block(self.robot_position, (0, -1))
        };
    }

    fn move_block(&mut self, block_to_move: (usize, usize), dir: (isize, isize)) -> bool {
        let new_row = block_to_move.0 as isize + dir.0;
        let new_col = block_to_move.1 as isize + dir.1;
        if !self.in_bounds(new_row, new_col) {
            // We cannot move!
            return false;
        }
        let new_row = new_row as usize;
        let new_col = new_col as usize;

        match self.map[new_row][new_col] {
            WarehouseItem::Wall => false,
            WarehouseItem::Empty => {
                self.map[new_row][new_col] = self.map[block_to_move.0][block_to_move.1];
                self.map[block_to_move.0][block_to_move.1] = WarehouseItem::Empty;
                true
            }
            _ => { 
                // Otherwise it's the box because there's only ever one robot and the robot doesn't push itself.
                // Apply force along direction
                if !self.move_block((new_row, new_col), dir) {
                    // If the next block cannot move, we cannot move.
                    return false;
                }

                // The block will have moved to the empty space now.
                self.map[new_row][new_col] = self.map[block_to_move.0][block_to_move.1];
                self.map[block_to_move.0][block_to_move.1] = WarehouseItem::Empty;
                true
            }
        }
    }

    fn in_bounds(&self, row: isize, col: isize) -> bool {
        let within_rows = 0 <= row && row < self.map.len() as isize;
        let within_cols = 0 <= col && col < self.map[0].len() as isize;
        within_rows && within_cols
    }

    fn gps_sum(&self) -> u64 {
        let mut sum = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if self.map[row][col] == WarehouseItem::Box {
                    sum += 100 * row as u64 + col as u64;
                }
            }
        }
        sum
    }

    fn print_map(&self) {
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                print!(" {:?} ", self.map[row][col].display_char());
            }
            println!();
        }
        println!();
    }
}

fn parse_warehouse(input: &str) -> Vec<Vec<WarehouseItem>> {
    let mut rows = Vec::new();
    input.lines()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            line.chars().map(|c| {
                WarehouseItem::from(c)
            }).collect()
        }).for_each(|cols| {
            rows.push(cols);
        });
    rows
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RoboMoves {
    Left,
    Up,
    Right,
    Down
}

impl RoboMoves {
    fn from(c: char) -> Option<RoboMoves> {
        match c {
            '<' => Some(RoboMoves::Left),
            '^' => Some(RoboMoves::Up),
            '>' => Some(RoboMoves::Right),
            'v' => Some(RoboMoves::Down),
            _ => None
        }
    }
}

fn parse_robot_input(input: &str) -> Vec<RoboMoves> {
    let mut moves = Vec::new();
    input.lines()
        .skip_while(|line| !line.is_empty())
        .skip_while(|line| line.is_empty())
        .map(|line| {
            line.chars().filter_map(|c| {
                RoboMoves::from(c)
            }).collect::<Vec<RoboMoves>>()
        }).for_each(|mut cols| {
            moves.append(&mut cols);
        });
    moves
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn calculates_gps_sum_correctly() {
        let verification_data = "##########
                                #.O.O.OOO#
                                #........#
                                #OO......#
                                #OO@.....#
                                #O#.....O#
                                #O.....OO#
                                #O.....OO#
                                #OO....OO#
                                ##########".replace(" ", "");
        let w = Warehouse::from(&verification_data);
        println!("{:?}", w);
        assert_eq!(10092, w.gps_sum());
    }
}
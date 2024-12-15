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
    part_2(&data, &robo_moves);
}

fn part_1(warehouse: &mut Warehouse, robo_moves: &[RoboMoves]) {
    for command in robo_moves {
        warehouse.update(*command);
        warehouse.print_map(false);
    }
    println!("Sum of box GPS: {:?}", warehouse.gps_sum());
}

fn part_2(data: &str, robo_moves: &[RoboMoves]) {
    let base = Warehouse::from(&data);
    let mut large_warehouse = base.scale_up();
    for command in robo_moves {
        large_warehouse.update(*command);
        large_warehouse.print_map(true);
    }
    println!("Sum of large warehouse GPS {:?}", large_warehouse.gps_sum());
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum LargeWarehouseItem {
    Wall,
    Robot,
    BoxLeft, BoxRight,
    Empty
}

impl LargeWarehouseItem {
    fn from(small: WarehouseItem) -> (LargeWarehouseItem, LargeWarehouseItem) {
        match small {
            WarehouseItem::Wall  => (LargeWarehouseItem::Wall, LargeWarehouseItem::Wall),
            WarehouseItem::Robot => (LargeWarehouseItem::Robot, LargeWarehouseItem::Empty),
            WarehouseItem::Box   => (LargeWarehouseItem::BoxLeft, LargeWarehouseItem::BoxRight),
            WarehouseItem::Empty => (LargeWarehouseItem::Empty, LargeWarehouseItem::Empty)
        }
    }

    fn display_char(&self) -> char {
        match self {
            LargeWarehouseItem::Wall => '#',
            LargeWarehouseItem::Robot => '@',
            LargeWarehouseItem::BoxLeft => '[',
            LargeWarehouseItem::BoxRight => ']',
            LargeWarehouseItem::Empty => '.',
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
                // println!("should move to empty {:?} -> {:?}", block_to_move, (new_row, new_col));
                self.map[new_row][new_col] = self.map[block_to_move.0][block_to_move.1];
                self.map[block_to_move.0][block_to_move.1] = WarehouseItem::Empty;
                if self.map[new_row][new_col] == WarehouseItem::Robot {
                    self.robot_position = (new_row, new_col);
                }
                true
            }
            _ => { 
                // Otherwise it's the box because there's only ever one robot and the robot doesn't push itself.
                // Apply force along direction
                if !self.move_block((new_row, new_col), dir) {
                    // If the next block cannot move, we cannot move.
                    // println!("Could not move {:?} -> {:?}", block_to_move, (new_row, new_col));
                    return false;
                }

                // The block will have moved to the empty space now.
                self.map[new_row][new_col] = self.map[block_to_move.0][block_to_move.1];
                self.map[block_to_move.0][block_to_move.1] = WarehouseItem::Empty;
                if self.map[new_row][new_col] == WarehouseItem::Robot {
                    self.robot_position = (new_row, new_col);
                }
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

    fn print_map(&self, are_we_debugging: bool) {
        if !are_we_debugging {
            return;
        }
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                print!(" {:?} ", self.map[row][col].display_char());
            }
            println!();
        }
        println!();
    }

    fn scale_up(&self) -> LargeWarehouse {
        let scaled: Vec<Vec<LargeWarehouseItem>> = self.map.iter().map(|row| {
            let mut scaled_columns = Vec::new();
            row.iter().for_each(|col| {
                let (left, right) = LargeWarehouseItem::from(*col);
                scaled_columns.push(left);
                scaled_columns.push(right);
            });
            scaled_columns
        }).collect();

        let mut robot_position = (0,0);
        for row in 0..scaled.len() {
            for col in 0..scaled[row].len() {
                if scaled[row][col] == LargeWarehouseItem::Robot {
                    robot_position = (row, col);
                }
            }
        }

        LargeWarehouse {
            robot_position,
            map: scaled
        }
    }
}

struct LargeWarehouse {
    robot_position: (usize, usize),
    map: Vec<Vec<LargeWarehouseItem>>
}

impl LargeWarehouse {
    fn update(&mut self, command: RoboMoves) {
        // match command {
        //     RoboMoves::Up  => self.move_block(self.robot_position, (-1, 0)),
        //     RoboMoves::Down => self.move_block(self.robot_position, (1, 0)),
        //     RoboMoves::Right => self.move_block(self.robot_position, (0, 1)),
        //     RoboMoves::Left => self.move_block(self.robot_position, (0, -1))
        // };
    }

    fn print_map(&self, are_we_debugging: bool) {
        if !are_we_debugging {
            return;
        }
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                print!(" {:?} ", self.map[row][col].display_char());
            }
            println!();
        }
        println!();
    }

    fn gps_sum(&self) -> u64 {
        let mut sum = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                // Note the writing is a bit ambigious on the closest edge, does that mean closest edge of the box to the left wall?
                // or does it mean the right edge could be close to the right wall?
                if self.map[row][col] == LargeWarehouseItem::BoxLeft {
                    sum += 100 * row as u64 + col as u64;
                }
            }
        }
        sum
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

fn parse_large_warehouse(input: &str) -> LargeWarehouse {
    let items: Vec<Vec<LargeWarehouseItem>> = input.lines().map(|line| {
        line.chars().filter_map(|c| {
            match c {
                '#' => Some(LargeWarehouseItem::Wall),
                '[' => Some(LargeWarehouseItem::BoxLeft),
                ']' => Some(LargeWarehouseItem::BoxRight),
                '.' => Some(LargeWarehouseItem::Empty),
                '@' => Some(LargeWarehouseItem::Robot),
                _ => None
            }
        }).collect()
    }).collect();

    let mut robot_position = (0,0);
    for row in 0..items.len() {
        for col in 0..items[row].len() {
            if items[row][col] == LargeWarehouseItem::Robot {
                robot_position = (row, col);
            }
        }
    }

    LargeWarehouse {
        robot_position,
        map: items
    }
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

    #[test]
    fn calculates_gps_sum_correctly_large_warehouse() {
        let verification_data ="####################
                                ##[].......[].[][]##
                                ##[]...........[].##
                                ##[]........[][][]##
                                ##[]......[]....[]##
                                ##..##......[]....##
                                ##..[]............##
                                ##..@......[].[][]##
                                ##......[][]..[]..##
                                ####################".replace(" ", "");

        
        let m = parse_large_warehouse(&verification_data);
        assert_eq!(9021, m.gps_sum());
    }

    #[test]
    fn scales_correctly() {
        let verification_data = "   ##########
                                    #..O..O.O#
                                    #......O.#
                                    #.OO..O.O#
                                    #..O@..O.#
                                    #O#..O...#
                                    #O..O..O.#
                                    #.OO.O.OO#
                                    #....O...#
                                    ##########".replace(" ", "");
        let w = Warehouse::from(&verification_data);
        let l = w.scale_up();
        
        let expected_large = "  ####################
                                ##....[]....[]..[]##
                                ##............[]..##
                                ##..[][]....[]..[]##
                                ##....[]@.....[]..##
                                ##[]##....[]......##
                                ##[]....[]....[]..##
                                ##..[][]..[]..[][]##
                                ##........[]......##
                                ####################".replace(" ", "");
        let e = parse_large_warehouse(&expected_large);

        for row in 0..e.map.len() {
            for col in 0..e.map[row].len() {
                assert_eq!(e.map[row][col], l.map[row][col]);
            }
        }
    }
}
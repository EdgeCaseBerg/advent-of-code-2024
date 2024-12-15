pub mod boilerplate;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

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
    let wall_reference = base.scale_up();
    let mut large_warehouse = base.scale_up();
    large_warehouse.print_map(true);
    for command in robo_moves {
        large_warehouse.update(*command);
        if large_warehouse.the_walls_have_moved_on_their_own(&wall_reference) {
            println!("The walls have moved! This command {:?} The walls should be:", command);
            wall_reference.print_map(true);
            println!("But instead, they were:");
            large_warehouse.print_map(true);
            println!("The robots position is {:?}", large_warehouse.robot_position);
            assert_eq!(false, true);
        }
        large_warehouse.print_map(false);
    }
    large_warehouse.print_map(true); 
    // too high 1555805
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
            let mut out_col = String::new();
            for col in 0..self.map[row].len() {
                out_col.push(self.map[row][col].display_char());
            }
            println!("{}", out_col);
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
        match command {
            RoboMoves::Up  => self.move_block(self.robot_position, (-1, 0)),
            RoboMoves::Down => self.move_block(self.robot_position, (1, 0)),
            RoboMoves::Right => self.move_block(self.robot_position, (0, 1)),
            RoboMoves::Left => self.move_block(self.robot_position, (0, -1))
        };
    }

    fn get_blocks_to_move(&mut self, start_pos: (usize, usize), dir: (isize, isize)) -> HashSet<(usize, usize)> {
        let mut to_process = VecDeque::new();
        let mut visited = HashSet::new();
        
        // bfs sorta kinda
        to_process.push_back(start_pos);

        match self.map[start_pos.0][start_pos.1] {
            LargeWarehouseItem::BoxLeft => {
                to_process.push_back((start_pos.0, start_pos.1 + 1));
            }
            LargeWarehouseItem::BoxRight => {
                to_process.push_back((start_pos.0, start_pos.1 - 1));
            }
            _ => {}
        }

        while let Some((current_row, current_col)) = to_process.pop_front() {
            // Calculate new position
            let new_row = current_row as isize + dir.0;
            let new_col = current_col as isize + dir.1;

            if !self.in_bounds(new_row, new_col) {
                // Out of bounds, cannot move
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if visited.contains(&(current_row, current_col)) {
                continue;
            }
            visited.insert((current_row, current_col));

            match self.map[new_row][new_col] {
                LargeWarehouseItem::BoxLeft => {
                    // Add both parts of the double-wide box to the queue
                    to_process.push_back((new_row, new_col));
                    to_process.push_back((new_row, new_col + 1));
                }
                LargeWarehouseItem::BoxRight => {
                    to_process.push_back((new_row, new_col));
                    to_process.push_back((new_row, new_col - 1));
                }
                _ => { continue }
            }
        }
        visited
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
        // println!("R: {:?}, {:?} {:?}", self.robot_position, dir, self.map[new_row][new_col]);

        match self.map[new_row][new_col] {
            LargeWarehouseItem::Wall => false,
            LargeWarehouseItem::Empty => {
                self.map[new_row][new_col] = self.map[block_to_move.0][block_to_move.1];
                self.map[block_to_move.0][block_to_move.1] = LargeWarehouseItem::Empty;
                if self.map[new_row][new_col] == LargeWarehouseItem::Robot {
                    self.robot_position = (new_row, new_col);
                }
                true
            }
            LargeWarehouseItem::BoxLeft | LargeWarehouseItem::BoxRight => {
                let mut to_verify_can_move = self.get_blocks_to_move((new_row, new_col), dir);
                if self.map[new_row][new_col] == LargeWarehouseItem::BoxLeft {
                    let others = self.get_blocks_to_move((new_row, new_col + 1), dir);
                    for other in others {
                        to_verify_can_move.insert(other);
                    }
                    to_verify_can_move.insert((new_row, new_col + 1));
                } else {
                    let others = self.get_blocks_to_move((new_row, new_col - 1), dir);
                    for other in others {
                        to_verify_can_move.insert(other);
                    }
                    to_verify_can_move.insert((new_row, new_col - 1));
                }
                let to_verify_can_move = Vec::from_iter(to_verify_can_move);
                match dir {
                    (0, -1) => {
                        // Pushing left, straight line case:
                        let minimum_col = to_verify_can_move.into_iter().min_by_key(|(_, c)| *c).unwrap();
                        let space_to_fill = (minimum_col.0, (minimum_col.1 as isize - 1) as usize);
                        if !self.in_bounds(space_to_fill.0 as isize, space_to_fill.1 as isize) {
                            // It's out of bounds somehow. Nope.
                            return false;
                        }
                        if self.map[space_to_fill.0][space_to_fill.1] == LargeWarehouseItem::Empty {
                            // We CAN move here!
                            let mut fill_row = space_to_fill;
                            loop {
                                let cur = self.map[fill_row.0][fill_row.1];
                                let moving =  self.map[fill_row.0][fill_row.1 + 1];

                                self.map[fill_row.0][fill_row.1] = moving;
                                self.map[fill_row.0][fill_row.1 + 1] = cur;
                                if fill_row.0 == block_to_move.0 && fill_row.1 + 1 == block_to_move.1 {
                                    self.robot_position = fill_row;
                                    break;
                                }
                                fill_row = (fill_row.0, fill_row.1 + 1);
                            }
                            return true;
                        }
                    },
                    (0, 1) => {
                        // Pushing right, straight line case:
                        let maximum_col = to_verify_can_move.into_iter().max_by_key(|(_, c)| *c).unwrap();
                        let space_to_fill = (maximum_col.0, (maximum_col.1 as isize + 1) as usize);
                        if !self.in_bounds(space_to_fill.0 as isize, space_to_fill.1 as isize) {
                            // It's out of bounds somehow. Nope.
                            return false;
                        }
                        if self.map[space_to_fill.0][space_to_fill.1] == LargeWarehouseItem::Empty {
                            // We CAN move here!
                            let mut fill_row = space_to_fill;
                            loop {
                                let cur = self.map[fill_row.0][fill_row.1];
                                let moving =  self.map[fill_row.0][(fill_row.1 as isize - 1) as usize];

                                self.map[fill_row.0][fill_row.1] = moving;
                                self.map[fill_row.0][fill_row.1 - 1] = cur;

                                if fill_row.0 == block_to_move.0 && (fill_row.1 as isize - 1) as usize == block_to_move.1 {
                                    self.robot_position = fill_row;
                                    break;
                                }
                                fill_row = (fill_row.0, fill_row.1 - 1);
                            }
                            return true;
                        }
                    }
                    (1, 0) => {
                        // The tricky case of down visually, positively numerical
                        let maximum_row = to_verify_can_move.iter().max_by_key(|(r, _)| *r).unwrap();
                        // for each of the items in that row, check if they can move down.
                        let mut row_to_check = to_verify_can_move.iter().filter(|(r, _)| *r == maximum_row.0);
                        let can_shift_all = row_to_check.all(|(r,c)| {
                            let below = self.map[*r + 1][*c];
                            below == LargeWarehouseItem::Empty
                        });
                        if !can_shift_all {
                            return false;
                        }

                         let mut blocks = to_verify_can_move.clone();
                        // The above has all blocks but not the robot. Move him too.
                        blocks.push(self.robot_position);
                        blocks.sort_by_key(|(r, _)| *r);
                        for position in blocks.iter().rev() {
                            let swap_row = position.0 + 1;
                            let swap_col = position.1;
                            let a = self.map[swap_row][swap_col];
                            let b = self.map[position.0][position.1];
                            self.map[position.0][position.1] = a;
                            self.map[swap_row][swap_col] = b;
                        }
                        self.robot_position = (new_row, new_col);
                        return false;
                    }
                    (-1, 0) => {
                        // The tricky case of upwards visually, negative numerically
                        let minimum_row = to_verify_can_move.iter().min_by_key(|(r, _)| *r).unwrap();
                        // for each of the items in that row, check if they can move down.
                        let mut row_to_check = to_verify_can_move.iter().filter(|(r, _)| *r == minimum_row.0);
                        let can_shift_all = row_to_check.all(|(r,c)| {
                            let above = self.map[*r - 1][*c];
                            above == LargeWarehouseItem::Empty
                        });
                        if !can_shift_all {
                            return false;
                        }

                        let mut blocks = to_verify_can_move.clone();
                        // The above has all blocks but not the robot. Move him too.
                        blocks.push(self.robot_position);
                        blocks.sort_by_key(|(r, _)| *r);
                        for position in blocks.iter() {
                            let swap_row = position.0 - 1;
                            let swap_col = position.1;
                            let a = self.map[swap_row][swap_col];
                            let b = self.map[position.0][position.1];
                            self.map[position.0][position.1] = a;
                            self.map[swap_row][swap_col] = b;
                        }
                        self.robot_position = (new_row, new_col);
                        return false;
                    }
                    other => {
                        println!("unexpected direction {:?}", other);
                    } // temp ignore
                }
                false
            },
            LargeWarehouseItem::Robot => {
                println!(" impossible unless screwed up somewheere ");
                if !self.move_block((new_row, new_col), dir) {
                    // If the next block cannot move, we cannot move.
                    // println!("Could not move {:?} -> {:?}", block_to_move, (new_row, new_col));
                    return false;
                }

                // The block will have moved to the empty space now.
                self.map[new_row][new_col] = self.map[block_to_move.0][block_to_move.1];
                self.map[block_to_move.0][block_to_move.1] = LargeWarehouseItem::Empty;
                if self.map[new_row][new_col] == LargeWarehouseItem::Robot {
                    self.robot_position = (new_row, new_col);
                }
                true
            }
        }
    }

    fn in_bounds(&self, row: isize, col: isize) -> bool {
        let within_rows = 0 <= row && row < self.map.len() as isize;
        let within_cols = 0 <= col && col < self.map[row as usize].len() as isize;
        within_rows && within_cols
    }

    fn print_map(&self, are_we_debugging: bool) {
        if !are_we_debugging {
            return;
        }
        for row in 0..self.map.len() {
            let mut out_col = String::new();
            for col in 0..self.map[row].len() {
                out_col.push(self.map[row][col].display_char());
            }
            println!("{}", out_col);
        }
        println!();
    }

    fn gps_sum(&self) -> u64 {
        let mut sum = 0;
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                // Note the writing is a bit ambigious on the closest edge, does that mean closest edge of the box to the left wall?
                // or does it mean the right edge could be close to the right wall?
                let is_closer_to_left = true; // col <= self.map[row].len() / 2;
                let edge_type_to_check = if is_closer_to_left {
                    LargeWarehouseItem::BoxLeft
                } else {
                    LargeWarehouseItem::BoxRight
                };
                if self.map[row][col] == edge_type_to_check {
                    sum += 100 * row as u64 + col as u64;
                }
            }
        }
        sum
    }

    fn the_walls_have_moved_on_their_own(&self, reference: &LargeWarehouse) -> bool {
        for r in 0..reference.map.len() {
            for c in 0..reference.map[r].len() {
                if reference.map[r][c] == LargeWarehouseItem::Wall {
                    if self.map[r][c] != LargeWarehouseItem::Wall {
                        return true;
                    }
                }
            }
        }
        return false;
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

    #[test]
    fn moves_robot_in_large_correctly() {
        let verification_data = "   #####
                                    #.@.#
                                    #.O.#
                                    #.OO#
                                    #.OO#
                                    #..O#
                                    #..O#
                                    #O.O#
                                    #..O#
                                    #O.O#
                                    #..O#
                                    #OO##
                                    #####".replace(" ", "");
        let w = Warehouse::from(&verification_data);
        let mut l = w.scale_up();
        let commands: Vec<RoboMoves> = "<v>^>vvvvvvvvvv".chars().filter_map(|c| RoboMoves::from(c)).collect();
        for command in commands {
            l.update(command);
        }
        
        let expected_large = "      ##########
                                    ##...@..##
                                    ##...[].##
                                    ##..[][]##
                                    ##..[][]##
                                    ##....[]##
                                    ##....[]##
                                    ##[]..[]##
                                    ##....[]##
                                    ##[]..[]##
                                    ##....[]##
                                    ##[][]####
                                    ##########".replace(" ", "");
        let e = parse_large_warehouse(&expected_large);

        for row in 0..e.map.len() {
            for col in 0..e.map[row].len() {
                if e.map[row][col] != l.map[row][col] {
                    l.print_map(true);
                }
                assert_eq!(e.map[row][col], l.map[row][col]);
            }
        }
    }

    #[test]
    fn moves_left_correctly() {
        let start_state = " ##########
                            ##[][].@##
                            ##..[]..##
                            ##...[].##
                            ##########
                            ".replace(" ", "");
        let end_state =  "  ##########
                            ##[][]..##
                            ##.[]...##
                            ##[]@...##
                            ##########
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "<<<v<v>>>v<<<".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn moves_right_correctly() {
        let start_state = " ##############
                            ##@[][].....##
                            ##.[].......##
                            ##.[].......##
                            ##.[][][][].##
                            ##############
                            ".replace(" ", "");
        let end_state =  "  ##############
                            ##......[][]##
                            ##......[]..##
                            ##.....[]...##
                            ##.@[][][][]##
                            ##############
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = ">>>>> <<<<< v >>>>> <<<<< v >>>> <<<< v >".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn moves_up_correctly() {
        let start_state = " ############
                            ##........##
                            ##........##
                            ##..[][]..##
                            ##...[]...##
                            ##...@....##
                            ############
                            ".replace(" ", "");
        let end_state =  "  ############
                            ##..[][]..##
                            ##...[]...##
                            ##....@...##
                            ##........##
                            ##........##
                            ############
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "^>^".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn moves_down_correctly() {
        let start_state = " ############
                            ##.....@..##
                            ##....[]..##
                            ##...[][].##
                            ##........##
                            ##........##
                            ############
                            ".replace(" ", "");
        let end_state =  "  ############
                            ##........##
                            ##........##
                            ##....@...##
                            ##....[]..##
                            ##...[][].##
                            ############
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "v<v".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn moves_a_stair_case_down_correctly() {
        let start_state = " ############
                            ##.....@..##
                            ##....[]..##
                            ##...[]...##
                            ##..[][]..##
                            ##........##
                            ############
                            ".replace(" ", "");
        let end_state =  "  ############
                            ##........##
                            ##.....@..##
                            ##....[]..##
                            ##...[]...##
                            ##..[][]..##
                            ############
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "v".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn moves_a_stair_case_up_correctly() {
        let start_state = " ############
                            ##........##
                            ##....[]..##
                            ##...[]...##
                            ##..[][]..##
                            ##...@....##
                            ############
                            ".replace(" ", "");
        let end_state =  "  ############
                            ##....[]..##
                            ##...[]...##
                            ##..[]....##
                            ##...@[]..##
                            ##........##
                            ############
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "^".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn move_a_case_around_again() {
        let start_state = " ####################
                            ##................##
                            ##.....[].........##
                            ##....[]..[]......##
                            ##.....[][].......##
                            ##......[]........##
                            ##......@.........##
                            ##................##
                            ##................##
                            ####################
                            ".replace(" ", "");
        let end_state =  "  ####################
                            ##.....[].........##
                            ##....[]..[]......##
                            ##.....[].........##
                            ##................##
                            ##................##
                            ##.......@........##
                            ##.......[].......##
                            ##......[]........##
                            ####################
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "^>>>>^^^^^<<<vvvvvvvvvv".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
        assert_eq!(2347, w.gps_sum());
    }

    #[test]
    fn does_not_move_a_wall_to_the_right() {
        let start_state = " ##############
                            ##.@[].#....##
                            ##..[][]..#.##
                            ##############
                            ".replace(" ", "");
        let end_state =  "  ##############
                            ##...[]#....##
                            ##...@[][]#.##
                            ##############
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = ">>> <<< v >>>>>>".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn does_not_move_a_wall_to_the_left() {
        let start_state = " ##############
                            ##.#.[]..@..##
                            ##.#.[][]...##
                            ##.#........##
                            ##############
                            ".replace(" ", "");
        let end_state =  "  ##############
                            ##.#[]......##
                            ##.#[][]@...##
                            ##.#........##
                            ##############
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "<<< >>>> v <<<<<".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn does_not_move_a_wall_upwards() {
        let start_state = " ##############
                            ##..........##
                            ##..##......##
                            ##....#.....##
                            ##..[]@.....##
                            ##..........##
                            ##############
                            ".replace(" ", "");
        let end_state =  "  ##############
                            ##..........##
                            ##..##......##
                            ##.[].#.....##
                            ##..@.......##
                            ##..........##
                            ##############
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "^ > ^ <<< v << v < ^^^".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w   = parse_large_warehouse(&start_state);
        let end = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn does_not_move_a_wall_downwards() {
        let start_state = " ##############
                            ##....@.....##
                            ##.[].#.....##
                            ##..........##
                            ##..##......##
                            ##..........##
                            ##############
                            ".replace(" ", "");
        let end_state =  "  ##############
                            ##..........##
                            ##..@.#.....##
                            ##.[].......##
                            ##..##......##
                            ##..........##
                            ##############
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "vvv << vvv".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w = parse_large_warehouse(&start_state);
        let end   = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
        w.print_map(true);
    }

    #[test]
    fn should_not_teleport_the_walls_when_moving_down() {
                let start_state = " 
                ####################################################################################################
                ##[]........[]........[]....[]....####[]........[]..........[]..[]....[][]....[][][]..[][]........##
                ##[]..............[]....##[]..............[]....[]......[]........[]......[][]..[][]..##..........##
                ####..[][]..[][]##..[]##..[]..[][]....[][][]..[]......##....##......[]......[]....##......[]......##
                ##..[]......[]....##....[][]..[]....[]....##[]..[][]......[][][]..[][]....[]........[]..........####
                ##..[]....[][]....[]..[][]##......[]......[][][]@.....[]..##[]..[][]..[][]..........[][]....[]..[]##
                ##....[]........[][]....##[]....[]..[]..[]......[]....[][]........[][]....[][]..##..##..[]........##
                ##..####......[]..[]......[][][][]..[]..[]....[]..[]....##[]##..[]....[]..[][]##....[]............##
                ##[][][]##....##..##..[]####..........[][]..[]..[]....##[]....[]......[]..........[][]........[][]##
                ##....[]..[]......##................[]....[]..##........[][][][]..........##....##[]......[]....####
                ##......[]........[][]............[]..[][]....[]##[]..[]......##................[]..[]..##....[]..##
                ##..........[][]......##[][]..[]..[][]..[]##..[]..[]..##....................##......[]........[]..##
                ##....[][]....##[]..[]....[][]..[]..[]..[]......[]..[]....[][]..[][]....[][]..........[]......[]..##
                ##................................................................................................##
                ####################################################################################################
                            ".replace(" ", "");
        let end_state =  " 
                ####################################################################################################
                ##[]........[]........[]....[]....####[]........[]..........[]..[]....[][]....[][][]..[][]........##
                ##[]..............[]....##[]..............[]....[]......[]........[]......[][]..[][]..##..........##
                ####..[][]..[][]##..[]##..[]..[][]....[][][]..[]......##....##......[]......[]....##......[]......##
                ##..[]......[]....##....[][]..[]....[]....##[]..[][]......[][][]..[][]....[]........[]..........####
                ##..[]....[][]....[]..[][]##......[]......[][][]......[]..##[]..[][]..[][]..........[][]....[]..[]##
                ##....[]........[][]....##[]....[]..[]..[]......@.....[][]........[][]....[][]..##..##..[]........##
                ##..####......[]..[]......[][][][]..[]..[]....[][][]....##[]##..[]....[]..[][]##....[]............##
                ##[][][]##....##..##..[]####..........[][]..[]..[]....##[]....[]......[]..........[][]........[][]##
                ##....[]..[]......##................[]....[]..##........[][][][]..........##....##[]......[]....####
                ##......[]........[][]............[]..[][]....[]##[]..[]......##................[]..[]..##....[]..##
                ##..........[][]......##[][]..[]..[][]..[]##..[]..[]..##....................##......[]........[]..##
                ##....[][]....##[]..[]....[][]..[]..[]..[]......[]..[]....[][]..[][]....[][]..........[]......[]..##
                ##................................................................................................##
                ####################################################################################################
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "v".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w = parse_large_warehouse(&start_state);
        let end   = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    println!("There is a problem detected at {:?}", (row, col));
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
    }

    #[test]
    fn should_not_teleport_the_walls_when_moving_down_case_2() {
                let start_state = " 
                ####################################################################################################
                ##................................................................................................##
                ##........[][]##........[]..[]......[]......[]....[]##...@...[].......[]........[]....[]......[]..##
                ##[]......[][]..[]....##....[]................[]..[]##[].[].##....##[]##......[]......[]....##..####
                ##........................[]....##....##..[]......[]..####[][]..........[]..[]....##..[][]##..[]..##
                ##........[]........##................##[]....[]..[]......[]......................[]..##....##....##
                ##[]........[][]......[][]......##......[]........[][]..##[]##..........................[]....[]####
                ##....[][][]............[][][][]##..[][]......[][]##..[]....[]................[][]..[]..........[]##
                ####..[][][][]##....[]..[]............[]............[]........[]......[][]..[][]..[][][]..........##
                ##................................................................................................##
                ####################################################################################################
                            ".replace(" ", "");
        let end_state =  " 
                ####################################################################################################
                ##................................................................................................##
                ##........[][]##........[]..[]......[]......[]....[]##...@...[].......[]........[]....[]......[]..##
                ##[]......[][]..[]....##....[]................[]..[]##[].[].##....##[]##......[]......[]....##..####
                ##........................[]....##....##..[]......[]..####[][]..........[]..[]....##..[][]##..[]..##
                ##........[]........##................##[]....[]..[]......[]......................[]..##....##....##
                ##[]........[][]......[][]......##......[]........[][]..##[]##..........................[]....[]####
                ##....[][][]............[][][][]##..[][]......[][]##..[]....[]................[][]..[]..........[]##
                ####..[][][][]##....[]..[]............[]............[]........[]......[][]..[][]..[][][]..........##
                ##................................................................................................##
                ####################################################################################################
                            ".replace(" ", "");
        let commands: Vec<RoboMoves> = "v".chars().filter_map(|c| RoboMoves::from(c)).collect();
        let mut w = parse_large_warehouse(&start_state);
        let end   = parse_large_warehouse(&end_state);

        for command in commands {
            w.update(command);
        }

        for row in 0..w.map.len() {
            for col in 0..w.map[row].len() {
                if w.map[row][col] != end.map[row][col] {
                    println!("There is a problem detected at {:?}", (row, col));
                    w.print_map(true);
                }
                assert_eq!(w.map[row][col], end.map[row][col]);
            }
        }
    }
}
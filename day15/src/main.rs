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
    }
    println!("Sum of box GPS: {:?}", warehouse.gps_sum());
}

#[derive(Debug, PartialEq)]
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
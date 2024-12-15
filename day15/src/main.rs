pub mod boilerplate;


fn main() {
    let raw_data = crate::boilerplate::get_sample_if_no_input();
    if let Err(ref problem) = raw_data {
        println!("Could not read input data: {:?}", problem);
        return;
    }
    let data = raw_data.unwrap();
    let warehouse_map = parse_warehouse(&data);
    let robo_moves = parse_robot_input(&data);
    println!("{:?}", warehouse_map);
    println!("{:?}", robo_moves);
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    #[test]
    fn we_can_test_the_program() {
        assert_eq!(1, 1);
    }
}
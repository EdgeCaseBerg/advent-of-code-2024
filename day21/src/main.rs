pub mod boilerplate;

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let raw_data = crate::boilerplate::get_sample_if_no_input();
    if let Err(ref problem) = raw_data {
        println!("Could not read input data: {:?}", problem);
        return;
    }
    let data = raw_data.unwrap();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &str) {
    let mut complexity_sum = 0;
    let codes = get_codes(data);
    for code in codes {
        let presses = get_presses(code, &KeyPad::numeric_keypard());
        let presses = get_presses(&presses, &KeyPad::directional_keypad());
        let presses = get_presses(&presses, &KeyPad::directional_keypad());
        let presses = get_presses(&presses, &KeyPad::directional_keypad());
        let complexity = presses.len() as u64 * get_numeric_of(code);
        complexity_sum += complexity;
    }
    println!("Part 1 {:?}", complexity_sum);
}

fn part_2(data: &str) {
    let _foo = data;
}

fn get_numeric_of(code: &str) -> u64 {
    code.chars().take_while(|c| c.is_digit(10)).fold(String::new(), |accum, c| accum + &c.to_string()).parse().unwrap()
}

type Position = (usize, usize);
#[derive(Debug)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Press
}

use Action::*;

struct KeyPad {
    buttons: Vec<Vec<char>>,
    position: Position,
    neighbors: HashMap<char, Vec<(char, Action)>>
}

impl KeyPad {

    fn numeric_keypard() -> Self {
        KeyPad {
            buttons:  vec![
                vec!['7', '8', '9'],
                vec!['4', '5', '6'],
                vec!['1', '2', '3'],
                vec![' ', '0', 'A']
            ],
            position: (3, 2),
            neighbors: HashMap::from([
                ('9', vec![ ('8', Left ), ('6', Down)                             ]),
                ('8', vec![ ('7', Left ), ('9', Right), ('5', Down)               ]),
                ('7', vec![ ('4', Down ), ('8', Right)                            ]),
                ('6', vec![ ('9', Up   ), ('5', Left ), ('3', Down)               ]),
                ('5', vec![ ('8', Up   ), ('4', Left ), ('6', Right), ('2', Down) ]),
                ('4', vec![ ('7', Up   ), ('5', Right), ('1', Down)               ]),
                ('3', vec![ ('6', Up   ), ('2', Left ), ('A', Down)               ]),
                ('2', vec![ ('5', Up   ), ('1', Left ), ('3', Right), ('0', Down) ]),
                ('1', vec![ ('4', Up   ), ('2', Right)                            ]),
                ('0', vec![ ('2', Up   ), ('A', Right)                            ]),
                ('A', vec![ ('0', Left ), ('3', Up   )                            ])
            ])
        }
    }

    fn action_of(from: char, to: char) -> Vec<Position> {
        vec![]
    }

    fn directional_keypad() -> Self {
        KeyPad {
            buttons: vec![
                vec![' ', '^', 'A'],
                vec!['<', 'v', '>']
            ],
            position: (0, 2),
            neighbors: HashMap::from([
                ('^', vec![ ('v', Up   ), ('A', Right)               ]),
                ('A', vec![ ('^', Left ), ('<', Down )               ]),
                ('>', vec![ ('v', Right), ('A', Up   )               ]),
                ('v', vec![ ('^', Up,  ), ('<', Left ), ('>', Right) ]),
                ('<', vec![ ('v', Right)                             ]),
            ])
        }
    }

    fn is_valid_move(&self, to_pos: (isize, isize)) -> bool {
        if self.buttons.len() < to_pos.0 as usize && 0 > to_pos.0 {
            return false;
        }
        if self.buttons[0].len() < to_pos.1 as usize && 0 > to_pos.1 {
            return false;
        }

        self.buttons[to_pos.0 as usize][to_pos.1 as usize] == ' '
    }
}

fn get_presses(target: &str, keypad: &KeyPad) -> String {
    // TODO
    target.to_string()
}


fn get_codes(data: &str) -> Vec<&str> {
    data.lines().map(|s| s).collect::<Vec<&str>>()
}

// You _cannot_ move to the ' ' space, ever.
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
#[derive(Debug, Copy, Clone)]
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
            // Thought: Should I include (9,9) with a Press action? 
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

    fn paths(&self) -> HashMap<(char, char), Vec<Vec<Action>>> {
        let pad = self.neighbors.clone().into_iter().collect::<HashMap<char, Vec<(char, Action)>>>();
        let mut paths = HashMap::new();

        for key1 in pad.keys() {
            for key2 in pad.keys() {
                paths.insert((*key1, *key2), self.shortest_paths_from(*key1, *key2));
            }
        }

        paths
    }

    fn shortest_paths_from(&self, from: char, to: char) -> Vec<Vec<Action>> {
        // Just BFS along the neighbors.
        let mut paths = vec![];
        let mut queue = VecDeque::from([
            // position, actions path, visited along this path so far
            (from, Vec::<Action>::new(), HashSet::<>::new())
        ]);
        let mut shortest = usize::MAX;
        while let Some(current) = queue.pop_front() {
            let (button, path, mut visited) = current;

            // Hit the target with our last movement, is this a short path?
            if button == to {
                if shortest >= path.len() {
                    shortest = path.len();
                    // Leaving this here because I feel like I might want to do this. But not sure yet.
                    // let mut new_path = path.clone();
                    // new_path.push(Press);
                    // paths.push(new_path);
                    paths.push(path);
                }
                continue;
            }

            if visited.contains(&button) {
                continue;
            }
            visited.insert(button);

            for (next_button, action) in self.neighbors.get(&button).unwrap() {
                let mut new_path = path.clone();
                new_path.push(*action);
                queue.push_back((*next_button, new_path, visited.clone()));
            }
        }

        paths
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
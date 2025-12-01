pub mod boilerplate;

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::sync::OnceLock;

type ActionsListByFromAndTo = HashMap<(char, char), Vec<Vec<Action>>>;
static NUMERIC_SHORTEST_PATHS: OnceLock<ActionsListByFromAndTo> = OnceLock::new();
static DIRECTIONAL_SHORTEST_PATHS: OnceLock<ActionsListByFromAndTo> = OnceLock::new();

fn main() {
    let raw_data = crate::boilerplate::get_sample_if_no_input();
    if let Err(ref problem) = raw_data {
        println!("Could not read input data: {:?}", problem);
        return;
    }
    let data = raw_data.unwrap();
    let cache = &mut HashMap::new();
    part_1(&data, cache);
    part_2(&data, cache);
}

fn part_1(data: &str, cache: &mut HashMap<(String, u64, bool), usize>) {
    let mut complexity_sum = 0;
    let codes = get_codes(data);
    for code in codes {
        let presses = get_presses(code, 2, true, cache);
        let complexity = presses as u64 * get_numeric_of(code);
        // println!("CODE {:?} {:?} {:?}", code, presses, complexity);
        complexity_sum += complexity;
    }
    // Input sample shuld be 126384
    println!("Part 1 {:?}", complexity_sum);
}

fn part_2(data: &str, cache: &mut HashMap<(String, u64, bool), usize>) {
    println!("Precached data {:?}", cache.len());
    let mut complexity_sum = 0;
    let codes = get_codes(data);
    for code in codes {
        let presses = get_presses(code, 25, true, cache);
        let complexity = presses as u64 * get_numeric_of(code);
        // println!("CODE {:?} {:?} {:?}", code, presses, complexity);
        complexity_sum += complexity;
    }
    println!("Postcached data {:?}", cache.len());
    println!("Part 2 {:?}", complexity_sum);
}

fn get_numeric_of(code: &str) -> u64 {
    code.chars().take_while(|c| c.is_ascii_digit()).fold(String::new(), |accum, c| accum + &c.to_string()).parse().unwrap()
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
            // Thought: Should I include (9,9) with a Press action? I suppose I don't need to since when from==target we add Press.
            // You _cannot_ move to the ' ' space, ever.
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

    fn directional_keypad() -> Self {
        KeyPad {
            buttons: vec![
                vec![' ', '^', 'A'],
                vec!['<', 'v', '>']
            ],
            position: (0, 2),
            neighbors: HashMap::from([
                ('^', vec![ ('v', Down ), ('A', Right)               ]),
                ('A', vec![ ('^', Left ), ('>', Down )               ]),
                ('>', vec![ ('v', Left ), ('A', Up   )               ]),
                ('v', vec![ ('^', Up,  ), ('<', Left ), ('>', Right) ]),
                ('<', vec![ ('v', Right)                             ]),
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
                    let mut new_path = path.clone();
                    new_path.push(Press);
                    paths.push(new_path);
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
}

fn actions_to_direction_string(actions: Vec<Action>) -> String {
    let mut s = String::new();
    for action in &actions {
        match action {
            Up => s.push('^'),
            Left => s.push('<'),
            Right => s.push('>'),
            Down => s.push('v'),
            Press => s.push('A'),
        }
    }
    s
}

fn get_presses(target: &str, indirection_level: u64, is_number_keyboard: bool, cache: &mut HashMap<(String, u64, bool), usize>) -> usize {
    if let Some(cached) = cache.get(&(target.to_string(), indirection_level, is_number_keyboard)) {
        return *cached;
    }

    let keypad = if is_number_keyboard {
        KeyPad::numeric_keypard()
    } else {
        KeyPad::directional_keypad()
    };

    let paths = if is_number_keyboard {
        NUMERIC_SHORTEST_PATHS.get_or_init(|| {
            keypad.paths()
        })
    } else {
        DIRECTIONAL_SHORTEST_PATHS.get_or_init(|| {
            keypad.paths()
        })
    };

    let start = keypad.buttons[keypad.position.0][keypad.position.1];
    let mut sequence = vec![];
    sequence.push(start);
    for c in target.chars() {
        sequence.push(c);
    }
    let mut sequence = sequence.iter();
    let mut from = sequence.next().unwrap();
    let mut total_button_presses = 0;
    loop {
        let maybe_to = sequence.next();
        if maybe_to.is_none() {
            break;
        }
        let to = maybe_to.unwrap();
        let shortest_paths = paths.get(&(*from, *to)).unwrap();
        let presses = match indirection_level {
            0 => {
                // At the human level. We just need to know what the shortest path is. So...
                shortest_paths[0].len()
            }
            _ => {
                let presses_for_each_path = shortest_paths.iter().cloned().map(|path| {
                    let new_target = actions_to_direction_string(path);
                    get_presses(&new_target, indirection_level - 1, false, cache)
                });
                presses_for_each_path.min().unwrap()
            }
        };
        total_button_presses += presses;
        from = to;
    }
    cache.insert((target.to_string(), indirection_level, is_number_keyboard), total_button_presses);
    total_button_presses
}


fn get_codes(data: &str) -> Vec<&str> {
    data.lines().collect::<Vec<&str>>()
}


pub mod boilerplate;


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

struct KeyPad {
    buttons: Vec<Vec<char>>,
    position: (usize, usize)
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
            position: (3, 2)
        }
    }

    fn directional_keypad() -> Self {
        KeyPad {
            buttons: vec![
                vec![' ', '^', 'A'],
                vec!['<', 'v', '>']
            ],
            position: (0, 2)
        }
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
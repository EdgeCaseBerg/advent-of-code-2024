use std::fs;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let maybe_filename = get_filename_from_args();
    let maybe_filename = Some(String::from("../sample.txt"));
    if maybe_filename.is_none() {
        return Err("No file provided".into());
    }
    let input: String = fs::read_to_string(maybe_filename.unwrap())?;

    let button_a_cost = 3;
    let button_b_cost = 1;

    let machines = create_machines(input);

    println!("{:?}", machines);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    button_a: ButtonCost,
    button_b: ButtonCost,
    prize: Location
}

#[derive(Debug, Clone, Copy)]
struct ButtonCost {
    x_right: i64,
    y_forward: i64,
}

#[derive(Debug, Clone, Copy)]
struct Location {
    x: i64,
    y: i64
}

impl ClawMachine {
    fn new(a: (i64, i64), b: (i64, i64), p: (i64, i64)) -> ClawMachine {
        ClawMachine {
            button_a: ButtonCost {
                x_right: a.0,
                y_forward: a.1
            },
            button_b: ButtonCost {
                x_right: b.0,
                y_forward: b.1
            },
            prize: Location {
                x: p.0,
                y: p.1
            }
        }
    }
}

fn create_machines(raw: String) -> Vec<ClawMachine> {
    let tuples: Vec<(i64, i64)> = raw
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if line.starts_with("Button A") || line.starts_with("Button B") {
                let data: Vec<&str> = line.split("+").collect();
                let x: i64 = data[1].split(",").collect::<Vec<&str>>()[0].parse().unwrap();
                let y: i64 = data[2].parse().unwrap();
                (x, y)
            } else {
                let data: Vec<&str> = line.split("=").collect();
                let x: i64 = data[1].split(",").collect::<Vec<&str>>()[0].parse().unwrap();
                let y: i64 = data[2].parse().unwrap();
                (x, y)
            }
        })
        .collect();

    let mut machines = Vec::new();
    let mut iter = tuples.iter();
    loop {
        let a = iter.next();
        if a.is_none() {
            break;
        }
        let a = a.unwrap();
        let b = iter.next().unwrap();
        let p = iter.next().unwrap();
        machines.push(ClawMachine::new(a.clone(), b.clone(), p.clone()))
    }

    machines
}


fn get_filename_from_args() -> Option<String> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.is_empty() {
        return None;
    }
    let mut arguments = arguments.iter();
    arguments.next(); // skip the name of the program being ran
    arguments.next().cloned()
}

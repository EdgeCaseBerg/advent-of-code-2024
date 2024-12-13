use std::fs;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    // let maybe_filename = get_filename_from_args();
    let maybe_filename = Some(String::from("../input.txt"));
    if maybe_filename.is_none() {
        return Err("No file provided".into());
    }
    let input: String = fs::read_to_string(maybe_filename.unwrap())?;

    let machines = create_machines(input);
    let (won, cost) = part1(machines.clone());
    println!("Part 1 Prizes won: {}, Total cost: {}", won, cost);
    let corrected_machines = machines.into_iter().map(|m| m.correct_conversion()).collect();
    let (won, cost) = part2(corrected_machines);
    println!("Part 2 Prizes won: {}, Total cost: {}", won, cost);

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
    fn correct_conversion(&self) -> ClawMachine {
        ClawMachine {
            button_a: self.button_a.clone(),
            button_b: self.button_b.clone(),
            prize: Location {
                x: self.prize.x + 10_000_000_000_000,
                y: self.prize.y + 10_000_000_000_000,
            }
        }
    }
}

fn part1(machines: Vec<ClawMachine>) -> (i64, i64) {
    let mut total_cost = 0;
    let mut prizes_won = 0;

    for machine in machines {
        if let Some((a_presses, b_presses)) = find_solution_p1(&machine) {
            total_cost += 3 * a_presses + b_presses;
            prizes_won += 1;
        }
    }

    (prizes_won, total_cost)
}

fn part2(machines: Vec<ClawMachine>) -> (i64, i64) {
    let mut total_cost = 0;
    let mut prizes_won = 0;

    for machine in machines {
        if let Some((a_presses, b_presses)) = get_button_presses_if_winable(&machine) {
            total_cost += 3 * a_presses + b_presses;
            prizes_won += 1;
        }
    }

    (prizes_won, total_cost)
}

fn find_solution_p1(machine: &ClawMachine) -> Option<(i64, i64)> {
    let (gcd_x, _, _) = extended_gcd(machine.button_a.x_right, machine.button_b.x_right);
    let (gcd_y, _, _) = extended_gcd(machine.button_a.y_forward, machine.button_b.y_forward);

    if machine.prize.x % gcd_x != 0 || machine.prize.y % gcd_y != 0 {
        return None; // No solution exists
    }

    let mut best_cost = i64::MAX;
    let mut best_solution = None;
    let button_a_cost = 3;
    let button_b_cost = 1;

    for a in 0..=100 {
        for b in 0..=100 {
            if machine.button_a.x_right * a + machine.button_b.x_right * b == machine.prize.x && 
               machine.button_a.y_forward * a + machine.button_b.y_forward * b == machine.prize.y 
            {
                let cost = button_a_cost * a + button_b_cost * b;
                if cost < best_cost {
                    best_cost = cost;
                    best_solution = Some((a, b));
                }
            }
        }
    }

    best_solution
}

fn get_button_presses_if_winable(machine: &ClawMachine) -> Option<(i64, i64)> {
    let a1 = machine.button_a.x_right;
    let b1 = machine.button_a.y_forward;
    let a2 = machine.button_b.x_right;
    let b2 = machine.button_b.y_forward;
    let c1 = machine.prize.x;
    let c2 = machine.prize.y;
    /* 
     * Let's represent this as 2 lines.
     * (Does not have a solution in part 2 due to the +10000000000000 to p value)
     * x94 + y34 - 8400 = 0
     * x22 + y67 - 5400 = 0
     *
     * Does have a solution in part 2
     * x26 + y66 - 12748 = 0
     * x67 + y21 - 12716 = 0
     *
     * Or, in variable form...
     * a1 + b1 - c1 = 0
     * a2 + b2 - c2 = 0 (where c is negative I guess)
     */
    find_intersection(a1 as f64, b1 as f64, c1 as f64, a2 as f64, b2 as f64, c2 as f64)
}

fn find_intersection(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> Option<(i64, i64)> {
    let determinant = a1 * b2 - b1 * a2;
    // parallel? 
    if determinant == 0.0 {
        return None;
    }

    let x = (a2 * -c2 - b2 * -c1) / determinant;
    let y = (-c1 * b1 - -c2 * a1) / determinant;

    if x.fract() != 0.0 {
        return None;
    }

    if y.fract() != 0.0 {
        return None;
    }

    Some((x.abs() as i64, y.abs() as i64))
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

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = extended_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
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

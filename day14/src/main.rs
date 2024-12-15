
use std::fs;
use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;

pub mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let maybe_filename = util::get_filename_from_args();
    if maybe_filename.is_none() {
        // maybe_filename = Some(String::from("../input.txt"));
        return Err("No file provided".into());
    }
    let raw_data: String = fs::read_to_string(maybe_filename.unwrap())?;
    let robots = parse(&raw_data);
    part1(robots.clone());
    part2(robots.clone());
    
    Ok(())
}

// not 500? (wups wrong seconds)
// not 219549980
// not 208205613 (not a - 1 issue on midrow for )
// not 210912768 (didnt get height / width backwards in midrow)
// not 215868576 (not =100 in range or a misread on timing...)
// not 212407272 (not a problem with hashmap default)
fn part1(robots: Vec<Robot>) {
    // Note our origin of 0,0 is the top left, negative y is up.
    // let space_width_and_height = (11, 7);
    let space_width_and_height = (101, 103);
    let seconds = 100;
    let mut robots = robots.clone();
    
    for _ in 0..seconds {
        for robot in &mut robots {
            robot.step_in(space_width_and_height);
        }
    }

    let mut count_by_quad = HashMap::new();
    for robot in &robots {
        let key = robot.report_quadrant(space_width_and_height);

        count_by_quad.entry(key.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut safety_factor = 1;
    safety_factor *= count_by_quad.get(&Quadrant::TopLeft).unwrap_or(&1);
    safety_factor *= count_by_quad.get(&Quadrant::BottomLeft).unwrap_or(&1);
    safety_factor *= count_by_quad.get(&Quadrant::TopRight).unwrap_or(&1);
    safety_factor *= count_by_quad.get(&Quadrant::BottomRight).unwrap_or(&1);
    if safety_factor == 1 {
        safety_factor = 0; // All in the middle
    }

    println!("Safety Factor {:?}", safety_factor);
}

// 14759 too high
// 9918 too high (weird long bunch of crap)
// 29 too low (it was weird long straight line of crap)
fn part2(robots: Vec<Robot>) {
    let space_width_and_height = (101, 103);
    let seconds = 9918;
    let mut robots = robots.clone();
    let mut set = HashSet::new();
    for s in 0..seconds {
        for robot in &mut robots {
            robot.step_in(space_width_and_height);
            set.insert(robot.p);
        }
        if is_suspiciously_like_a_christmas_tree(&set, space_width_and_height) {
            println!("robots after {} seconds", s + 1);
            print_robots(&set, space_width_and_height);    
        }
        set.clear();
    }
}

fn is_suspiciously_like_a_christmas_tree(robots: &HashSet<(i32, i32)>, in_space: (i32,i32)) -> bool {
    let (w, h) = in_space;
    let sussy_size = w / 4;
    for x in 0..w {
        let mut in_same_row_near_each_other_count = 0;
        for y in 0..h {
            if robots.get(&(x,y)).is_some() {
                in_same_row_near_each_other_count = 0;
                for sus_factor in 0..=sussy_size {
                    if robots.get(&(x + sus_factor, y)).is_some() {
                        in_same_row_near_each_other_count += 1;
                    }
                }
            }
        }
        if in_same_row_near_each_other_count > sussy_size {
            return true;
        }
    }
    false
}

fn print_robots(robots: &HashSet<(i32, i32)>, in_space: (i32,i32)) {
    let (w, h) = in_space;
    for x in 0..w {
        for y in 0..h {
            if robots.contains(&(x,y)) {
                print!("1");
            } else {
                print!("0")
            }
        }
        println!();
    }
    println!();
}

#[derive(Clone, Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32)
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Middle
}

impl Robot {
    fn step_in(&mut self, in_space: (i32, i32)) {
        self.p.0 += self.v.0;
        self.p.0 = self.p.0.rem_euclid(in_space.0);

        self.p.1 += self.v.1;
        self.p.1 = self.p.1.rem_euclid(in_space.1);
    }

    fn report_quadrant(&self, in_space: (i32, i32)) -> Quadrant {
        let mid_x  = in_space.0 /2;
        let mid_y = in_space.1 /2;

        if mid_x == self.p.0 || mid_y == self.p.1 {
            return Quadrant::Middle;
        }

        let in_left = self.p.0 < mid_x;
        let in_top = self.p.1 < mid_y;

        match (in_left, in_top) {
            (true, true)   => Quadrant::TopLeft,
            (true, false)  => Quadrant::BottomLeft,
            (false, true)  => Quadrant::TopRight,
            (false, false) => Quadrant::BottomRight,
        }
    }
}

fn parse(raw_data: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    for line in raw_data.lines() {
        let data: Vec<&str> = line.split(" ").collect();
        let raw_p = data[0].split("=").collect::<Vec<&str>>();
        let p = tuple_from_after_equals(raw_p[1]);
        let raw_v = data[1].split("=").collect::<Vec<&str>>();
        let v = tuple_from_after_equals(raw_v[1]);
        robots.push(Robot { p, v });
    }
    robots
}

fn tuple_from_after_equals(after_equals: &str) -> (i32, i32) {
    let raw = after_equals.split(",").collect::<Vec<&str>>();
    let px: i32 = raw[0].parse().unwrap();
    let py: i32 = raw[1].parse().unwrap();
    (px, py)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_positively() {
        let space = (2,2);
        let mut m = Robot {
            p: (0, 0),
            v: (1, 1)
        };
        m.step_in(space);
        m.step_in(space);
        assert_eq!(m.p.0, 0);
        assert_eq!(m.p.1, 0);
    }

    #[test]
    fn wraps_negatively() {
        let space = (2,2);
        let mut m = Robot {
            p: (0, 0),
            v: (-1, -1)
        };
        m.step_in(space);
        m.step_in(space);
        assert_eq!(m.p.0, 0);
        assert_eq!(m.p.1, 0);
    }
}
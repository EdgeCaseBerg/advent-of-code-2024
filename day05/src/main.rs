use std::fs;
use std::cmp::Ordering;

fn main() {
    let raw_data = fs::read_to_string("../input.txt").expect("bad input data");
    let ordering = build_ordering(&raw_data);
    let reports = build_reports(&raw_data);

    let mut correctly_ordered = Vec::new();
    let mut incorrectly_ordered = Vec::new();
    for report in reports.iter() {
        if report.is_valid_according_to(&ordering) {
            correctly_ordered.push(report);
        } else {
            incorrectly_ordered.push(report);
        }
    }

    let mut sum_of_middles = 0;
    for report in correctly_ordered.iter() {
        sum_of_middles += report.middle_number();
    }
    println!("Correct: {:?}", sum_of_middles);

    let mut sum_of_middles = 0;
    for report in incorrectly_ordered.iter() {
        let fixed = report.re_order_according_to(&ordering);
        sum_of_middles += fixed.middle_number();
    }

    println!("Incorrect: {:?}", sum_of_middles);
}

#[derive(Debug)]
struct Report {
    data: Vec<i32>
}

impl Report {
    fn is_valid_according_to(&self, ordering: &[(i32, i32)]) -> bool {
        let mut valid = true;
        for idx in 0..self.data.len() {
            let report = self.data[idx];
            let ordering = ordering.iter().filter(|o| o.0 == report);
            let must_appear_after: Vec<i32> = ordering.map(|o| o.1).collect();
            for order in must_appear_after.iter() {
                match self.data.iter().position(|&r| r == *order) {
                    None => {}, // If the right side is not in the report then ignore it.
                    Some(location) => {
                        valid = valid && idx < location;
                    }
                }
            }
        }
        valid
    }

    fn middle_number(&self) -> i32 {
        let mid = self.data.len() / 2;
        self.data[mid]
    }

    fn re_order_according_to(&self, ordering: &[(i32, i32)]) -> Report {
        let mut new_data = self.data.clone();
        new_data.sort_by(|a,b| {
            for (before, after) in ordering.iter() {
                if a == before && b == after {
                    return Ordering::Less
                } else if b == before && a == after {
                    return Ordering::Greater
                }
            }
            Ordering::Equal
        });
        Report { data: new_data.clone() }
    }
}

fn build_reports(raw_data: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();
    let mut past_rules = false;
    for line in raw_data.lines() {
        if !line.is_empty() && !past_rules {
            continue;
        }
        if line.is_empty() && !past_rules {
            past_rules = true;
            continue;
        }
        let report = line.split(",").map(|no| {
            let number: i32 = no.parse().expect("Non-digit in input?");
            number
        }).collect();
        reports.push(Report { data: report });
    }
    reports
}

fn build_ordering(raw_data: &str) -> Vec<(i32, i32)> {
    let ordering: Vec<(&str, &str)> = raw_data
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|raw| {
            let raw: Vec<&str> = raw.splitn(2, "|").collect();
            (raw[0], raw[1])
        })
        .collect();
    let ordering: Vec<(i32, i32)> = ordering.iter()
        .map(|(fst, snd)| {
            let fst: i32 = fst.parse().expect("Failed to parse first number");
            let snd: i32 = snd.parse().expect("Failed to parse second number");
            (fst, snd)
        }).collect();
    ordering
} 

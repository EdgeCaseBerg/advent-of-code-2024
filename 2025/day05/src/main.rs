use std::fs;

fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    p1(raw_data);
    p2(raw_data);
}

fn p1(raw_data: &str) {
    let rules: Vec<(usize, usize)> = raw_data.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut raw_range = line.split("-");
            let low: usize = raw_range.next().expect("couldnt read low").parse().expect("couldnt convert low");
            let high: usize = raw_range.next().expect("couldnt read high").parse().expect("couldnt convert high");
            (low, high)
        }).collect();
    let skip_length = raw_data.lines().take_while(|line| !line.is_empty()).count() + 1 ;
    let ids: Vec<usize> = raw_data.lines().skip(skip_length).map(|line| {
        let id: usize = line.parse().expect("Id could not become a number");
        id
    }).collect();
    
    let mut fresh_count = 0;
    for id in ids {
        let mut fresh = false;
        for &(low, high) in &rules {
            fresh = fresh || low <= id && id <= high;
        }
        if fresh {
            fresh_count += 1;
        }
    }
    println!("{:?}", fresh_count);
}

fn p2(raw_data: &str) {
    let rules: Vec<Range> = raw_data.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut raw_range = line.split("-");
            let low: usize = raw_range.next().expect("couldnt read low").parse().expect("couldnt convert low");
            let high: usize = raw_range.next().expect("couldnt read high").parse().expect("couldnt convert high");
            Range { low, high }
        }).collect();

    let mut ranges: Vec<Range> = vec![];
    ranges.push(Range { low: rules[0].low, high: rules[0].high });

    let mut the_rules = rules.clone();
    while let Some(rule) = the_rules.pop() {
        for existing_range in ranges.clone() {
            if existing_range.contains_whole_range(&rule) {
                continue;
            }

            if existing_range.contains_low(&rule) {
                println!("contains_low {:?} {:?}", existing_range, rule);
                println!("before\n{:?}", ranges);
                ranges.retain(|r| r.not_equals(&existing_range));

                ranges.push(existing_range.expand_higher(&rule));
                println!("after\n{:?}\n", ranges);
                continue;
            }

            if existing_range.contains_high(&rule) {
                println!("contains_high {:?} {:?}", existing_range, rule);
                ranges.retain(|r| r.not_equals(&existing_range));
                ranges.push(existing_range.expand_lower(&rule));
                continue;
            }

            // It's new
            ranges.push(rule.clone());
        }
        println!("{:?}", ranges);
    }
    ranges.sort();
    ranges.dedup();
    
    let mut ids_in_ranges = 0;
    for range in ranges {
        ids_in_ranges += range.high - range.low + 1;
    }
    println!("{:?}", ids_in_ranges);
    
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, Eq, PartialOrd)]
struct Range {
    low: usize,
    high: usize
}

impl Range {
    fn contains_whole_range(&self, other: &Range) -> bool {
        self.contains_low(other) && self.contains_high(other)
    }

    fn contains_low(&self, other: &Range) -> bool {
        self.low <= other.low && other.low <= self.high
    }
    fn contains_high(&self, other: &Range) -> bool {
        self.low <= other.high && other.high <= self.high
    }

    fn expand_higher(&self, other: &Range) -> Range {
        Range { low: self.low, high: other.high }
    }

    fn expand_lower(&self, other: &Range) -> Range {
        Range { low: other.low, high: self.high }
    }

    fn not_equals(&self, other: &Range) -> bool {
        !(self.low == other.low && self.high == other.high)
    }
}
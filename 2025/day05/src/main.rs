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

    // This _would_ work if we had infinite memory and time.
    // let mut in_range = HashSet::new();
    // for rule in rules {
    //     for i in rule.low..=rule.high {
    //         in_range.insert(i);
    //     }
    // }
    // println!("{:?}", in_range.len());

    let mut ranges = rules.clone();
    ranges.sort();
    let mut the_rules = rules.clone();
    the_rules.sort();
    let mut new_rules = Vec::<Range>::new();

    // For each rule
    //   expand it to include any ranges in the other rules
    // deduplicate ranges that should now be the same
    for rule in &the_rules {
        let mut new_rule = *rule;
        for _ in &ranges {
            new_rule = new_rule.expand(&ranges);
        }
        new_rules.push(new_rule);
    }
    new_rules.sort();
    new_rules.dedup();
    
    let mut ids_in_ranges = 0;
    for range in new_rules {
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

    fn expand(&self, others: &Vec<Range>) -> Range {
        let mut expanded_self = *self;
        for range in others {
            if self.contains_whole_range(range) {
                // self.
                continue;
            }
            if range.contains_whole_range(self) {
                expanded_self = *range;
            }
            if self.contains_low(range) {
                expanded_self = self.expand_higher(range);
            }
            if self.contains_high(range) {
                expanded_self = self.expand_lower(range);
            }
        }
        expanded_self
    }
}
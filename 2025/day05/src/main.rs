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

fn p2(_raw_data: &str) {

}
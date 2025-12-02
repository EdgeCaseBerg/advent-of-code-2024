use std::fs;

fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    let mut result = 0;
    for item in raw_data.trim().split(",") {
        // get the numbers, the problem says no leading 0s but 
        // just in case, be ready for a string if we need it I guess.
        let mut iter = item.split("-");
        let first_str = iter.next().unwrap();
        let first_num: usize = first_str.parse().expect("bad number 1");

        let last_str = iter.next().unwrap();
        let last_num: usize = last_str.parse().expect("bad number 2");

        for id in first_num..=last_num {
            if is_invalid_p2(&id.to_string()) {
                result += id
            }
        }
    }
    println!("{:?}", result);
}

fn is_invalid_p2(id: &str) -> bool {
    for window_size in 1..id.len() {

        let chars: Vec<char> = id.to_string().chars().collect();
        let mut chunk_iter = chars.chunks(window_size);
        let mut seen = 0;
        let mut pattern = chunk_iter.next();
        let mut right = chunk_iter.next();
        while right.is_some() {
            if pattern.unwrap().len() != right.unwrap().len() {
                right = None;
                seen = 0;
                continue;
            }
            if pattern.unwrap() != right.unwrap() {
                seen = 0;
                right = None;    
                continue;
            } else {
                seen += 1
            }
            right = chunk_iter.next();
        }
        // we saw 1010, but not 101010
        // println!("{:?} {:?} {:?}", window_size, seen, id);
        if seen > 0 {
            return true
        }
    }
    return false
}


fn is_invalid_p1(id: &str) -> bool {
    if (id.len() % 2 == 1) {
        return false
    }
    let window_size = id.len()/2;
    let chars: Vec<char> = id.to_string().chars().collect();
    let mut chunk_iter = chars.chunks(window_size);
    let mut seen = 0;
    let mut left = chunk_iter.next();
    let mut right = chunk_iter.next();
    if right.is_none() {
        return false;
    }
        if left.unwrap()[0] == '0' || right.unwrap()[0] == '0' || left.unwrap().len() != right.unwrap().len() {
            return false
        }
        if left.unwrap() == right.unwrap() {
            seen += 1
        }

    // we saw 1010, but not 101010
    if seen == 1 {
        return true
    }
    return false
}
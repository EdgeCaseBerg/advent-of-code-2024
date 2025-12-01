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
    let (keys, locks) = parse_data(data);

    let mut non_overlapping_pairs = vec![];
    for key in &keys {
        for lock in &locks {
            let mut has_overlap = false;
            for i in 0..5 {
                let sum = key[i] + lock[i];
                if sum > 5 {
                    has_overlap = true;
                }
            }
            if !has_overlap {
                non_overlapping_pairs.push((key, lock));
            }
        }
    }

    non_overlapping_pairs.dedup();
    println!("Number of non_overlapping_pairs: {:?}", non_overlapping_pairs.len());
}
fn part_2(data: &str) {
    let (keys, locks) = parse_data(data);
    let _foo = keys.len() == locks.len();
    // It'd be fun to make a little animation of the locks and keys trying each other out.
}

fn parse_data(data: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut keys = vec![];
    let mut locks = vec![];
    let mut accum = vec![];
    for line in data.lines() {
        match line.chars().nth(0) {
            None => continue,
            Some(' ') => continue,
            _ => {}
        }

        accum.push(line);
        // Wait until full schema is ready
        if accum.len() != 7 {
            continue;
        }

        let first_row = accum[0];
        let is_lock = first_row.chars().all(|c| c == '#');
        let mut schema = vec![];
        for col in 0..5 {
            let mut count = 0;
            for row in 0..7 {
                let c: char = accum[row].chars().nth(col).unwrap();
                if c == '#' {
                    count += 1;
                }
            }
            count -= 1; // Ignore the schema marker
            schema.push(count);
        }
        if is_lock {
            locks.push(schema);    
        } else {
            keys.push(schema);
        }
        

        accum.clear();
    }
    (keys, locks)
}
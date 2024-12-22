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
    let initial_buyer_numbers = parse_input_file(data);
    let mut sum_of_2000th_numbers = 0;
    for initial_buyer_number in &initial_buyer_numbers {
        sum_of_2000th_numbers += compute_secret_number(*initial_buyer_number, 2000);
        println!("Buyer initial: {:?} 2000: {:?}", initial_buyer_number, compute_secret_number(*initial_buyer_number, 2000));
    }
    println!("Part 1 {:?}", sum_of_2000th_numbers);
}

fn part_2(data: &str) {
    let _foo = data;
    println!("Part 2 {:?}", 0);
}

fn parse_input_file(data: &str) -> Vec<u64> {
    data.lines().map(|line| {
        let u: u64 = line.parse().unwrap();
        u
    }).collect()
}

fn compute_secret_number(initial_secret: u64, iterations: u64) -> u64 {
    let mut secret = initial_secret;
    for _ in 0..iterations {
        secret = prune(mix(secret, secret * 64));
        secret = prune(mix(secret, secret / 32));
        secret = prune(mix(secret, secret * 2048));
    }
    secret
}

fn mix(secret: u64, other: u64) -> u64 {
    other ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}
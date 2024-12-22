pub mod boilerplate;

use std::collections::HashMap;

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
    let initial_buyer_numbers = parse_input_file(data);
    let mut buyer_to_prices = HashMap::new();
    for initial_buyer_number in &initial_buyer_numbers {
        let prices = compute_prices(*initial_buyer_number, 10);
        buyer_to_prices.insert(initial_buyer_number, prices);
    }
    
    for initial_buyer_number in &initial_buyer_numbers {
        let prices = buyer_to_prices.get(&initial_buyer_number).unwrap();
        let mut two_at_a_time = prices.windows(2);
        let mut differences = vec![];
        while let Some(window) = two_at_a_time.next() {
            differences.push(window[1] - window[0]);
        }
        println!("{:?} \n{:?}\n{:?}", initial_buyer_number, prices, differences);
    }
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

fn compute_prices(initial_secret: u64, iterations: u64) -> Vec<i64> {
    // Feels like there ought to be a nicer way of doing this, maybe I should just us i64's everywhere. anyway.
    let mut buyers_selling_prices = Vec::from([((initial_secret % 10) as i64).try_into().unwrap()]);
    let mut secret = initial_secret;
    for _ in 0..iterations {
        secret = prune(mix(secret, secret * 64));
        secret = prune(mix(secret, secret / 32));
        secret = prune(mix(secret, secret * 2048));
        let price: i64 = ((secret % 10) as i64).try_into().unwrap();
        buyers_selling_prices.push(price);
    }
    buyers_selling_prices
}

fn mix(secret: u64, other: u64) -> u64 {
    other ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}
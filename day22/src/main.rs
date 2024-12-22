pub mod boilerplate;

use std::collections::{ HashMap, HashSet };

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
        let prices = compute_prices(*initial_buyer_number, 2000);
        buyer_to_prices.insert(initial_buyer_number, prices);
    }
    
    let mut buyer_diff_to_price = HashMap::new();
    let mut buyer_diffs = vec![];
    let mut sequence_to_check = HashSet::new();

    for initial_buyer_number in &initial_buyer_numbers {
        /* The monkey will sell once it has seen the FULL four difference sequence we provide
         * it. It will not sell if it doesn't see the whole sequence. The number of bananas we 
         * get is the price at the 4th sequence change.
         */
        let prices = buyer_to_prices.get(&initial_buyer_number).unwrap();
        let mut two_at_a_time = prices.windows(2);
        let mut differences = vec![];
        while let Some(window) = two_at_a_time.next() {
            differences.push(window[1] - window[0]);
        }

        let mut seq_already_seen = HashSet::new();
        let mut four_at_a_time = differences.windows(4);
        let mut idx = 4; // This is not 3 because the price vector is off by one since there is no price for the starting point
        while let Some(window) = four_at_a_time.next() {
            let seq_as_str: Vec<char> = window.iter().map(|&diff| diff.to_string() + "").collect::<String>().chars().collect();

            // Accumulate every sequence across every buyer
            sequence_to_check.insert(seq_as_str.clone());

            // A monkey sells the first time it sees a sequence, so there's no reason to keep track of it later in the list
            if seq_already_seen.contains(&seq_as_str) {
                idx += 1;
                continue;
            }
            seq_already_seen.insert(seq_as_str.clone());

            buyer_diff_to_price.insert((initial_buyer_number, seq_as_str), prices[idx]);
            idx += 1;
        }
        let difference_as_big_string: String = differences.iter().map(|&diff| diff.to_string() + "").collect();
        buyer_diffs.push(difference_as_big_string);
    }

    /* We have a list of all possible sequences for all buyers now.
     * So, just check to see what the price of each is against our mapping table to look up the price
     */
    let mut best_price = 0;
    for seq in &sequence_to_check {
        let mut price_for_seq = 0;
        for initial_buyer_number in &initial_buyer_numbers {
            price_for_seq += match buyer_diff_to_price.get(&(initial_buyer_number, seq.to_vec())) {
                None => 0,
                Some(price) => *price
            };
        }
        if best_price < price_for_seq {
            best_price = price_for_seq;
        }
    }
    println!("Best {:?}", best_price);


    // It's not 36 :P so it's not just the maximum one digit place across all four monkeys.
    println!("Part 2 {:?}", buyer_diff_to_price.get(&(&1, vec!['-', '2', '1', '-', '1', '3'])));
    println!("Part 2 {:?}", buyer_diff_to_price.get(&(&2, vec!['-', '2', '1', '-', '1', '3'])));
    println!("Part 2 {:?}", buyer_diff_to_price.get(&(&3, vec!['-', '2', '1', '-', '1', '3'])));
    println!("Part 2 {:?}", buyer_diff_to_price.get(&(&2024, vec!['-', '2', '1', '-', '1', '3'])));
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
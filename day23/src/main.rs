pub mod boilerplate;

// use std::collections::{ HashMap, HashSet };

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
    let list_of_connections = parse_network_map_from(data);
    println!("{:?}", list_of_connections);
}

fn part_2(data: &str) {
    let _foo = data;
}

#[derive(Debug)]
struct Connection {
    from: String,
    to: String
}

fn parse_network_map_from(data: &str) -> Vec<Connection> {
    data.lines().map(|line| {
        let strings: Vec<String> = line.split("-").map(|s| s.to_string()).collect::<Vec<String>>();
        Connection {
            from: strings[0].clone(),
            to: strings[1].clone()
        }
    }).collect()
}
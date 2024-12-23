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
    let list_of_connections = parse_network_map_from(data);
    let mut connected_map = HashMap::new();
    for connection in &list_of_connections {
        let set = connected_map.entry(&connection.from).or_insert(HashSet::new());
        set.insert(connection.to.clone());
        let set = connected_map.entry(&connection.to).or_insert(HashSet::new());
        set.insert(connection.from.clone());
    }
    
    let mut unique_sets = HashSet::new();
    for (&pc1, p1_connected_to) in &connected_map {
        // We also need a set of three computers before we count.
        for pc2 in p1_connected_to {
            if pc2 == pc1 {
                continue;
            }
            let (is_set_of_3, node_contains_t) = match connected_map.get(pc2) {
                None => (false, false),
                Some(pc2_connections_to_pc3) => {
                    let shared_connections: HashSet<_> = p1_connected_to.intersection(pc2_connections_to_pc3).collect();
                    // println!("{:?}", (shared_connections.len() == 2, &pc1, &pc2, &shared_connections));
                    for pc3 in &shared_connections {
                        let mut vector = Vec::from([pc1, pc2, pc3]);
                        vector.sort();
                        unique_sets.insert(vector.clone());
                    }
                    (shared_connections.len() == 2, shared_connections.iter().any(|node| node.starts_with("t")))
                }
            };
        }
    }

    // println!("{:?}", connected_map);
    // println!("{:?}", unique_sets);
    let mut might_be_chief_historian_count = 0;
    for set in unique_sets {
        if set.iter().any(|computer| computer.starts_with("t")) {
            might_be_chief_historian_count += 1;
        }
    }
    println!("{:?}", might_be_chief_historian_count);
    
}

fn part_2(data: &str) {
    let _foo = data;
}

#[derive(Debug)]
struct Connection {
    from: String,
    to: String
}

impl Connection {
    fn is_connected_to(&self, other: Connection) -> bool {
        self.to == other.from || self.from == other.to
    }
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
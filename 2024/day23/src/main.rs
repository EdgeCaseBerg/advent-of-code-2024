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
    let list_of_connections = parse_network_map_from(data);
    let mut connected_map = HashMap::new();
    for connection in &list_of_connections {
        let set = connected_map.entry(&connection.from).or_insert(HashSet::new());
        set.insert(connection.to.clone());
        let set = connected_map.entry(&connection.to).or_insert(HashSet::new());
        set.insert(connection.from.clone());
    }

    // Observation. Every Set has the same size! Is this information useful in some way?
    
    // let mut largest = HashSet::new();
    let mut potential_passwords = HashSet::new();
    for (&pc1, p1_connected_to) in &connected_map {
        // Part 2 doesn't say this implicitly, but can I assume that t must be in the input?
        // NO! If you do that then you might miss out on some connections!
        // if !pc1.starts_with("t") {
        //     continue;
        // }

        let mut all_connected = HashSet::from([pc1]); // PC 1 is connected to itself trivialy
        all_connected.insert(p1_connected_to.iter().next().unwrap()); // Seed the first node we're connected to.

        // We need to find a CLIQUE!
        for p2 in p1_connected_to.iter().skip(1) {
            let p2_connected_to = connected_map.get(p2).unwrap(); // PC 2 is of course connected to P1
            // But are all the other nodes adjacent too??
            // A Clique is only a clique if everyone is best buds with everyone else
            let mut held_up_for_all = true;
            for connected in &all_connected {
                if p2_connected_to.contains(connected.clone()) {
                    // don't insert yet. it need to be adjacent to ALL of them!
                } else {
                    held_up_for_all = false;
                    break; // Stop trying to make fetch happen, you can't be friends with her.
                }
            }
            // If fetch happened, let the girl become mean.
            if held_up_for_all {
                all_connected.insert(p2);
            }
            
        }
        let mut password = Vec::new();
        for pc in all_connected {
            password.push(pc);
        }
        password.sort();
        potential_passwords.insert(password);
    }

    // "ta": {"ka", "co", "de", "kh"}
    // "co": {"ta", "ka", "de", "tc"},
    // "ka": {"ta", "co", "tb", "de"}
    // "de": {"ta", "cg", "co", "ka"}
    // "kh": {"ub", "ta", "tc", "qp"}

    let mut longest_password = Vec::new();
    for p in &potential_passwords {
        if p.len() > longest_password.len() {
            longest_password = p.to_vec();
        }
    }

    // I can't get join to work? the following trait bounds were not satisfied: `[&String]: Join<_>`
    let mut the_password = String::new();
    let mut not_first = false;
    for pc_name in longest_password.iter() {
        if not_first {
            the_password.push_str(",");
        }
        the_password.push_str(*pc_name);
        not_first = true;
    }
    println!("LAN Party password is {:?}", the_password);
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
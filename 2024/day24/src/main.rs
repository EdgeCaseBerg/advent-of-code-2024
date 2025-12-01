pub mod boilerplate;

use std::collections::HashMap;
use std::collections::VecDeque;

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

fn part_1(data: &str) -> i64 {
    let mut variables = parse_data_for_initial_variables(data);
    let mut gates_to_bind = parse_data_for_unbound_gates(data);
    let mut bound_gates = vec![];

    loop {
        if gates_to_bind.is_empty() {
            break;
        }

        let gate_to_bind = gates_to_bind.pop_front().unwrap();
        match gate_to_bind.bind_with(&variables, &gate_to_bind.output_name) {
            None => {
               // Not enough info yet!
               gates_to_bind.push_back(gate_to_bind);
            }, 
            Some(bound_gate) => {
                variables.insert(bound_gate.output_name.clone(), bound_gate.output());
                bound_gates.push(bound_gate);
            }
        }
    }

    // println!("{:?}", variables);
    // println!("{:?}", gates_to_bind);
    // println!("{:?}", bound_gates);

    bound_gates.sort_by_key(|gate| gate.output_name.clone());

    // println!("BG\n{:?}", bound_gates);

    let bits_from_least_to_most_significant: Vec<bool> = bound_gates.iter()
        .filter(|gate| gate.output_name.starts_with("z"))
        .map(|gate| {
            gate.output()
        })
        .collect();

    // 764 is too low
    // 2024 is too low
    // Oh I'm a dumb dumb and was submitting the sample data. Wups!
    println!("Part 1 {:?}", bools_to_decimal(&bits_from_least_to_most_significant));
    bools_to_decimal(&bits_from_least_to_most_significant)
}

fn run_gate_setup_with(variables: HashMap<String, bool>, gates_to_run: VecDeque<UnboundGate>) -> HashMap<String, bool> {
    let mut variables = variables.clone();
    let mut gates_to_bind = gates_to_run.clone();
    let mut bound_gates = vec![];

    loop {
        if gates_to_bind.is_empty() {
            break;
        }

        let gate_to_bind = gates_to_bind.pop_front().unwrap();
        match gate_to_bind.bind_with(&variables, &gate_to_bind.output_name) {
            None => {
               // Not enough info yet!
               gates_to_bind.push_back(gate_to_bind);
            }, 
            Some(bound_gate) => {
                variables.insert(bound_gate.output_name.clone(), bound_gate.output());
                bound_gates.push(bound_gate);
            }
        }
    }

    bound_gates.sort_by_key(|gate| gate.output_name.clone());

    bound_gates.iter()
        .map(|gate| {
            (gate.output_name.clone(), gate.output())
        })
        .collect()
}

fn part_2(data: &str) {
    let gates_to_bind = parse_data_for_unbound_gates(data);
    let mut suspicious_outputs = vec![];

    // Do we try to brute force this in some way?
    // That seems like a really terrible idea considering we have 44 bits from x 
    // and 44 from y and they have to go through these adder gates to output 44 bits of z.
    // It's probably easier to figure this out not with math but with human eyes and intuition.
    // So let's aid that by visualizing what this is doing.
    //
    //
    // [x0] -> XOR_1 \__z0 (SUM)
    // [y0] -> XOR_2 /
    // [x0] -> AND_1 \____ (CARRY) ___  XOR(2)
    // [y0] -> ANF_2 /
    //

    // let mut visual = vec![];
    
    // for gate in &gates_to_bind {
    //     visual.push(gate);
    // }
    // visual.sort_by_key(|gate| gate.output_name.clone());
    
    // for gate in &visual {
    //     println!("{:?}", gate);
    // }


    // All -> z outputs must be XOR unless its the last one.
    for gate in &gates_to_bind {
        if !gate.output_name.starts_with("z") || gate.output_name.starts_with("z45") {
            continue
        }
        if gate.gate_type != GateType::XOR {
            suspicious_outputs.push(gate.output_name.clone());
        }
    }

    // If gate is internal and not going to z, and also its not connected to the ipnuts
    // then its gotta be and or or, 
    for gate in gates_to_bind.iter() {
        if gate.output_name.starts_with("z") {
            continue
        }
        let mut connected_to_inputs = true;
        for input in &gate.variables {
            connected_to_inputs = connected_to_inputs && (input.starts_with("x") || input.starts_with("y"))
        }
        if connected_to_inputs {
            continue;
        }

        if gate.gate_type != GateType::XOR {
            continue;
        }

        suspicious_outputs.push(gate.output_name.clone());
    }

    // In a ripple adder, all AND gates must go to an OR gate. Any gates that fail that rule are sus:
    // besides x00 and y00
    for gate in &gates_to_bind {
        if gate.gate_type != GateType::AND {
            continue;
        }
        for gate2 in &gates_to_bind {
            if gate2.variables.contains(&gate.output_name) {
                if gate2.gate_type != GateType::OR && !(gate.variables.contains(&"x00".to_string()) && gate.variables.contains(&"y00".to_string())) {
                    println!("SUSPICIOUS AND to not OR Gate: {:?} {:?}", gate, gate2);
                    suspicious_outputs.push(gate.output_name.clone());

                }
            }
        }
    }

    // OR gates do the carry, so they feed into XOR and AND, but never directly into an OR gate
    for gate in &gates_to_bind {
        if gate.gate_type != GateType::OR {
            continue;
        }
        let mut connected_ands = 0;
        let mut connected_xors = 0;
        let mut weirdos = vec![];
        for gate2 in &gates_to_bind {
            if gate2.variables.contains(&gate.output_name) {
                if gate2.gate_type == GateType::OR {
                    println!("SUSPICIOUS OR Gate: {:?}", gate);
                    suspicious_outputs.push(gate.output_name.clone());
                } else {
                    // It should go to only 1 AND, and only 1 XOR
                    if gate2.gate_type == GateType::AND {
                        connected_ands += 1;
                        weirdos.push(gate2.clone());
                    }
                    if gate2.gate_type == GateType::XOR {
                        connected_xors += 1;
                        weirdos.push(gate2.clone());
                    }
                }
            }
        }
        if connected_ands > 1 || connected_xors > 1 {
            println!("SUSPICIOUS OR GATE {:?}, weirdos: {:?}", gate, weirdos);
            suspicious_outputs.push(gate.output_name.clone());
        }

        // An OR gate should also only be fed by AND gates
        for gate2 in &gates_to_bind {
            if gate.variables.contains(&gate2.output_name) {
                if gate2.gate_type != GateType::AND {
                    println!("SUSPICIOUS OR GATE HAS NOT ANDS FEEDING IT {:?}", gate2);
                    suspicious_outputs.push(gate2.output_name.clone());
                }
            }
        }
    }

    // x##,y## -> must connect to another XOR gate
    for gate in &gates_to_bind {
        if gate.gate_type != GateType::XOR {
            continue;
        }
        if !gate.variables.iter().all(|in_name| !(in_name.starts_with("x") || in_name.starts_with("y")) ) {
            continue;
        }
        for gate2 in &gates_to_bind {
            if gate2.variables.contains(&gate.output_name) {
                if gate2.gate_type != GateType::XOR {
                    println!("SUSPICIOUS AND to not XOR Gate: {:?}", gate);
                    suspicious_outputs.push(gate.output_name.clone());
                }
            }
        }
    }

    // NOT gmh,jmq,qrh,rqf,z06,z13,z38,z45
    // NOT gmh,jmq,nqp,qrh,rqf,z06,z13,z38
    // NOT gmh,jmq,qrh,rqf,z06,z13,z25,z38
    suspicious_outputs.sort();
    suspicious_outputs.dedup();
    println!("{:?}", suspicious_outputs.join(","));
}

fn parse_data_for_initial_variables(data: &str) -> HashMap<String, bool> {
    data.lines().take_while(|line| !line.is_empty()).map(|line| {
        let mut var_to_bool = line.split(": ");
        let name = var_to_bool.next().unwrap();
        let value = var_to_bool.next().unwrap().starts_with("1");
        (String::from(name), value)
    }).collect()
} 

fn parse_data_for_unbound_gates(data: &str) -> VecDeque<UnboundGate> {
    data.lines().skip_while(|line| !line.is_empty()).skip(1).map(|line| {
        let mut parts  = line.split(" ");
        let left_name  = parts.next().unwrap();
        let gate_type  = match parts.next().unwrap() {
            "XOR" => GateType::XOR,
            "OR" => GateType::OR,
            _ => GateType::AND,
        };
        let right_name = parts.next().unwrap();
        parts.next(); // skip ->
        let output_name = parts.next().unwrap();
        UnboundGate {
            variables: vec![left_name.to_string(), right_name.to_string()],
            output_name: output_name.to_string(),
            gate_type
        }
    }).collect()
}

fn bools_to_decimal(least_to_most: &Vec<bool>) -> i64 {
    let mut value = 0;
    for (i, &bit) in least_to_most.iter().enumerate() {
        if bit {
            value |= 1 << i;  // Set the i-th bit if bit is true
        }
    }
    value
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GateType {
    AND,
    XOR,
    OR
}

#[derive(Debug, Clone)]
struct UnboundGate {
    gate_type: GateType,
    variables: Vec<String>,
    output_name: String,
}

impl UnboundGate {
    fn bind_with(&self, known_values: &HashMap<String, bool>, to: &String) -> Option<Gate> {
        let two_inputs_bound = !self.variables.is_empty() && self.variables.iter().all(|variable| known_values.contains_key(variable));
        if two_inputs_bound {
            let g = Gate {
                gate_type: self.gate_type,
                in1: *known_values.get(&self.variables[0]).unwrap(),
                in2: *known_values.get(&self.variables[1]).unwrap(),
                output_name: to.clone()
            };
            Some(g)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone)]
struct Gate {
    gate_type: GateType,
    in1: bool,
    in2: bool,
    output_name: String,
}

impl Gate {
    fn output(&self) -> bool {
        match self.gate_type {
            GateType::AND => self.in1 && self.in2,
            GateType::OR => self.in1 || self.in2,
            GateType::XOR => self.in1 ^ self.in2,
        }
    }
}


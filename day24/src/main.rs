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
    let mut variables = parse_data_for_initial_variables(data);
    let mut gates_to_bind = parse_data_for_unbound_gates(data);

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

    let mut visual = vec![];
    
    for gate in &gates_to_bind {
        visual.push(gate);
    }
    visual.sort_by_key(|gate| gate.output_name.clone());
    
    for gate in &visual {
        println!("{:?}", gate);
    }

    // In a ripple adder, all AND gates must go to an OR gate. Any gates that fail that rule are sus:
    // besides x00 and y00
    for gate in &gates_to_bind {
        if gate.gate_type != GateType::AND {
            continue;
        }
        for gate2 in &gates_to_bind {
            if gate2.variables.contains(&gate.output_name) {
                if gate2.gate_type != GateType::OR {
                    println!("SUSPICIOUS AND to not OR Gate: {:?}", gate);
                }
            }
        }
    }
    // We find one more sus gate this way with UnboundGate { gate_type: AND, variables: ["x25", "y25"], output_name: "rqf" }

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
                }
            }
        }
    }

    // If output is to a z then it has to come from an adder, so it should be an XOR
    // (except for z45! since its just overflow )
    // So... do we have any gates that output to Z that are not an XOR type?
    let mut suspicious_gates = vec![];
    for gate in gates_to_bind.iter() {
        if gate.output_name.starts_with("z") && gate.gate_type != GateType::XOR && !gate.output_name.starts_with("z45") {
            suspicious_gates.push(gate);
            println!("Suspicious output to Z: {:?}", gate);
        }
    }
    // We get 3 gates so far.
    /*
    Suspicious output to Z: UnboundGate { gate_type: AND, variables: ["x06", "y06"], output_name: "z06" }
    Suspicious output to Z: UnboundGate { gate_type: OR, variables: ["gbd", "fjv"], output_name: "z13" }
    Suspicious output to Z: UnboundGate { gate_type: AND, variables: ["njc", "ngk"], output_name: "z38" }
    Suspicious output to Z: UnboundGate { gate_type: OR, variables: ["wvm", "dhs"], output_name: "z45" } <-- this is fine though because it's the very last bit!
    */
    // If out is NOT to z, then what can be suspicious now? 
    // https://www.101computing.net/binary-additions-using-logic-gates/
    // outputs within the circuit should go to either an AND gate or an OR gate
    // if they're not part of the initial x,y bits that is since those can go to XORs
    for gate in gates_to_bind.iter() {
        if !gate.output_name.starts_with("z") && 
            gate.variables.iter().all(|in_name| !(in_name.starts_with("x") || in_name.starts_with("y")) )
            && gate.gate_type == GateType::XOR
        {
            suspicious_gates.push(gate);
            println!("Suspicious internal gate not pointing to XOR: {:?}", gate);
        }
    }
    // We get 3 more gates.
    /*
        Suspicious internal gate not pointing to XOR: UnboundGate { gate_type: XOR, variables: ["cjt", "sfm"], output_name: "jmq" }
        Suspicious internal gate not pointing to XOR: UnboundGate { gate_type: XOR, variables: ["nmm", "kwb"], output_name: "gmh" }
        Suspicious internal gate not pointing to XOR: UnboundGate { gate_type: XOR, variables: ["ngk", "njc"], output_name: "qrh" }
    */
    // So that's 6 gates so far that seem sort of weird and likely should be swapped with each other to do some
    // correction. 
    // x06, y06 to z06 is just wrong, so where should it go?
    //  What about jmq? it goes to
    //   UnboundGate { gate_type: OR, variables: ["fmd", "jmq"], output_name: "qsf" }
    //   qsf goes to... UnboundGate { gate_type: XOR, variables: ["mms", "qsf"], output_name: "z07" }
    //   it probably _should_ be going to z06 though because we're carrying probably. So
    //   let's swap jmq and z06
    // What about gmh? it goes to
    //  UnboundGate { gate_type: AND, variables: ["fgr", "gmh"], output_name: "ckd" }
    //  and ckd goes to... UnboundGate { gate_type: OR, variables: ["ckd", "cqb"], output_name: "dmm" }
    //  and dmm goes to...  UnboundGate { gate_type: XOR, variables: ["hch", "dmm"], output_name: "z15" }
    //   which isn't quite right as it's not close enough to be screwy or at least, not yet.
    //  gmh also goes to
    //   UnboundGate { gate_type: XOR, variables: ["gmh", "fgr"], output_name: "z14" }
    //   Which is right next to the other weirdo, z13 so let's consider that our swap
    //
    // What about qrh? it goes to
    //  UnboundGate { gate_type: OR, variables: ["cpg", "qrh"], output_name: "wtp" }
    //  UnboundGate { gate_type: XOR, variables: ["mpv", "wtp"], output_name: "z39" }
    //  and that's close to z38! So let's consider swapping those
    let mut swaps = HashMap::from([
        (String::from("jmq"), String::from("z06")), (String::from("z06"), String::from("jmq")),
        (String::from("qrh"), String::from("z38")), (String::from("z38"), String::from("qrh")),
        (String::from("gmh"), String::from("z13")), (String::from("z13"), String::from("gmh"))
    ]);
    let mut fixed_gates_first_pass = vec![];
    for gate in &mut suspicious_gates {
        match swaps.get(&gate.output_name) {
            None => {},
            Some(to_fix) => {
                let mut fixed = gate.clone();
                fixed.output_name = to_fix.clone();
                swaps.remove(&gate.output_name); // only swap once!
                fixed_gates_first_pass.push(fixed.clone());
            }
        }
    }
    println!("{:?}",fixed_gates_first_pass );
    // Now lets splice those in and see what sort of offness we're getting.
    for fixed in fixed_gates_first_pass {
        loop {
            let gate = gates_to_bind.pop_front().unwrap();
            if gate.variables == fixed.variables && gate.gate_type == fixed.gate_type {
                gates_to_bind.push_back(fixed);
                break;
            } else {
                gates_to_bind.push_back(gate);
            }
        }
    }

    // Now compute the slightly more fixed adder
    let mut variables = parse_data_for_initial_variables(data);
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
    let almost_fixed: Vec<bool> = bound_gates.iter()
        .filter(|gate| gate.output_name.starts_with("z"))
        .map(|gate| {
            gate.output()
        })
        .collect();

    let original_answer = part_1(&data);
    let x_input = {
        let mut x_inputs: Vec<(String, bool)> = variables.keys().filter_map(|key| {
            if key.starts_with("x") {
                Some((key.clone(), *variables.get(key).unwrap()))
            } else {
                None
            }
        }).collect();
        x_inputs.sort_by_key(|tuple| tuple.0.clone());
        x_inputs.into_iter().map(|tuple| tuple.1).collect()
    };
    let y_input = {
        let mut y_inputs: Vec<(String, bool)> = variables.keys().filter_map(|key| {
            if key.starts_with("y") {
                Some((key.clone(), *variables.get(key).unwrap()))
            } else {
                None
            }
        }).collect();
        y_inputs.sort_by_key(|tuple| tuple.0.clone());
        y_inputs.into_iter().map(|tuple| tuple.1).collect()
    };
    let needs_a_swap_answer = bools_to_decimal(&x_input) + bools_to_decimal(&y_input);
    println!("TAR{:?}", original_answer);
    println!("OFF{:?}", needs_a_swap_answer);
    println!("XOR {:?}", original_answer ^  needs_a_swap_answer);

    let answer = vec!["jmq","z06","qrh","z38","gmh","z13",];
    let mut answer = vec!["rqf","z13","z45","jmq","gmh","qrh","z06","z38"];
    // gmh,jmq,qrh,rqf,z06,z13,z38,z45
    answer.sort();
    println!("{:?}", answer.join(","));
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


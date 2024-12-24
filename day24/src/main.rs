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

fn part_1(data: &str) {
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

    println!("{:?}", variables);
    println!("{:?}", gates_to_bind);
    println!("{:?}", bound_gates);

    bound_gates.sort_by_key(|gate| gate.output_name.clone());

    println!("BG\n{:?}", bound_gates);

    let bits_from_least_to_most_significant: Vec<bool> = bound_gates.iter()
        .filter(|gate| gate.output_name.starts_with("z"))
        .map(|gate| {
            // println!("{:?}", (&gate.output_name, gate.output()));
            gate.output()
        })
        .collect();

    // 764 is too low
    // 2024 is too low
    // Oh I'm a dumb dumb and was submitting the sample data. Wups!
    println!("{:?}", bools_to_decimal(&bits_from_least_to_most_significant));
}

fn part_2(data: &str) {
    let _foo = data;
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

#[derive(Debug, Clone, Copy)]
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


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
    let variables = parse_data_for_initial_variables(data);
    let gates_to_bind = parse_data_for_unbound_gates(data);

    println!("{:?}", variables);
    println!("{:?}", gates_to_bind);
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

fn parse_data_for_unbound_gates(data: &str) -> Vec<UnboundGate> {
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
    fn bind_with(&self, known_values: HashMap<String, bool>, to: String) -> Option<Gate> {
        let two_inputs_bound = !self.variables.is_empty() && self.variables.iter().all(|variable| known_values.contains_key(variable));
        if two_inputs_bound {
            let g = Gate {
                gate_type: self.gate_type,
                in1: *known_values.get(&self.variables[0]).unwrap(),
                in2: *known_values.get(&self.variables[1]).unwrap(),
                output_name: to
            };
            Some(g)
        } else {
            None
        }
    }
}

struct Gate {
    gate_type: GateType,
    in1: bool,
    in2: bool,
    output_name: String,
}

impl Gate {
    fn and(in1: bool, in2: bool, to: String) -> Self {
        Gate {
            gate_type: GateType::AND,
            in1,
            in2,
            output_name: to
        }
    }

    fn or(in1: bool, in2: bool, to: String) -> Self {
        Gate {
            gate_type: GateType::OR,
            in1,
            in2,
            output_name: to
        }
    }

    fn xor(in1: bool, in2: bool, to: String) -> Self {
        Gate {
            gate_type: GateType::XOR,
            in1,
            in2,
            output_name: to
        }
    }

    fn output(&self) -> bool {
        match self.gate_type {
            GateType::AND => self.in1 && self.in2,
            GateType::OR => self.in1 || self.in2,
            GateType::XOR => self.in1 ^ self.in2,
        }
    }
}


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
    let raw_designs = data
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(", ").collect::<Vec<&str>>())
        .flatten()
        .collect::<Vec<&str>>();
    let parsers = raw_designs.iter().map(|raw| {
        let design_tokens = raw.chars().filter_map(|c| TowelStripe::from(c)).collect::<Vec<TowelStripe>>();
        Design {
            design: design_tokens
        }
    }).collect::<Vec<Design>>();

    let request_designs = data
        .lines()
            .skip_while(|line| !line.is_empty()).skip(1)
            .map(|raw| {
                raw.chars().filter_map(|c| TowelStripe::from(c)).collect::<Vec<TowelStripe>>()
            })
            .collect::<Vec<Vec<TowelStripe>>>();
    
    let mut valid_designs = 0;
    for design in &request_designs {
        let mut leftover = design.clone();
        if try_parsers(&parsers, &leftover) {
            valid_designs += 1;
        }
    }
    println!("Valid designs {:?}", valid_designs);
}

fn try_parsers(parsers: &Vec<Design>, input: &[TowelStripe]) -> bool {
    let mut valid_parsers = vec![];
    for parser in parsers {
        if let Some(to_consume) = parser.matches(input) {
            // Fully matched?
            if to_consume == input.len() {
                return true;
            }

            // Deepen the search for this parse
            let eventually_worked = try_parsers(parsers, &input[to_consume..]);
            if eventually_worked {
                valid_parsers.push(parser);
            }
        }
    }
    return valid_parsers.len() > 0;
}


fn part_2(data: &str) {
    let _foo = data;
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum TowelStripe {
    White,
    Blue,
    Black,
    Red,
    Green
}
impl TowelStripe {
    fn from(c: char) -> Option<TowelStripe> {
        match c {
            'w' | 'u' | 'b' | 'r' | 'g' => Some(TowelStripe::of(c)),
            _ => None
        }
    }

    fn of(c: char) -> TowelStripe {
        match c {
            'w' => TowelStripe::White,
            'u' => TowelStripe::Blue,
            'b' => TowelStripe::Black,
            'r' => TowelStripe::Red,
            'g' => TowelStripe::Green,
            _ => panic!("Invalid character given for towel")
        }
    }

    fn is_value(&self, value: &char) -> bool {
        if let Some(stripe) = TowelStripe::from(*value) {
            *self == stripe
        } else {
            false
        }
    }    
}

#[derive(Debug)]
struct Design {
    design: Vec<TowelStripe>,
}

impl Design {
    fn matches(&self, against: &[TowelStripe]) -> Option<usize> {
        let num_tokens = against.len();
        let num_chars = self.design.len();
        let mut chars = self.design.clone().into_iter();
        let mut matching = Vec::new();

        if num_chars > against.len() {
            return None;
        }

        for idx in 0..num_chars.min(num_tokens) {
            match chars.next() {
                None => return None,
                Some(char_to_match) => {
                    if against[idx] == char_to_match {
                        matching.push(char_to_match.clone())
                    } else {
                        return None
                    }
                }
            }
        }
        return Some(matching.len());
    }
}


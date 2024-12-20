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
    let (parsers, request_designs) = get_parsed_data(data);

    let mut valid_designs = 0;
    for design in &request_designs {
        let leftover = design.clone();
        if try_parsers(&parsers, &leftover) {
            valid_designs += 1;
        }
    }
    println!("Valid designs {:?}", valid_designs);
}

fn part_2(data: &str) {
    let (parsers, request_designs) = get_parsed_data(data);

    let mut designs_to_find_arrangements_of = vec![];
    for design in &request_designs {
        if try_parsers(&parsers, design) {
            designs_to_find_arrangements_of.push(design);
        }
    }
    let mut trie = boilerplate::GenericTrie::new();
    for pattern in parsers {
        trie.insert(&pattern.design);
    }

    let mut different_ways_to_make_design = 0;
    for design in request_designs {
        different_ways_to_make_design += count_combos(&trie, &design);
    }

    println!("How many ways? {:?}", different_ways_to_make_design);
}


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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
}

#[derive(Debug, Clone)]
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

        for item in against.iter().take(num_chars.min(num_tokens)) {
            match chars.next() {
                None => return None,
                Some(char_to_match) => {
                    if *item == char_to_match {
                        matching.push(char_to_match.clone())
                    } else {
                        return None
                    }
                }
            }
        }
        Some(matching.len())
    }
}

fn get_parsed_data(data: &str) -> (Vec<Design>, Vec<Vec<TowelStripe>>) {
    let raw_designs = data
        .lines()
        .take_while(|line| !line.is_empty())
        .flat_map(|line| line.split(", ").collect::<Vec<&str>>())
        .collect::<Vec<&str>>();
    let parsers = raw_designs.iter().map(|raw| {
        let design_tokens = raw.chars().filter_map(TowelStripe::from).collect::<Vec<TowelStripe>>();
        Design {
            design: design_tokens
        }
    }).collect::<Vec<Design>>();

    let request_designs = data
        .lines()
        .skip_while(|line| !line.is_empty()).skip(1)
        .map(|raw| {
            raw.chars().filter_map(TowelStripe::from).collect::<Vec<TowelStripe>>()
        })
        .collect::<Vec<Vec<TowelStripe>>>();

    (parsers, request_designs)
}

fn try_parsers(parsers: &Vec<Design>, input: &[TowelStripe]) -> bool {
    for parser in parsers {
        if let Some(to_consume) = parser.matches(input) {
            // Fully matched?
            if to_consume == input.len() {
                return true;
            }

            // Deepen the search for this parse
            let eventually_worked = try_parsers(parsers, &input[to_consume..]);
            if eventually_worked {
                return true;
            }
        }
    }
    false
}


fn count_combos(prefix_trie: &boilerplate::GenericTrie<TowelStripe>, design: &[TowelStripe]) -> u64 {
    let len = design.len() + 1;
    let mut combos_for_index = vec![0u64; len];
    combos_for_index[0] = 1;
    for i in 0..design.len() {
        if combos_for_index[i] == 0 {
            continue;
        }

        let mut node = &prefix_trie.root;
        let mut j = i;
        while j < design.len() {
            if let Some(next) = node.children.get(&design[j]) {
                node = next;
                if node.ends_a_word {
                    combos_for_index[j + 1] += combos_for_index[i];
                }
                j += 1;
            } else {   
                break;
            }
        }
    }

    combos_for_index[design.len()]
}

